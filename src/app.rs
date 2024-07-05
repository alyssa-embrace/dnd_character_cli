pub mod context;

use std::io::{self, Stdout};
use context::Context;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind}, layout::{Constraint, Direction, Layout, Rect}, prelude::CrosstermBackend, Frame
};
use crate::views::{
    character_wizard::{character_editor_widget::CharacterEditorWidget, character_list_widget::CharacterListWidget, CharacterWizard},
    initiative_wizard::InitiativeWizard,
};

pub enum AppMode {
    CharacterWizard,
    InitiativeWizard,
}

enum AppCommand {
    ChangeMode,
    SignalEscape,
}


pub struct App<'a> {
    pub app_mode: AppMode,
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
            should_exit: false,
            ref_context: context,
            character_wizard: CharacterWizard::new(),
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
            AppMode::CharacterWizard => 
                frame.render_stateful_widget(&self.character_wizard, frame.size(), &mut self.ref_context),
            AppMode::InitiativeWizard => frame.render_widget(&self.initiative_wizard, frame.size()),
        }   
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
            KeyCode::Esc => self.handle_command(AppCommand::SignalEscape),
            _ => self.forward_key_event(event)
        }
    }

    fn handle_command(&mut self, command: AppCommand) {
        match command {
            AppCommand::ChangeMode => self.change_mode(),
            AppCommand::SignalEscape => self.signal_escape(),
        }
    }

    fn forward_key_event(&mut self, event: KeyEvent) {
        match self.app_mode {
            AppMode::CharacterWizard => 
                self.character_wizard.handle_key_event(event.code, &mut self.ref_context),
            AppMode::InitiativeWizard => todo!(),
        }
    }

    fn signal_escape(&mut self) {
        if let Some(handler) = &self.escape_handler {
            handler();
        } else {
            self.should_exit = true;
        }
    }

    fn change_mode(&mut self) {
        let mode = match self.app_mode {
            AppMode::CharacterWizard => AppMode::InitiativeWizard,
            AppMode::InitiativeWizard => AppMode::CharacterWizard,
        };
        self.app_mode = mode;
    }
}