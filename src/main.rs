/*

SynoDL - CLI for Synology's DownloadStation
Copyright (C) 2015 - 2023  Stefan Ott

This program is free software: you can redistribute it and/or
modify it under the terms of the GNU General Public License as
published by the Free Software Foundation, version 3.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>.

*/

mod ui;
mod syno;

use std::{io, io::{Error, ErrorKind}, fs, error, path::Path, cmp::min, env};
use dirs::home_dir;
use getopts::Options;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
               LeaveAlternateScreen},
};
use subprocess::Exec;
use tui::{
    backend::{Backend, CrosstermBackend},
    widgets::TableState,
    Terminal
};
use serde::{Deserialize, Serialize};

use crate::{ui::ui, syno::api::{syno_list, syno_list_tasks, syno_resume_all, syno_login, syno_logout, syno_download, syno_delete,
Session}};

#[derive(Deserialize, Serialize)]
pub struct Task {
    id: String,
    title: String,
    status: String,
    size: u64,
    size_downloaded: u64,
    size_uploaded: u64,
    speed_download: u64,
    speed_upload: u64,
    percent_dn: f64,
    percent_up: f64
}

#[derive(PartialEq)]
enum Activity {
    Idle,
    Refresh,
    Quit,
    Submit
}

pub struct UI {
    ask_for_task_url: bool,
    confirm_delete: bool,
    delete_yes_selected: bool,
    show_details: bool,
    show_help: bool
}

pub struct App {
    state: TableState,
    tasks: Vec<Task>,
    input: String,
    error: String,
    activity: Activity,
    ui: UI
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    user: String,
    url: String,

    password: Option<String>,
    password_command: Option<String>,
    cacert: Option<String>
}

impl App {
    fn new() -> App {
        let ui = UI {
            ask_for_task_url: false,
            confirm_delete: false,
            delete_yes_selected: false,
            show_details: false,
            show_help: false,
        };
        App {
            activity: Activity::Refresh,
            state: TableState::default(),
            tasks: vec![],
            input: String::new(),
            error: String::new(),
            ui: ui
        }
    }

    fn next(&mut self) {
        if self.tasks.len() > 0 {
            let i = match self.state.selected() {
                Some(i) => {
                    if i >= self.tasks.len() - 1 {
                        0
                    } else {
                        i + 1
                    }
                }
                None => 0,
            };
            self.state.select(Some(i));
        }
    }

    fn previous(&mut self) {
        let len = self.tasks.len();
        if len > 0 {
            let i = match self.state.selected() {
                Some(i) => {
                    if i == 0 {
                        self.tasks.len() - 1
                    } else {
                        i - 1
                    }
                }
                None => len - 1,
            };
            self.state.select(Some(i));
        }
    }

    fn first(&mut self) {
        if self.tasks.len() > 0 {
            self.state.select(Some(0));
        }
    }

    fn last(&mut self) {
        let len = self.tasks.len();
        if len > 0 {
            self.state.select(Some(len - 1));
        }
    }

    fn next_page(&mut self, lines: usize) {
        let len = self.tasks.len();
        if len > 0 {
            let i = match self.state.selected() {
                Some(i) => min(i + lines, len - 1),
                None => 0
            };

            self.state.select(Some(i));
        };
    }

    fn previous_page(&mut self, lines: usize) {
        if self.tasks.len() > 0 {
            let i = match self.state.selected() {
                Some(i) => {
                    if lines > i {
                        0
                    } else {
                        i - lines
                    }
                },
                None => 0
            };

            self.state.select(Some(i));
        };
    }

    fn reload(&mut self, cfg: &Config, session: &Session) -> Result<(), Box<dyn error::Error>> {
        self.activity = Activity::Refresh;
        self.tasks = syno_list(cfg, session)?;
        self.activity = Activity::Idle;

        match self.state.selected() {
            Some(_i) => {},
            None => self.first()
        };

        Ok(())
    }

