use crossterm::event::KeyEvent;
use std::io;

// Forward reference to App struct
use crate::app::App;

// Define the different application modes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppMode {
    Main,
    Info,
    Settings,
    Help,
    Exit,
}

// Trait that each mode must implement
pub trait Mode {
    // Initialize the mode
    fn new() -> Self where Self: Sized;
    
    // Render the mode's UI
    fn render(&self) -> io::Result<()>;
    
    // Handle mode-specific keys
    fn handle_key(&self, key: KeyEvent, app: &mut App) -> io::Result<()>;
}