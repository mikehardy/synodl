use tui::{
    style::{Modifier, Style},
    widgets::{Cell, Row},
    layout::{Layout, Constraint, Direction, Rect}
};
use std::cmp::min;
use byte_unit::Byte;

// TODO: extract functions to get coordinates

// taken from examples/popup.rs
pub fn centered_rect_relative(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        ).split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ].as_ref(),
        )
        .split(popup_layout[1])[1]
}

pub fn centered_rect_abs(mut width: u16, mut height: u16, r: Rect) -> Rect {
    let w = r.width;
    let h = r.height;

    height = min(h, height);
    width = min(w, width);

    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
                Constraint::Length((h - height) / 2),
                Constraint::Length(height),
                Constraint::Length((h - height) / 2),
            ].as_ref(),
        ).split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
                Constraint::Length((w - width) / 2),
                Constraint::Length(width),
                Constraint::Length((w - width) / 2),
            ].as_ref(),
        ).split(popup_layout[1])[1]
}

pub fn make_row(label: &str, value: String) -> Row {
    Row::new([
        Cell::from(label).style(Style::default().add_modifier(Modifier::BOLD)),
        Cell::from(value)
    ])
}

pub fn size_text(n: u64) -> String {
    let byte = Byte::from_bytes(n as u128);
    byte.get_appropriate_unit(false).to_string()
}

pub fn speed_text(n: u64) -> String {
    size_text(n) + "/s"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn byte_text_bytes()
    {
        let result = size_text(500);
        assert_eq!(result, "500 B");
    }

    #[test]
    fn byte_text_kb()
    {
        let result = size_text(5300);
        assert_eq!(result, "5.30 KB");
    }

    #[test]
    fn byte_text_mb()
    {
        let result = size_text(50000000);
        assert_eq!(result, "50.00 MB");
    }

    #[test]
    fn speed_text_bytes()
    {
        let result = speed_text(500);
        assert_eq!(result, "500 B/s");
    }

    #[test]
    fn speed_text_kb()
    {
        let result = speed_text(5300);
        assert_eq!(result, "5.30 KB/s");
    }

    #[test]
    fn speed_text_mb()
    {
        let result = speed_text(50000000);
        assert_eq!(result, "50.00 MB/s");
    }
}