    fn start_download(&mut self, cfg: &Config, session: &Session) {
        match syno_download(cfg, session, &self.input) {
            Ok(()) => {
                self.input.clear();
                self.activity = Activity::Refresh;
            },
            Err(e) => {
                self.error = e.to_string();
            }
        }
        self.ui.ask_for_task_url = false;
    }

    fn delete(&mut self, cfg: &Config, session: &Session) {
        match self.state.selected() {
            Some(i) => {
                let t = &self.tasks.get(i);
                match t {
                    Some(task) =>  {
                        match syno_delete(cfg, session, task) {
                            Ok(()) => {
                                self.input.clear();
                                self.activity = Activity::Refresh;
                            },
                            Err(e) => {
                                self.error = e.to_string();
                            }
                        }
                    },
                    None => {
                        self.error = String::from("No task found");
                    }
                }
            },
            None => {}
        };
    }

    fn quit(&mut self, cfg: &Config, session: &Session)
            -> Result<(), Box<dyn error::Error>> {
        syno_logout(cfg, session)?;
        Ok(())
    }
}

fn validate_cacert(f: &str) -> Result<(), io::Error> {
    match f {
        "ignore" => Ok(()),
        _ => {
            if Path::new(f).exists() {
                Ok(())
            } else {
                Err(Error::new(ErrorKind::NotFound, "CA certificate not found"))
            }
        }
    }
}

fn validate_config(opt: Config) -> Result<Config, io::Error> {
    match &opt.cacert {
        Some(f) => {
            match validate_cacert(f) {
                Ok(()) => Ok(opt),
                Err(e) => Err(e)
            }
        }
        None => Ok(opt)
    }
}

fn load_config(file: &Path) -> Result<Config, Box<dyn error::Error>> {
    let file_content = fs::read_to_string(file)
        .expect("Error reading file");
    let mut opt = serde_ini::from_str::<Config>(&file_content)
        .expect("Failed to load configuration");

    // run password command if set
    match &opt.password_command {
        None => {},
        Some(password_command) => {
            let output = {
                Exec::shell(password_command)
            }.capture()?.stdout_str();

            match output.lines().next() {
                Some(line) => opt.password = Some(String::from(line)),
                None => {}
            };
        }
    };

    match validate_config(opt) {
        Ok(cfg) => Ok(cfg),
        Err(e) => Err(Box::new(e))
    }
}

fn add_task(cfg: Config, session: Session, url: String) -> Result<(), Box<dyn error::Error>> {
    println!("Adding download task ...");
    syno_download(&cfg, &session, &url)?;

    println!("Disconnecting ...");
    syno_logout(&cfg, &session)
}


fn list_tasks(cfg: Config, session: Session) -> Result<(), Box<dyn error::Error>> {
    println!("Listing download tasks ...");
    syno_list_tasks(&cfg, &session)?;

    println!("Disconnecting ...");
    syno_logout(&cfg, &session)
}

fn resume_all_tasks(cfg: Config, session: Session) -> Result<(), Box<dyn error::Error>> {
    println!("Resuming all download tasks ...");
    syno_resume_all(&cfg, &session)?;

    println!("Disconnecting ...");
    syno_logout(&cfg, &session)
}

fn run_tui(cfg: Config, session: Session) -> Result<(), Box<dyn error::Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new();
    let res = run_app(&mut terminal, app, &cfg, &session);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }
    Ok(())
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} URL [options]\n
If URL is empty a list of current download tasks is shown,
otherwise the URL is added as a download task.", program);
    print!("{}", opts.usage(&brief));

    println!("\nThis is synodl {}.", env!("CARGO_PKG_VERSION"));
    println!("Report bugs at {}", env!("CARGO_PKG_HOMEPAGE"));
}

