pub mod character_wizard_layout;
pub mod character_list_widget;
pub mod character_src_widget;
pub mod character_editor_widget;

use {
    crate::app::InputMode, character_editor_widget::CharacterEditorWidget, character_list_widget::CharacterListWidget, character_src_widget::CharacterSrcWidget, ratatui::crossterm::event::KeyCode, std::sync::Arc
};

pub struct CharacterWizard {
    pub character_src_widget: CharacterSrcWidget,
    pub character_list_widget: CharacterListWidget,
    pub character_editor_widget: CharacterEditorWidget,
}

impl CharacterWizard {
    pub fn handle_key_event(&mut self, key_code: KeyCode, input_mode: InputMode) {
        match key_code {
            _ => {}
        }
    }
}