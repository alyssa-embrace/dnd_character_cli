use std::sync::Arc;

use ratatui::{
    buffer::Buffer, 
    style::Stylize, 
    layout::{Alignment, Rect}, 
    symbols::border, 
    widgets::{block::Title, Block, Widget}
};

use crate::app::context::Context;

pub struct CharacterListWidget {
    pub arc_context: Arc<Context>,
}

impl Widget for &CharacterListWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Character List ".bold());
        Block::bordered()
            .title(title.alignment(Alignment::Center))
            .border_set(border::THICK)
            .render(area, buf);
    }
}