pub mod character_wizard_layout;
pub mod character_list_widget;
pub mod character_editor_widget;

use {
    crate::app::context::{CharacterList, Context}, character_editor_widget::CharacterEditorWidget, character_list_widget::CharacterListWidget, character_wizard_layout::calc_character_wizard_layouts, ratatui::{buffer::Buffer, crossterm::event::KeyCode, layout::Rect, widgets::{StatefulWidget, Widget}}, std::sync::Arc
};

#[derive(Copy, Clone)]
enum InputMode {
    Control,
    Editing,
    TextInput,
}

#[derive(Copy, Clone)]
pub struct CharacterWizard {
    input_mode: InputMode,
    pub character_list_widget: CharacterListWidget,
    pub character_editor_widget: CharacterEditorWidget,
}

enum CharacterWizardCommand {
    AddCharacter,
    EditCharacter,
    DeleteCharacter,
}

impl StatefulWidget for &CharacterWizard {
    type State = Context;

    fn render(self, _area: Rect, _buf: &mut Buffer, _state: &mut Self::State) {
        let layout_chunks = calc_character_wizard_layouts(&_area);

        StatefulWidget::render(&self.character_list_widget, 
            layout_chunks[0], _buf, &mut _state.dir_list);
        Widget::render(&self.character_editor_widget, layout_chunks[1], _buf)
    }
}

impl CharacterWizard {
    pub fn new() -> Self {
        CharacterWizard {
            input_mode: InputMode::Control,
            character_list_widget: CharacterListWidget {},
            character_editor_widget: CharacterEditorWidget {},
        }
    }

    pub fn handle_key_event(&mut self, key_code: KeyCode, _state: &mut Context) {
        match self.input_mode {
            InputMode::Control => match key_code 
            {
                KeyCode::Up => _state.dir_list.previous(),
                KeyCode::Down => _state.dir_list.next(),
                KeyCode::Enter => todo!(),
                _ => {}
            },
            InputMode::Editing => todo!(),
            InputMode::TextInput => todo!(),
        }
    }

    fn enable_character_edit(char_list: CharacterList) {
        if let Some(char_file) = char_list.get_selected_character_file() {
            // Open the file in the editor
            todo!()
        }
    }

    fn change_mode(&mut self, mode: InputMode) {
        self.input_mode = mode;
    }
}