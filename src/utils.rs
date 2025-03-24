use crossterm::{
    cursor,
    event::{self, Event},
    execute,
    terminal::ClearType,
};
use std::io::{self, stdout};
use std::time::Duration;

// Clear the terminal screen
pub fn clear_screen() -> io::Result<()> {
    execute!(
        stdout(),
        crossterm::terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    )
}

// Wait for any key press
pub fn wait_for_key() -> io::Result<()> {
    loop {
        if let Event::Key(_) = event::read()? {
            break;
        }
    }
    Ok(())
}

// Simulate work with a progress bar
pub fn simulate_progress(task_name: &str, steps: u8, delay_ms: u64) -> io::Result<()> {
    for i in 0..=steps {
        clear_screen()?;
        println!("{}... {}%", task_name, (i as f32 / steps as f32 * 100.0) as u8);
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    Ok(())
}