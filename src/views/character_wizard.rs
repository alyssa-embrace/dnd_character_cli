use ratatui::{
    layout::Alignment, prelude::{Buffer, Rect}, style::Stylize, symbols::border, widgets::{
        block::Title, Block, Widget
    }
};

pub struct CharacterWizard {
    pub src_directories: Vec<String>,
}

impl Widget for &CharacterWizard {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Character Wizard ".bold());
        Block::bordered()
            .title(title.alignment(Alignment::Center))
            .border_set(border::THICK)
            .render(area, buf);
    }
}