use crate::mode::AppMode;
use crate::modes::{help_mode::HelpMode, info_mode::InfoMode, main_mode::MainMode, settings_mode::SettingsMode};
use crate::utils;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal,
};
use std::io::{self, stdout, Write};

// Trait for mode implementations to implement
use crate::mode::Mode;

// Struct to manage the application state
pub struct App {
    current_mode: AppMode,
    running: bool,
    // Mode instances
    main_mode: MainMode,
    info_mode: InfoMode,
    settings_mode: SettingsMode,
    help_mode: HelpMode,
}

impl App {
    pub fn new() -> Self {
        Self {
            current_mode: AppMode::Main,
            running: true,
            main_mode: MainMode::new(),
            info_mode: InfoMode::new(),
            settings_mode: SettingsMode::new(),
            help_mode: HelpMode::new(),
        }
    }

    // Change to a different mode
    pub fn change_mode(&mut self, mode: AppMode) {
        self.current_mode = mode;
        utils::clear_screen().unwrap();
        self.render().unwrap();
    }

    // Handle key events
    fn handle_key_event(&mut self, key: KeyEvent) -> io::Result<()> {
        // Handle global keys first
        match key.code {
            KeyCode::Char('q') => {
                self.running = false;
                return Ok(());
            }
            KeyCode::Char('1') => {
                self.change_mode(AppMode::Main);
                return Ok(());
            }
            KeyCode::Char('2') => {
                self.change_mode(AppMode::Info);
                return Ok(());
            }
            KeyCode::Char('3') => {
                self.change_mode(AppMode::Settings);
                return Ok(());
            }
            KeyCode::Char('h') => {
                self.change_mode(AppMode::Help);
                return Ok(());
            }
            _ => {}
        }

        // Handle mode-specific keys
        match self.current_mode {
            AppMode::Main => self.main_mode.handle_key(key, self),
            AppMode::Info => self.info_mode.handle_key(key, self),
            AppMode::Settings => self.settings_mode.handle_key(key, self),
            AppMode::Help => self.help_mode.handle_key(key, self),
            AppMode::Exit => Ok(()),
        }
    }

    // Render the current mode's UI
    pub fn render(&self) -> io::Result<()> {
        match self.current_mode {
            AppMode::Main => self.main_mode.render(),
            AppMode::Info => self.info_mode.render(),
            AppMode::Settings => self.settings_mode.render(),
            AppMode::Help => self.help_mode.render(),
            AppMode::Exit => Ok(()),
        }
    }

    // Main application loop
    pub fn run(&mut self) -> io::Result<()> {
        // Setup terminal
        terminal::enable_raw_mode()?;
        utils::clear_screen()?;

        // Initial render
        self.render()?;

        // Event loop
        while self.running {
            if let Event::Key(key) = event::read()? {
                self.handle_key_event(key)?;
            }
        }

        // Cleanup and exit
        terminal::disable_raw_mode()?;
        utils::clear_screen()?;

        println!("Thank you for using the multi-mode CLI application!");

        Ok(())
    }
}