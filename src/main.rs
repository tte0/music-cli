mod app;
mod mode;
mod modes;
mod utils;

use app::App;
use std::io;

fn main() -> io::Result<()> {
    let mut app = App::new();
    app.run()
}