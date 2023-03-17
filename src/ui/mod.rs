mod addtask;
mod delete;
mod help;
mod taskdetails;
mod util;
mod widgets;

use tui::{
    backend::Backend,
    style::{Color, Style},
    widgets::{Block, Cell, Clear, Row, Table},
    layout::{Alignment, Layout, Constraint, Direction},
    Frame
};
use crate::{
    ui::{help::show_help, taskdetails::show_details, addtask::add_task,
         delete::ask_delete, util::{speed_text, size_text},
         widgets::show_error},
    App, Task, Config
};


struct Summary {
    speed_download: u64,
    speed_upload: u64
}

impl Summary {
    fn new(t: &Vec<Task>) -> Summary {
        Summary {
            speed_download: t.iter().fold(0, |acc, e| acc + e.speed_download),
            speed_upload: t.iter().fold(0, |acc, e| acc + e.speed_upload)
        }
    }
}

fn status_color(s: &str) -> Color {
    match s {
        "waiting" => Color::Yellow,
        "downloading" => Color::Cyan,
        "paused" => Color::Magenta,
        "finishing" => Color::Cyan,
        "finished" => Color::Green,
        "hash_checking" => Color::Cyan,
        "seeding" => Color::Blue,
        "filehosting_waiting" => Color::Yellow,
        "extracting" => Color::Cyan,
        _ => Color::Red
    }
}

fn window_too_small<B: Backend>(f: &mut Frame<B>) {
    let area = f.size();
    let window = Block::default()
        .title("Window too small")
        .title_alignment(Alignment::Center)
        .style(Style::default().fg(Color::Red));

    f.render_widget(Clear, area);
    f.render_widget(window, area);
}

fn show_tasks<B: Backend>(f: &mut Frame<B>, app: &mut App, cfg: &Config) {
    let size = f.size();
    let rects = Layout::default()
        .constraints([Constraint::Min(10), Constraint::Length(1)].as_ref())
        .split(size);

    let header_cells = ["Download task", "Size", "Status", "Prog"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default()));
    let header = Row::new(header_cells)
        .style(Style::default().bg(Color::White).fg(Color::Black));
    let rows = app.tasks.iter().map(|item| {
        let cells = [
            Cell::from(item.title.as_str()),
            Cell::from(size_text(item.size)),
            Cell::from(item.status.as_str())
                .style(Style::default().fg(status_color(&item.status))),
            Cell::from(format!("{0:.0}%", 100.0 * item.percent_dn))
        ];
        Row::new(cells)
    });

    let columns = [
        Constraint::Length(size.width - 30),
        Constraint::Length(10),
        Constraint::Length(11),
        Constraint::Length(4),
    ];

    let tasks = Table::new(rows)
        .header(header)
        .highlight_style(Style::default().bg(Color::Cyan).fg(Color::Black))
        .highlight_symbol(" ")
        .widths(&columns);

    let status_row = Block::default()
        .style(Style::default().bg(Color::Blue).fg(Color::White));

    let status_left = Block::default()
        .title(format!(" {}", status_text(&app, cfg)));

    let st = status_traffic(&app);
    let status_right = Block::default()
        .title(format!("{} ", &st));

    let parts = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length((cfg.url.len() + 2) as u16),
                      Constraint::Min(1),
                      Constraint::Length((st.len() + 1) as u16)].as_ref())
        .split(rects[1]);

    f.render_stateful_widget(tasks, rects[0], &mut app.state);
    f.render_widget(status_row, rects[1]);
    f.render_widget(status_left, parts[0]);
    f.render_widget(status_right, parts[2]);
}

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App, config: &Config) {
    let size = f.size();

    if size.width < 30 {
        window_too_small(f);
    } else {
        show_tasks(f, app, config);
    }

    if app.show_help {
        show_help(f);
    }

    if app.ask_delete {
        ask_delete(f, app);
    }

    if app.show_details {
        show_details(f, app);
    }

    if app.add_task {
        add_task(f, app);
    }

    if app.error.len() > 0 {
        show_error(f, app);
        app.error = String::from("");
    }
}

fn status_text(app: &App, cfg: &Config) -> String {
    if app.quitting {
        return String::from("Quitting ...");
    } else if app.loading {
        return String::from("Refreshing ...");
    } else {
        return String::from(&cfg.url);
    }
}

