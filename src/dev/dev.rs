use crate::db;
use crate::models::game_state;
use crate::models::player;
use crate::routines::book_tutorial::BookTutorialRoutine;
use crossterm::terminal;
use game_state::game_state::GameState;
use player::gender::Gender;
use player::height::Height;
use player::player::Player;
use std::error::Error;

/// WIP implementation of book building!
pub fn run() -> Result<(), Box<dyn Error>> {
    // Enable raw mode
    terminal::enable_raw_mode().unwrap();

    let conn = db::connection::get_connection(None)
        .expect("Failed to initialize database connection in game engine constructor");

    // FIXME: Setting up a player is hard -- can I mock?
    //        How can I re-use my mocked stuff here?

    let player = match Player::load(&conn)? {
        Some(player) => player,
        None => {
            // New player!
            // Load the character creation dialogue tree.

            let player_name = "Jimmynick".to_string();

            // We save with a default Gender and Height. These get overwritten in the next steps.
            let mut new_player = Player::new(player_name, Gender::Male, Height::Average);
            new_player = new_player.create(&conn);

            // Using the newly created player's id from the database,
            // create the player's game state
            GameState::new(new_player.id).create(&conn);

            new_player
        }
    };

    BookTutorialRoutine::new(player).run();

    terminal::disable_raw_mode().unwrap();

    return Ok(());
}
