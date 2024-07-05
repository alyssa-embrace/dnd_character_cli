pub mod context;

use std::io::{self, Stdout};
use context::Context;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind}, layout::{Constraint, Direction, Layout, Rect}, prelude::CrosstermBackend, Frame
};
use crate::views::{
    character_wizard::{character_editor_widget::CharacterEditorWidget, character_list_widget::CharacterListWidget, character_src_widget::CharacterSrcWidget, character_wizard_layout::calc_character_wizard_layouts, CharacterWizard},
    initiative_wizard::InitiativeWizard,
};

enum AppMode {
    CharacterWizard,
    InitiativeWizard,
}

#[derive(Clone)]
pub enum InputMode {
    Control,
    TextInput,
}

pub struct App<'a> {
    app_mode: AppMode,
    input_mode: InputMode,
    should_exit: bool,
    ref_context: &'a mut Context,
    character_wizard: CharacterWizard,
    initiative_wizard: InitiativeWizard,
    escape_handler: Option<Box<dyn Fn()>>,
}

impl<'a> App<'a> {
    pub fn default(context: &'a mut Context) -> Self {
        App {
            app_mode: AppMode::CharacterWizard,
            input_mode: InputMode::Control,
            should_exit: false,
            ref_context: context,
            character_wizard: CharacterWizard {
                character_src_widget: CharacterSrcWidget {},
                character_list_widget: CharacterListWidget {},
                character_editor_widget: CharacterEditorWidget {},
            },
            escape_handler: None,
            initiative_wizard: InitiativeWizard {},
        }
    }

    pub fn run(&mut self, terminal: &mut ratatui::Terminal<CrosstermBackend<Stdout>>) -> io::Result<()> {
        while !self.should_exit {
            terminal.draw(|frame: &mut Frame| self.render_frame(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn render_frame(&mut self, frame: &mut Frame) {
        match self.app_mode {
            AppMode::CharacterWizard => self.render_character_wizard_frame(frame),
            AppMode::InitiativeWizard => frame.render_widget(&self.initiative_wizard, frame.size()),
        }   
    }

    fn render_character_wizard_frame(&mut self, frame: &mut Frame) {
        let layout_chunks = calc_character_wizard_layouts(frame);
        let left_inner_layout = &layout_chunks[1];
        let right_inner_layout = &layout_chunks[2];

        frame.render_widget(&self.character_wizard.character_src_widget, left_inner_layout[0]);
        frame.render_stateful_widget(&self.character_wizard.character_list_widget, left_inner_layout[1], &mut self.ref_context.dir_list);
        frame.render_widget(&self.character_wizard.character_editor_widget, right_inner_layout[0]);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(event) => {
                self.handle_key_event(event)
            },
            _ => {},
        }
        Ok(())
    }

    fn handle_key_event(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Esc => {
                if let Some(handler) = &self.escape_handler {
                    handler();
                } else {
                    self.mark_should_exit();
                }
            },
            KeyCode::F(1) => self.change_mode(),
            _ => match self.app_mode {
                AppMode::CharacterWizard => 
                    self.character_wizard.handle_key_event(event.code, self.input_mode.clone()),
                AppMode::InitiativeWizard => todo!(),
            }
        }
    }

    fn mark_should_exit(&mut self) {
        self.should_exit = true;
    }

    fn change_mode(&mut self) {
        let mode = match self.app_mode {
            AppMode::CharacterWizard => AppMode::InitiativeWizard,
            AppMode::InitiativeWizard => AppMode::CharacterWizard,
        };
        self.app_mode = mode;
    }
}