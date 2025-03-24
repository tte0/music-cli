use crate::app::App;
use crate::mode::Mode;
use crate::utils;
use crossterm::event::{KeyCode, KeyEvent};
use std::io::{self, stdout, Write};

pub struct MainMode {}

impl Mode for MainMode {
    fn new() -> Self {
        Self {}
    }

    fn render(&self) -> io::Result<()> {
        println!("=== MAIN MODE ===");
        println!();
        println!("Welcome to the multi-mode CLI application!");
        println!();
        println!("Press keys 1-3 to switch modes:");
        println!("  1: Main mode");
        println!("  2: Info mode");
        println!("  3: Settings mode");
        println!("  h: Help");
        println!("  q: Quit");
        println!();
        println!("Main mode specific commands:");
        println!("  a: Action 1");
        println!("  b: Action 2");
        
        stdout().flush()
    }

    fn handle_key(&self, key: KeyEvent, app: &mut App) -> io::Result<()> {
        match key.code {
            KeyCode::Char('a') => {
                utils::clear_screen()?;
                println!("Executing Main Mode - Action 1");
                println!("\nPress any key to return...");
                utils::wait_for_key()?;
                utils::clear_screen()?;
                app.render()
            }
            KeyCode::Char('b') => {
                utils::clear_screen()?;
                println!("Executing Main Mode - Action 2");
                println!("\nPress any key to return...");
                utils::wait_for_key()?;
                utils::clear_screen()?;
                app.render()
            }
            _ => Ok(()),
        }
    }
}