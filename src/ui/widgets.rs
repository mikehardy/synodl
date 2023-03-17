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
