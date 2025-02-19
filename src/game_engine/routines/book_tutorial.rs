// use crate::game_engine::interactions;
use crate::models::player::player::Player;
// use crate::models:: // TODO: Fill in book model(s), grab Book, PlayerBook
use crate::terminal_utils;
// use crate::tui; // FIXME: Use TUI for rendering books during
use crossterm::terminal;

pub struct BookTutorialRoutine {
    _player: Player,
}

impl BookTutorialRoutine {
    pub fn new(player: Player) -> Self {
        BookTutorialRoutine { _player: player }
    }

    // Runs the book tutorial, in which the player will learn how to
    // use books, culminating in them choosing their first book.
    pub fn run(&mut self) {
        terminal::enable_raw_mode().unwrap();

        terminal_utils::simulate_typing("Okay, young one.");
        terminal_utils::prompt_enter_to_continue();

        terminal_utils::simulate_typing("It's time to begin your own story.");
        terminal_utils::prompt_enter_to_continue();

        // TODO: Show a book, closed.
        //

        loop {
            break;
        }

        // Create the player's book after choosing
        // PlayerBook::new(player, book)... something

        terminal::disable_raw_mode().unwrap();
    }
}
