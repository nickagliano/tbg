//! # GameEngine::Routines
//!
//! Routines are high-level execution loops that manage structured gameplay interactions.
//!
//! Each Routine encapsulates a distinct mode of player engagement, handling UI state,
//! input processing, and game logic flow. These routines ensure consistent execution
//! and seamless transitions between different gameplay contexts.
//!
//! ## Available Routines
//! - `Battle` – Turn-based combat system
//! - `Dialogue` – Interactive text sequences with decision-making elements.
//! - `BookBuilder` – Inventory-style interface for book building / editing (similar to  a deck builder)
//! - `WorldNavigation` – Free-roam movement and world exploration.
//!
//! Each Routine is built from `GameEngine::Interactions`, orchestrating user input,
//! game state updates, and rendering.
//!
//! # Example Usage
//! ```rust
//! use tbg::game_engine::routines::battle::BattleRoutine;
//! use tbg::db;
//! use tbg::models::player::player::Player;
//! use tbg::models::non_player_character::non_player_character::NPC;
//! use tbg::models::player::gender::Gender;
//! use tbg::models::player::height::Height;
//!
//! let conn = db::connection::get_connection(None).expect("Load connection");
//! let player = Player::new("Player1".to_string(), Gender::Male, Height::Average);
//! let npc = NPC::new("Jimmy".to_string(), Gender::Male);
//!
//! let mut battle = BattleRoutine::new(player, npc);
//! battle.run(); // Executes the battle loop
//! ```
pub mod battle;
pub mod book_builder;
pub mod book_tutorial;
pub mod dialogue;
pub mod title_menu;
pub mod world_navigation;
