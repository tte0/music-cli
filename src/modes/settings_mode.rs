use crate::app::App;
use crate::mode::Mode;
use crate::utils;
use crossterm::event::{KeyCode, KeyEvent};
use std::io::{self, stdout, Write};
use std::time::Duration;

pub struct SettingsMode {}

impl Mode for SettingsMode {
    fn new() -> Self {
        Self {}
    }

    fn render(&self) -> io::Result<()> {
        println!("=== SETTINGS MODE ===");
        println!();
        println!("Configure application settings here.");
        println!();
        println!("Press keys 1-3 to switch modes:");
        println!("  1: Main mode");
        println!("  2: Info mode");
        println!("  3: Settings mode");
        println!("  h: Help");
        println!("  q: Quit");
        println!();
        println!("Settings mode specific commands:");
        println!("  c: Change config");
        println!("  r: Reset settings");
        
        stdout().flush()
    }

    fn handle_key(&self, key: KeyEvent, app: &mut App) -> io::Result<()> {
        match key.code {
            KeyCode::Char('c') => {
                utils::clear_screen()?;
                println!("Changing configuration");
                println!("Option [1/3]: Debug mode (y/n): ");
                utils::wait_for_key()?;
                println!("Option [2/3]: Verbose logging (y/n): ");
                utils::wait_for_key()?;
                println!("Option [3/3]: Dark mode (y/n): ");
                utils::wait_for_key()?;
                println!("\nConfiguration updated!");
                println!("\nPress any key to return...");
                utils::wait_for_key()?;
                utils::clear_screen()?;
                app.render()
            }
            KeyCode::Char('r') => {
                utils::clear_screen()?;
                println!("Resetting all settings to default...");
                std::thread::sleep(Duration::from_millis(1000));
                println!("Settings reset complete!");
                println!("\nPress any key to return...");
                utils::wait_for_key()?;
                utils::clear_screen()?;
                app.render()
            }
            _ => Ok(()),
        }
    }
}