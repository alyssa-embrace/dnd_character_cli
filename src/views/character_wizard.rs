pub mod character_wizard_layout;
pub mod character_list_widget;
pub mod character_src_widget;
pub mod character_editor_widget;

use {
    character_editor_widget::CharacterEditorWidget, character_list_widget::CharacterListWidget, character_src_widget::CharacterSrcWidget, std::sync::Arc
};

pub struct CharacterWizard {
    pub character_src_widget: CharacterSrcWidget,
    pub character_list_widget: CharacterListWidget,
    pub character_editor_widget: CharacterEditorWidget,
}