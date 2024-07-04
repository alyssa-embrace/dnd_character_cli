use ratatui::{
    buffer::Buffer, 
    style::Stylize, 
    layout::{Alignment, Rect}, 
    symbols::border, 
    widgets::{block::Title, Block, Widget}
};

pub struct CharacterSrcWidget {}

impl Widget for &CharacterSrcWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Character File Sources ".bold());
        Block::bordered()
            .title(title.alignment(Alignment::Center))
            .border_set(border::THICK)
            .render(area, buf);
    }
}