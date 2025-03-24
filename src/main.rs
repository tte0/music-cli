use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{self, ClearType},
};
use std::io::{self, stdout, Write};
use std::time::Duration;

// Define the different application modes
#[derive(Debug, Clone, Copy, PartialEq)]
enum AppMode {
    Main,
    Info,
    Settings,
    Help,
    Exit,
}

// Struct to manage the application state
struct App {
    mode: AppMode,
    running: bool,
}

impl App {
    fn new() -> Self {
        Self {
            mode: AppMode::Main,
            running: true,
        }
    }

    // Change to a different mode
    fn change_mode(&mut self, mode: AppMode) {
        self.mode = mode;
        self.clear_screen().unwrap();
        self.render().unwrap();
    }

    // Clear the terminal screen
    fn clear_screen(&self) -> io::Result<()> {
        execute!(
            stdout(),
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0, 0)
        )
    }

    // Handle key events
    fn handle_key_event(&mut self, key: KeyEvent) -> io::Result<()> {
        match key.code {
            KeyCode::Char('q') => {
                self.running = false;
                Ok(())
            }
            KeyCode::Char('1') => {
                self.change_mode(AppMode::Main);
                Ok(())
            }
            KeyCode::Char('2') => {
                self.change_mode(AppMode::Info);
                Ok(())
            }
            KeyCode::Char('3') => {
                self.change_mode(AppMode::Settings);
                Ok(())
            }
            KeyCode::Char('h') => {
                self.change_mode(AppMode::Help);
                Ok(())
            }
            _ => {
                // Mode-specific key handling
                match self.mode {
                    AppMode::Main => self.handle_main_mode_keys(key),
                    AppMode::Info => self.handle_info_mode_keys(key),
                    AppMode::Settings => self.handle_settings_mode_keys(key),
                    AppMode::Help => self.handle_help_mode_keys(key),
                    AppMode::Exit => Ok(()),
                }
            }
        }
    }

    // Render the current mode's UI
    fn render(&self) -> io::Result<()> {
        match self.mode {
            AppMode::Main => self.render_main_mode(),
            AppMode::Info => self.render_info_mode(),
            AppMode::Settings => self.render_settings_mode(),
            AppMode::Help => self.render_help_mode(),
            AppMode::Exit => Ok(()),
        }
    }

    // Main mode implementation
    fn render_main_mode(&self) -> io::Result<()> {
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

    fn handle_main_mode_keys(&mut self, key: KeyEvent) -> io::Result<()> {
        match key.code {
            KeyCode::Char('a') => {
                self.clear_screen()?;
                println!("Executing Main Mode - Action 1");
                println!("\nPress any key to return...");
                self.wait_for_key()?;
                self.render()
            }
            KeyCode::Char('b') => {
                self.clear_screen()?;
                println!("Executing Main Mode - Action 2");
                println!("\nPress any key to return...");
                self.wait_for_key()?;
                self.render()
            }
            _ => Ok(()),
        }
    }

    // Info mode implementation
    fn render_info_mode(&self) -> io::Result<()> {
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

    fn handle_info_mode_keys(&mut self, key: KeyEvent) -> io::Result<()> {
        match key.code {
            KeyCode::Char('v') => {
                self.clear_screen()?;
                println!("Viewing Stats:");
                println!("- Users: 1,024");
                println!("- Sessions: 8,192");
                println!("- Uptime: 99.9%");
                println!("\nPress any key to return...");
                self.wait_for_key()?;
                self.render()
            }
            KeyCode::Char('d') => {
                self.clear_screen()?;
                println!("Generating report...");
                // Simulate work
                for i in 0..=10 {
                    self.clear_screen()?;
                    println!("Generating report... {}0%", i);
                    std::thread::sleep(Duration::from_millis(100));
                }
                println!("\nReport complete!");
                println!("\nPress any key to return...");
                self.wait_for_key()?;
                self.render()
            }
            _ => Ok(()),
        }
    }

    // Settings mode implementation
    fn render_settings_mode(&self) -> io::Result<()> {
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

    fn handle_settings_mode_keys(&mut self, key: KeyEvent) -> io::Result<()> {
        match key.code {
            KeyCode::Char('c') => {
                self.clear_screen()?;
                println!("Changing configuration");
                println!("Option [1/3]: Debug mode (y/n): ");
                self.wait_for_key()?;
                println!("Option [2/3]: Verbose logging (y/n): ");
                self.wait_for_key()?;
                println!("Option [3/3]: Dark mode (y/n): ");
                self.wait_for_key()?;
                println!("\nConfiguration updated!");
                println!("\nPress any key to return...");
                self.wait_for_key()?;
                self.render()
            }
            KeyCode::Char('r') => {
                self.clear_screen()?;
                println!("Resetting all settings to default...");
                std::thread::sleep(Duration::from_millis(1000));
                println!("Settings reset complete!");
                println!("\nPress any key to return...");
                self.wait_for_key()?;
                self.render()
            }
            _ => Ok(()),
        }
    }

    // Help mode implementation
    fn render_help_mode(&self) -> io::Result<()> {
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

    fn handle_help_mode_keys(&mut self, _key: KeyEvent) -> io::Result<()> {
        // Any key in help mode returns to the previous mode
        self.change_mode(AppMode::Main);
        Ok(())
    }

    // Wait for a key press
    fn wait_for_key(&self) -> io::Result<()> {
        loop {
            if let Event::Key(_) = event::read()? {
                break;
            }
        }
        Ok(())
    }

    // Main application loop
    fn run(&mut self) -> io::Result<()> {
        // Setup terminal
        terminal::enable_raw_mode()?;
        self.clear_screen()?;
        
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
        self.clear_screen()?;
        
        println!("Thank you for using the multi-mode CLI application!");
        
        Ok(())
    }
}

fn main() -> io::Result<()> {
    let mut app = App::new();
    app.run()
}