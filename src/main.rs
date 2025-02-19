mod args;
mod db;
mod dev;
mod game_engine;
pub mod models;
pub mod music;
pub mod terminal_utils;
pub mod tui;
pub mod world;
use args::parse_args;
pub use db::connection::get_connection;
pub use db::save::{delete_save, save_exists};
use game_engine::game_engine::GameEngine;
pub use game_engine::interactions;
pub use game_engine::routines;
pub use models::dialogue;
pub use world::navigation;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let game_args = parse_args();

    if game_args.dev {
        dev::dev::run();
        return Ok(()); // Early exit!
    }

    if game_args.new_game {
        if save_exists(None) {
            delete_save(None)?;
            println!("Previous save deleted. Starting a new game...");
        } else {
            println!("No existing save found. Starting a new game...");
        }
    }

    let mut game_engine = GameEngine::new();
    game_engine.start();

    Ok(())
}
