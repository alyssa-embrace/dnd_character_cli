use ratatui::{
    buffer::Buffer, 
    style::Stylize, 
    layout::{Alignment, Rect}, 
    symbols::border, 
    widgets::{block::Title, Block, Widget}
};

#[derive(Copy, Clone)]
pub struct CharacterEditorWidget;

impl Widget for &CharacterEditorWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Character Editor ".bold());
        Block::bordered()
            .title(title.alignment(Alignment::Center))
            .border_set(border::THICK)
            .render(area, buf);
    }
}