// use crate::terminal_utils::prompt_enter_to_continue;
use crate::models::player::player::Player;
use crossterm::terminal;

pub struct BookBuilderRoutine {
    _player: Player,
}

impl BookBuilderRoutine {
    pub fn new(player: Player) -> Self {
        BookBuilderRoutine { _player: player }
    }

    // Runs the book builder routine until the player exits
    pub fn run(&mut self) {
        terminal::enable_raw_mode().unwrap();

        loop {
            println!("Do something");
            break;
        }

        terminal::disable_raw_mode().unwrap();
    }
}
