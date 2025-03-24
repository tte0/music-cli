use crate::app::App;
use crate::mode::{AppMode, Mode};
use crate::utils;
use crossterm::event::KeyEvent;
use std::io::{self, stdout, Write};

pub struct HelpMode {}

impl Mode for HelpMode {
    fn new() -> Self {
        Self {}
    }

    fn render(&self) -> io::Result<()> {
        println!("=== HELP MODE ===");
        println!();
        println!("Multi-mode CLI Application Help");
        println!();
        println!("Global commands:");
        println!("  1: Switch to Main mode");
        println!("  2: Switch to Info mode");
        println!("  3: Switch to Settings mode");
        println!("  h: Show this help");
        println!("  q: Quit application");
        println!();
        println!("Each mode has its own specific commands.");
        println!("Return to a mode to see its available commands.");
        println!();
        println!("Press any key to return to previous mode...");
        
        stdout().flush()
    }

    fn handle_key(&self, _key: KeyEvent, app: &mut App) -> io::Result<()> {
        // Any key in help mode returns to the main mode
        app.change_mode(AppMode::Main);
        Ok(())
    }
}