fn status_traffic(app: &App) -> String {
    let s = Summary::new(&app.tasks);

    format!("up: {}, down: {}.  Press '?' for help.",
        speed_text(s.speed_upload), speed_text(s.speed_download))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_no_traffic() {
        let mut app = App::new();
        app.loading = false;
        assert_eq!(status_traffic(&app),
            "up: 0 B/s, down: 0 B/s.  Press '?' for help.");
    }

    #[test]
    fn status_text_quitting() {
        let mut app = App::new();
        app.quitting = true;

        let cfg = Config {
            user: String::from(""),
            url: String::from("http://foo/"),
            password: None,
            password_command: None,
            cacert: None
        };

        assert_eq!(status_text(&app, &cfg), "Quitting ...");
    }

    #[test]
    fn status_line_loading() {
        let mut app = App::new();
        app.loading = true;

        let cfg = Config {
            user: String::from(""),
            url: String::from("http://foo/"),
            password: None,
            password_command: None,
            cacert: None
        };

        assert_eq!(status_text(&app, &cfg), "Refreshing ...");
    }

    #[test]
    fn status_line_normal() {
        let mut app = App::new();
        app.loading = false;

        let cfg = Config {
            user: String::from(""),
            url: String::from("http://foo/"),
            password: None,
            password_command: None,
            cacert: None
        };

        assert_eq!(status_text(&app, &cfg), "http://foo/");
    }

    #[test]
    fn sumary_single_finished_task() {
        let tasks = vec![
            Task {
                id: String::from("uuid01"),
                title: "Peppermint-7-20160616-amd64.iso".to_owned(),
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

        let s = Summary::new(&tasks);
        assert_eq!(s.speed_download , 0);
        assert_eq!(s.speed_upload , 0);
    }

    #[test]
    fn sumary_single_downlodaing_task() {
        let tasks = vec![
            Task {
                id: String::from("uuid01"),
                title: "Peppermint-7-20160616-amd64.iso".to_owned(),
                status: "finished".to_owned(),
                size: 1024000,
                size_downloaded: 1024000,
                size_uploaded: 1024000,
                speed_download: 1234,
                speed_upload: 0,
                percent_dn: 1.00,
                percent_up: 1.00
            }
        ];

        let s = Summary::new(&tasks);
        assert_eq!(s.speed_download , 1234);
        assert_eq!(s.speed_upload , 0);
    }

    #[test]
    fn sumary_single_uplodaing_task() {
        let tasks = vec![
            Task {
                id: String::from("uuid01"),
                title: "Peppermint-7-20160616-amd64.iso".to_owned(),
                status: "finished".to_owned(),
                size: 1024000,
                size_downloaded: 1024000,
                size_uploaded: 1024000,
                speed_download: 0,
                speed_upload: 2345,
                percent_dn: 1.00,
                percent_up: 1.00
            }
        ];

        let s = Summary::new(&tasks);
        assert_eq!(s.speed_download , 0);
        assert_eq!(s.speed_upload , 2345);
    }

    #[test]
    fn sumary_single_downloading_and_uplodaing_task() {
        let tasks = vec![
            Task {
                id: String::from("uuid01"),
                title: "Peppermint-7-20160616-amd64.iso".to_owned(),
                status: "finished".to_owned(),
                size: 1024000,
                size_downloaded: 1024000,
                size_uploaded: 1024000,
                speed_download: 1234,
                speed_upload: 2345,
                percent_dn: 1.00,
                percent_up: 1.00
            }
        ];

        let s = Summary::new(&tasks);
        assert_eq!(s.speed_download , 1234);
        assert_eq!(s.speed_upload , 2345);
    }

    #[test]
    fn sumary_no_tasks() {
        let tasks = vec![];

        let s = Summary::new(&tasks);
        assert_eq!(s.speed_download , 0);
        assert_eq!(s.speed_upload , 0);
    }

    #[test]
    fn sumary_multiple_tasks() {
        let tasks = vec![
            Task {
                id: String::from("uuid01"),
                title: "Peppermint-7-20160616-amd64.iso".to_owned(),
                status: "finished".to_owned(),
                size: 1024000,
                size_downloaded: 1024000,
                size_uploaded: 1024000,
                speed_download: 100,
                speed_upload: 200,
                percent_dn: 1.00,
                percent_up: 1.00
            },
            Task {
                id: String::from("uuid01"),
                title: "Peppermint-7-20160616-amd64.iso".to_owned(),
                status: "finished".to_owned(),
                size: 1024000,
                size_downloaded: 1024000,
                size_uploaded: 1024000,
                speed_download: 50,
                speed_upload: 0,
                percent_dn: 1.00,
                percent_up: 1.00
            },
            Task {
                id: String::from("uuid01"),
                title: "Peppermint-7-20160616-amd64.iso".to_owned(),
                status: "finished".to_owned(),
                size: 1024000,
                size_downloaded: 1024000,
                size_uploaded: 1024000,
                speed_download: 0,
                speed_upload: 25,
                percent_dn: 1.00,
                percent_up: 1.00
            },
            Task {
                id: String::from("uuid01"),
                title: "Peppermint-7-20160616-amd64.iso".to_owned(),
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

        let s = Summary::new(&tasks);
        assert_eq!(s.speed_download , 150);
        assert_eq!(s.speed_upload , 225);
    }

    #[test]
    fn test_status_colors() {
        assert_eq!(status_color("waiting"), Color::Yellow);
        assert_eq!(status_color("downloading"), Color::Cyan);
        assert_eq!(status_color("paused"), Color::Magenta);
        assert_eq!(status_color("finishing"), Color::Cyan);
        assert_eq!(status_color("finished"), Color::Green);
        assert_eq!(status_color("hash_checking"), Color::Cyan);
        assert_eq!(status_color("seeding"), Color::Blue);
        assert_eq!(status_color("filehosting_waiting"), Color::Yellow);
        assert_eq!(status_color("extracting"), Color::Cyan);
        assert_eq!(status_color("something else"), Color::Red);
    }
}
