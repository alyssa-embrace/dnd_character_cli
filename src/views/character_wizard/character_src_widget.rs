use ratatui::{
    buffer::Buffer, 
    style::Stylize, 
    layout::{Alignment, Rect}, 
    symbols::border, 
    widgets::{block::Title, Block, Widget}
};

#[derive(Copy, Clone)]
pub struct CharacterSrcWidget;

impl Widget for &CharacterSrcWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Character File Sources ".bold());
        Block::bordered()
            .title(title.alignment(Alignment::Center))
            .border_set(border::THICK)
            .render(area, buf);
    }
}

impl CharacterSrcWidget {
    pub fn show_add_src_dialog(self) {
        
    }
}