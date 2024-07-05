use std::rc::Rc;

use ratatui::{layout::{Constraint, Direction, Layout, Rect}, Frame};

pub fn calc_character_wizard_layouts(rect: &Rect) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ]).split(rect.clone())
}