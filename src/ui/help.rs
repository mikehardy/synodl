use tui::{
    backend::Backend,
    style::{Color, Style},
    widgets::{Block, Borders, Table, Clear, Paragraph, Wrap},
    layout::{Layout, Constraint, Alignment, Margin},
    text::{Spans, Span},
    Frame
};
use crate::ui::util::{centered_rect_abs, make_row};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const URL: &str = env!("CARGO_PKG_HOMEPAGE");

pub fn show_help<B: Backend>(f: &mut Frame<B>) {
    let items = [
        make_row("A", String::from("Add download task")),
        make_row("D", String::from("Delete selected task")),
        make_row("I", String::from("Show task details")),
        make_row("Q", String::from("Quit")),
        make_row("R", String::from("Refresh list"))
    ];

    let text = vec![
        Spans::from(Span::raw(format!("This is synodl {}", VERSION))),
        Spans::from(Span::raw(format!("{}", URL)))
    ];

    let area = centered_rect_abs(33, items.len() as u16 + 7, f.size());
    let window = Block::default()
        .title("Keyboard shortcuts")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Blue).fg(Color::White));

    f.render_widget(Clear, area);
    f.render_widget(window, area);

    let rects = Layout::default()
        .constraints([Constraint::Length(items.len() as u16 + 1),
                      Constraint::Length(2)].as_ref())
        .split(area.inner(&Margin{vertical: 2, horizontal: 3}));

    let help = Table::new(items)
        .widths(&[
            Constraint::Length(2),
            Constraint::Length(20)
        ]);

    let about = Paragraph::new(text)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    f.render_widget(help, rects[0]);
    f.render_widget(about, rects[1])
}
