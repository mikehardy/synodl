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
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    text::{Span, Spans},
    layout::{Alignment, Margin},
    Frame
};
use crate::{
    ui::util::{centered_rect_relative},
    App
};


pub fn show_error<B: Backend>(f: &mut Frame<B>, app: &App) {
    let area = centered_rect_relative(80, 50, f.size());
    let window = Block::default()
        .title("API error")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Red).fg(Color::White));

    f.render_widget(Clear, area);
    f.render_widget(window, area);

    let text = Spans::from(vec![
        Span::raw(&app.error)
    ]);

    let p = Paragraph::new(text)
        .wrap(Wrap { trim: true });

    let inner = area.inner(&Margin{vertical: 1, horizontal: 2});
    f.render_widget(p, inner);
}
