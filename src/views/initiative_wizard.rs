use ratatui::{
    layout::Alignment, prelude::{Buffer, Rect}, style::Stylize, symbols::border, widgets::{
        block::Title, Block, Widget
    }
};

pub struct InitiativeWizard {}

impl Widget for &InitiativeWizard {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Initiative Wizard ".bold());
        Block::bordered()
            .title(title.alignment(Alignment::Center))
            .border_set(border::THICK)
            .render(area, buf);
    }
}