fn main() -> Result<(), Box<dyn error::Error>> {

    /* load command line arguments */
    let args: Vec<String> = env::args().collect();

    let program = args[0].clone();
    let mut opts = Options::new();

    opts.optflag("h", "help", "Print help");
    opts.optflag("l", "list", "List tasks");
    opts.optflag("r", "resume-all", "resume all tasks");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!("{}", f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return Ok(());
    }

    let add_url = match matches.free.len() {
        0 => None,
        _ => Some(matches.free[0].clone())
    };

    /* load configuration */
    let mut path = home_dir().expect("Cannot find your home directory");
    path.push(".synodl");

    if !path.exists() {
        println!("Configuration file not found, aborting...");
        return Ok({});
    }

    let cfg = load_config(path.as_path())?;

    /* start operation */
    println!("Connecting to {} ...", cfg.url);
    let session = match syno_login(&cfg) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Login failed");
            return Err(e)
        }
    };

    if matches.opt_present("l") {
        return list_tasks(cfg, session);
    }

    if matches.opt_present("r") {
        return resume_all_tasks(cfg, session);
    }

    match add_url {
        None => run_tui(cfg, session),
        Some(url) => add_task(cfg, session, url)
    }
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App,
                       cfg: &Config, session: &Session)
                       -> Result<(), Box<dyn error::Error>> {
    let mut lines = 0;

    loop {
        terminal.draw(|f| {
            ui(f, &mut app, cfg);
            lines = f.size().height - 2
        })?;

        match app.activity {
            Activity::Quit => {
                return app.quit(cfg, session)
            }
            Activity::Submit => {
                app.start_download(cfg, session);
                app.activity = Activity::Refresh;
                continue
            }
            Activity::Refresh => {
                match app.reload(cfg, session) {
                    Ok(_) => {},
                    Err(e) => {
                        app.error = e.to_string()
                    }
                }
                app.activity = Activity::Idle;
                continue
            }
            _ => {}
        };

        if let Event::Key(key) = event::read()? {
            if app.ui.show_help {
                app.ui.show_help = false
            } else if app.ui.show_details {
                app.ui.show_details = false
            } else if app.ui.ask_for_task_url {
                match key.code {
                    KeyCode::Enter => {
                        app.ui.ask_for_task_url = false;
                        app.activity = Activity::Submit;
                    },
                    KeyCode::Esc => {
                        app.ui.ask_for_task_url = false;
                        app.input.clear();
                    },
                    KeyCode::Backspace => { app.input.pop(); },
                    KeyCode::Char(c) => { app.input.push(c) },
                    _ => { }
                }
            } else if app.ui.confirm_delete {
                app.ui.delete_yes_selected = match key.code {
                    KeyCode::Left => !app.ui.delete_yes_selected,
                    KeyCode::Down => !app.ui.delete_yes_selected,
                    KeyCode::Char('h') => !app.ui.delete_yes_selected,
                    KeyCode::Char('H') => !app.ui.delete_yes_selected,
                    KeyCode::Char('l') => !app.ui.delete_yes_selected,
                    KeyCode::Char('L') => !app.ui.delete_yes_selected,
                    KeyCode::Enter => {
                        app.ui.confirm_delete = false;
                        if app.ui.delete_yes_selected {
                            app.delete(cfg, session);
                        }
                        false
                    },
                    KeyCode::Esc => {
                        app.ui.confirm_delete = false;
                        false
                    }
                    _ => app.ui.delete_yes_selected
                }
            } else {
                match key.code {
                    KeyCode::Down => app.next(),
                    KeyCode::Up => app.previous(),
                    KeyCode::Char('a') => app.ui.ask_for_task_url = true,
                    KeyCode::Char('A') => app.ui.ask_for_task_url = true,
                    KeyCode::Char('d') => app.ui.confirm_delete = true,
                    KeyCode::Char('D') => app.ui.confirm_delete = true,
                    KeyCode::Char('i') => app.ui.show_details = true,
                    KeyCode::Char('I') => app.ui.show_details = true,
                    KeyCode::Char('j') => app.next(),
                    KeyCode::Char('J') => app.next(),
                    KeyCode::Char('k') => app.previous(),
                    KeyCode::Char('K') => app.previous(),
                    KeyCode::Char('q') => app.activity = Activity::Quit,
                    KeyCode::Char('Q') => app.activity = Activity::Quit,
                    KeyCode::Char('r') => app.activity = Activity::Refresh,
                    KeyCode::Char('R') => app.activity = Activity::Refresh,
                    KeyCode::Char('?') => app.ui.show_help = true,
                    KeyCode::Home => app.first(),
                    KeyCode::End => app.last(),
                    KeyCode::PageDown => app.next_page(lines as usize),
                    KeyCode::PageUp => app.previous_page(lines as usize),
                    _ => {}
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_app(_howmany: usize) -> App {
        let mut app = App::new();
        app.tasks = vec![
            Task {
                id: String::from("uuid01"),
                title: String::from("test-task-1"),
                status: "finished".to_owned(),
                size: 1024000,
                size_downloaded: 1024000,
                size_uploaded: 1024000,
                speed_download: 0,
                speed_upload: 0,
                percent_dn: 1.00,
                percent_up: 1.00
            },
            Task {
                id: String::from("uuid01"),
                title: String::from("test-task-2"),
                status: "finished".to_owned(),
                size: 1024000,
                size_downloaded: 1024000,
                size_uploaded: 1024000,
                speed_download: 0,
                speed_upload: 0,
                percent_dn: 1.00,
                percent_up: 1.00
            },
            Task {
                id: String::from("uuid01"),
                title: String::from("test-task-3"),
                status: "finished".to_owned(),
                size: 1024000,
                size_downloaded: 1024000,
                size_uploaded: 1024000,
                speed_download: 0,
                speed_upload: 0,
                percent_dn: 1.00,
                percent_up: 1.00
            },
            Task {
                id: String::from("uuid01"),
                title: String::from("test-task-4"),
                status: "finished".to_owned(),
                size: 1024000,
                size_downloaded: 1024000,
                size_uploaded: 1024000,
                speed_download: 0,
                speed_upload: 0,
                percent_dn: 1.00,
                percent_up: 1.00
            },
            Task {
                id: String::from("uuid01"),
                title: String::from("test-task-5"),
                status: "finished".to_owned(),
                size: 1024000,
                size_downloaded: 1024000,
                size_uploaded: 1024000,
                speed_download: 0,
                speed_upload: 0,
                percent_dn: 1.00,
                percent_up: 1.00
            }
        ];
        app
    }

    #[test]
    fn select_next_item_with_no_item_selected() {
        let mut app = get_test_app(5);
        assert_eq!(app.state.selected(), None);

        app.next();
        assert_eq!(app.state.selected(), Some(0));
    }

    #[test]
    fn select_next_item_with_item_selected() {
        let mut app = get_test_app(5);
        app.state.select(Some(3));
        assert_eq!(app.state.selected(), Some(3));

        app.next();
        assert_eq!(app.state.selected(), Some(4));
    }

    #[test]
    fn select_next_item_wraps_around() {
        let mut app = get_test_app(5);
        app.state.select(Some(4));
        assert_eq!(app.state.selected(), Some(4));

        app.next();
        assert_eq!(app.state.selected(), Some(0));
    }

    #[test]
    fn select_next_item_when_no_items_present() {
        let mut app = App::new();
        assert_eq!(app.state.selected(), None);

        app.next();
        assert_eq!(app.state.selected(), None);
    }

    #[test]
    fn select_prev_item_with_no_item_selected() {
        let mut app = get_test_app(5);
        assert_eq!(app.state.selected(), None);

        app.previous();
        assert_eq!(app.state.selected(), Some(4));
    }

    #[test]
    fn select_prev_item_with_item_selected() {
        let mut app = get_test_app(5);
        app.state.select(Some(3));
        assert_eq!(app.state.selected(), Some(3));

        app.previous();
        assert_eq!(app.state.selected(), Some(2));
    }

    #[test]
    fn select_prev_item_wraps_around() {
        let mut app = get_test_app(5);
        app.state.select(Some(0));
        assert_eq!(app.state.selected(), Some(0));

        app.previous();
        assert_eq!(app.state.selected(), Some(4));
    }

    #[test]
    fn select_prev_item_when_no_items_present() {
        let mut app = App::new();
        assert_eq!(app.state.selected(), None);

        app.previous();
        assert_eq!(app.state.selected(), None);
    }

    #[test]
    fn select_first_item_with_no_item_selected() {
        let mut app = get_test_app(5);
        assert_eq!(app.state.selected(), None);

        app.first();
        assert_eq!(app.state.selected(), Some(0));
    }

    #[test]
    fn select_first_item_with_item_selected() {
        let mut app = get_test_app(5);
        app.state.select(Some(3));
        app.first();
        assert_eq!(app.state.selected(), Some(0));
    }

    #[test]
    fn select_first_item_when_no_items_present() {
        let mut app = App::new();
        assert_eq!(app.state.selected(), None);

        app.first();
        assert_eq!(app.state.selected(), None);
    }

    #[test]
    fn select_last_item_with_no_item_selected() {
        let mut app = get_test_app(5);
        assert_eq!(app.state.selected(), None);

        app.last();
        assert_eq!(app.state.selected(), Some(4));
    }

    #[test]
    fn select_last_item_with_item_selected() {
        let mut app = get_test_app(5);
        app.state.select(Some(3));
        app.last();
        assert_eq!(app.state.selected(), Some(4));
    }

    #[test]
    fn select_last_item_when_no_items_present() {
        let mut app = App::new();
        assert_eq!(app.state.selected(), None);

        app.last();
        assert_eq!(app.state.selected(), None);
    }

    #[test]
    fn select_next_page_with_no_item_selected() {
        let mut app = get_test_app(5);
        assert_eq!(app.state.selected(), None);

        app.next_page(4);
        assert_ne!(app.state.selected(), None);
    }

    #[test]
    fn select_next_page_with_item_selected() {
        let mut app = get_test_app(5);
        app.state.select(Some(1));
        app.next_page(3);
        assert_eq!(app.state.selected(), Some(4));

        app.state.select(Some(1));
        app.next_page(2);
        assert_eq!(app.state.selected(), Some(3));
    }

    #[test]
    fn select_next_page_when_already_on_last_page() {
        let mut app = get_test_app(5);
        app.state.select(Some(3));

        app.next_page(3);
        assert_eq!(app.state.selected(), Some(4));
    }

    #[test]
    fn select_next_page_when_already_on_last_item() {
        let mut app = get_test_app(5);
        app.state.select(Some(4));

        app.next_page(3);
        assert_eq!(app.state.selected(), Some(4));
    }

    #[test]
    fn select_previous_page_with_no_item_selected() {
        let mut app = get_test_app(5);
        assert_eq!(app.state.selected(), None);

        app.previous_page(4);
        assert_ne!(app.state.selected(), None);
    }

    #[test]
    fn select_previous_page_with_item_selected() {
        let mut app = get_test_app(5);
        app.state.select(Some(4));
        app.previous_page(3);
        assert_eq!(app.state.selected(), Some(1));

        app.state.select(Some(4));
        app.previous_page(2);
        assert_eq!(app.state.selected(), Some(2));
    }

    #[test]
    fn select_previous_page_when_already_on_first_page() {
        let mut app = get_test_app(5);
        app.state.select(Some(1));

        app.previous_page(3);
        assert_eq!(app.state.selected(), Some(0));
    }

    #[test]
    fn select_previous_page_when_already_on_first_item() {
        let mut app = get_test_app(5);
        app.state.select(Some(0));

        app.previous_page(3);
        assert_eq!(app.state.selected(), Some(0));
    }
}
