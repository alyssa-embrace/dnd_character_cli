use std::io::{self, Stdout};
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    prelude::CrosstermBackend,
    widgets::Widget,
    Frame,
};
use crate::{models::character::Character, views::{
    character_wizard::CharacterWizard,
    initiative_wizard::InitiativeWizard,
}};

enum AppMode {
    CharacterWizard,
    InitiativeWizard,
}

enum InputMode {
    Control,
    TextInput,
}

pub struct App {
    app_mode: AppMode,
    input_mode: InputMode,
    should_exit: bool,
    character_wizard: CharacterWizard,
    initiative_wizard: InitiativeWizard,
}

impl App {
    pub fn default() -> Self {
        App {
            app_mode: AppMode::CharacterWizard,
            input_mode: InputMode::Control,
            should_exit: false,
            character_wizard: CharacterWizard {
                src_directories: vec![],
            },
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

    fn render_frame(&self, frame: &mut Frame) {
        match self.app_mode {
            AppMode::CharacterWizard => frame.render_widget(&self.character_wizard, frame.size()),
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
        match self.input_mode {
            InputMode::Control => match event.code {
                KeyCode::Char('q') => self.markShouldExit(),
                KeyCode::Tab => self.change_mode(),
                KeyCode::Char('i') => self.app_mode = AppMode::InitiativeWizard,
                _ => {},
            },
            InputMode::TextInput => todo!(),
        }
        
    }

    fn markShouldExit(&mut self) {
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