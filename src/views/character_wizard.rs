pub mod character_wizard_layout;
pub mod character_list_widget;
pub mod character_src_widget;
pub mod character_editor_widget;

use {
    character_editor_widget::CharacterEditorWidget, character_list_widget::CharacterListWidget, character_src_widget::CharacterSrcWidget, std::sync::Arc
};

pub struct CharacterWizard<'a> {
    pub character_src_widget: CharacterSrcWidget<'a>,
    pub character_list_widget: CharacterListWidget<'a>,
    pub character_editor_widget: CharacterEditorWidget<'a>,
}