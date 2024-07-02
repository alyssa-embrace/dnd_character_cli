use ratatui::{
    buffer::Buffer, 
    style::Stylize, 
    layout::{Alignment, Rect}, 
    symbols::border, 
    widgets::{block::Title, Block, Widget}
};

use crate::app::context::Context;

pub struct CharacterEditorWidget<'a> {
    pub ref_context: &'a Context,
}

impl Widget for &CharacterEditorWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Character Editor ".bold());
        Block::bordered()
            .title(title.alignment(Alignment::Center))
            .border_set(border::THICK)
            .render(area, buf);
    }
}