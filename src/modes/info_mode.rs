use crate::app::App;
use crate::mode::Mode;
use crate::utils;
use crossterm::event::{KeyCode, KeyEvent};
use std::io::{self, stdout, Write};

pub struct InfoMode {}

impl Mode for InfoMode {
    fn new() -> Self {
        Self {}
    }

    fn render(&self) -> io::Result<()> {
        println!("=== INFO MODE ===");
        println!();
        println!("This is the info mode. Here you can view various information.");
        println!();
        println!("Press keys 1-3 to switch modes:");
        println!("  1: Main mode");
        println!("  2: Info mode");
        println!("  3: Settings mode");
        println!("  h: Help");
        println!("  q: Quit");
        println!();
        println!("Info mode specific commands:");
        println!("  v: View stats");
        println!("  d: Display report");
        
        stdout().flush()
    }

    fn handle_key(&self, key: KeyEvent, app: &mut App) -> io::Result<()> {
        match key.code {
            KeyCode::Char('v') => {
                utils::clear_screen()?;
                println!("Viewing Stats:");
                println!("- Users: 1,024");
                println!("- Sessions: 8,192");
                println!("- Uptime: 99.9%");
                println!("\nPress any key to return...");
                utils::wait_for_key()?;
                utils::clear_screen()?;
                app.render()
            }
            KeyCode::Char('d') => {
                utils::clear_screen()?;
                utils::simulate_progress("Generating report", 10, 100)?;
                println!("\nReport complete!");
                println!("\nPress any key to return...");
                utils::wait_for_key()?;
                utils::clear_screen()?;
                app.render()
            }
            _ => Ok(()),
        }
    }
}