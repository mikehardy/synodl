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
    widgets::{Block, Borders, Clear, Paragraph},
    layout::{Layout, Constraint, Alignment, Margin, Direction},
    Frame
};
use crate::{
    ui::util::{centered_rect_abs},
    App
};


fn highlight(b: bool) -> Style {
    match b {
        true => Style::default(),
        false => Style::default().fg(Color::Black).bg(Color::White)
    }
}

/* minimal size: 21x5 */
pub fn ask_delete<B: Backend>(f: &mut Frame<B>, app: &App) {
    let area = centered_rect_abs(21, 5, f.size());
    let window = Block::default()
        .title("Delete")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Red).fg(Color::White));

    f.render_widget(Clear, area);
    f.render_widget(window, area);

    let rects = Layout::default()
        .constraints([Constraint::Length(1),
                      Constraint::Length(1),
                      Constraint::Length(1)].as_ref())
        .split(area.inner(&Margin{vertical: 1, horizontal: 2}));

    let prompt = Block::default()
        .title("Delete this task?");
    f.render_widget(prompt, rects[0]);

    let selection = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(6),
                Constraint::Length(2),
                Constraint::Length(7)].as_ref())
        .split(rects[2].inner(&Margin{vertical: 0, horizontal: 1 }));

    let no = Paragraph::new("[ No ]")
        .style(highlight(app.ui.delete_yes_selected));
    let yes = Paragraph::new("[ Yes ]")
        .style(highlight(!app.ui.delete_yes_selected));

    f.render_widget(no, selection[0]);
    f.render_widget(yes, selection[2]);
}
