use crossterm::terminal;

/// WIP implementation of book building!
pub fn run() {
    // Enable raw mode
    terminal::enable_raw_mode().unwrap();

    // loop {
    //     // Update size dynamically
    //     viewport.update_size();
    // }

    terminal::disable_raw_mode().unwrap();
}
