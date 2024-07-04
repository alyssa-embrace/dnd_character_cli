use ratatui::{
    buffer::Buffer, layout::{Alignment, Rect}, style::{Modifier, Style, Stylize}, symbols::border, widgets::{block::Title, Block, Borders, HighlightSpacing, List, ListItem, ListState, Paragraph, StatefulWidget, Widget}
};

use crate::app::context::DirectoryList;

#[derive(Copy, Clone)]
pub struct CharacterListWidget {
}

impl StatefulWidget for &CharacterListWidget {
    type State = DirectoryList;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        self.render_list(area, buf, state);
    }
}

impl CharacterListWidget {
    fn render_list(self, area: Rect, buf: &mut Buffer, state: &mut DirectoryList) {
        let title = Title::from(" Character List ".bold());
        let outer_block= Block::new()
            .borders(Borders::ALL)
            .title(title.alignment(Alignment::Center))
            .border_set(border::THICK);
        
        let inner_area = outer_block.inner(area);
        let items: Vec<ListItem> = state.directories.iter()
            .enumerate().map(|(i, character_file)| {
                ListItem::new(character_file.clone())
            }).collect();
        
        let items = List::new(items)
            .block(Block::default())
            .highlight_style(Style::default()
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::REVERSED)
            )
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        outer_block.render(area, buf);
        StatefulWidget::render(items, inner_area, buf, &mut state.state);
    }
}