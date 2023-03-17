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

use tui::{
    backend::Backend,
    style::{Color, Style},
    widgets::{Block, Borders, Table, Clear},
    layout::{Constraint, Alignment, Margin},
    Frame
};
use crate::{
    ui::util::{centered_rect_abs, make_row, speed_text, size_text},
    App, Task
};


fn make_table(task: &Task) -> Table {
    let downloaded = format!("{} ({1:.2})",
                            size_text(task.size_downloaded), task.percent_dn);
    let uploaded = format!("{} ({1:.2})",
                            size_text(task.size_uploaded), task.percent_up);

    let rows = [
        make_row("Status", String::from(&task.status)),
        make_row("Size", size_text(task.size)),
        make_row("Downloaded", downloaded),
        make_row("Uploaded", uploaded),
        make_row("Speed down", speed_text(task.speed_download)),
        make_row("Speed up", speed_text(task.speed_upload))
    ];

    Table::new(rows)
        .block(Block::default())
        .widths(&[
            Constraint::Length(12),
            Constraint::Length(20),
    ])
}

pub fn show_details<B: Backend>(f: &mut Frame<B>, app: &App) {
    let area = centered_rect_abs(36, 10, f.size());
    let window = Block::default()
        .title("Download task details")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Blue).fg(Color::White));

    f.render_widget(Clear, area);
    f.render_widget(window, area);

    match app.state.selected() {
        Some(i) => {
            let table = make_table(&app.tasks[i]);
            let m = Margin { vertical: 2, horizontal: 3 };
            f.render_widget(table, area.inner(&m));
        },
        None => { }
    };
}
