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


pub fn add_task<B: Backend>(f: &mut Frame<B>, app: &App) {
    let area = centered_rect_relative(80, 50, f.size());
    let window = Block::default()
        .title("Add download task")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Blue).fg(Color::White));

    f.render_widget(Clear, area);
    f.render_widget(window, area);

    let prompt = Block::default()
        .title("Enter URL:");
    let inner = area.inner(&Margin{vertical: 1, horizontal: 2});
    f.render_widget(prompt, inner);

    let text = Spans::from(vec![
        Span::raw(&app.input),
        Span::styled("_", Style::default().bg(Color::White))
    ]);

    let textarea = Paragraph::new(text)
        .style(Style::default().bg(Color::Cyan).fg(Color::Black))
        .wrap(Wrap { trim: true });
    let area = inner.inner(&Margin{vertical: 1, horizontal: 0});
    f.render_widget(textarea, area);
}
