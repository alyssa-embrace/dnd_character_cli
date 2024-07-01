use std::rc::Rc;

use ratatui::{layout::{Constraint, Direction, Layout, Rect}, Frame};

pub fn calc_character_wizard_layouts(frame: &mut Frame) -> Vec<Rc<[Rect]>> {
    let outer_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ]).split(frame.size());
    let left_inner_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ]).split(outer_layout[0]);
    let right_inner_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(100),
        ]).split(outer_layout[1]);

    vec![outer_layout, left_inner_layout, right_inner_layout]
}