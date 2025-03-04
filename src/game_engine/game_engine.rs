use super::routines::dialogue::DialogueRoutine;
use crate::db;
use crate::game_engine::routines;
// use crate::models::dialogue::decision::capture_decision;
use crate::models::dialogue::dialogue;
use crate::models::game_state;
use crate::models::player;
use crate::music::music_player::MusicPlayer;
use crate::terminal_utils;
use game_state::game_state::GameState;
use player::gender::Gender;
use player::height::Height;
use player::player::Player;
// TODO: Add battle::BattleRoutine, book_builder::BookBuilderRoutine
use super::InterfaceMode;
use routines::{
    book_tutorial::BookTutorialRoutine, title_menu::TitleMenuRoutine,
    world_navigation::WorldNavigationRoutine,
};
use rusqlite::{Connection, Result};
use std::error::Error;

pub struct GameEngine {
    music_player: MusicPlayer,
    conn: Connection,
}

impl GameEngine {
    pub fn new() -> Self {
        let music_player = MusicPlayer::new();
        let conn = db::connection::get_connection(None)
            .expect("Failed to initialize database connection in game engine constructor");

        Self { music_player, conn }
    }

    pub fn start(&mut self) {
        if false {
            // Hard-code this to off for now... It's annoying!
            self.music_player.play();
        }
        self.start_game().expect("Failed to start game");
    }

    pub fn start_game(&mut self) -> Result<(), Box<dyn Error>> {
        // Run title screen and menu routine
        // - Show title screen
        // - TODO: Let user select save file
        TitleMenuRoutine.new().run();

        // Start game by loading player, or
        // guiding them through the character creation intro
        let mut player = match Player::load(&self.conn)? {
            Some(player) => player,
            None => {
                // New player!
                // Load the character creation dialogue tree.
                let root_node = dialogue::load_by_root_name(&self.conn, "character_creation")
                    .unwrap()
                    .unwrap();

                let mut dialogue_routine = DialogueRoutine::new(root_node);
                let args = dialogue_routine.run();

                // Unpack args (player's name)
                let player_name = args[0].clone();

                // We save with a default Gender and Height. These get overwritten in the next steps.
                let mut new_player = Player::new(player_name, Gender::Male, Height::Average);
                new_player = new_player.create(&self.conn);

                // Using the newly created player's id from the database,
                // create the player's game state
                GameState::new(new_player.id).create(&self.conn);

                new_player
            }
        };

        // Reload game state
        let mut game_state = GameState::load_for_player(&self.conn, player.id)?.unwrap();

        // FIXME: Implement this as a routine
        if game_state.current_stage == "character_creation" {
            // Start gender selection experience
            // FIXME: start_timer (so we can capture deliberation time)...
            let options = vec![Gender::Male, Gender::Female, Gender::Unspecified];
            let gender = self.menu_select(
                "Wait. One more thing before we continue. Are you a...:",
                options,
            );

            // decision::capture_decision(...)

            // Update player's gender
            player.gender = gender.clone();
            player.update(&self.conn)?;

            terminal_utils::simulate_typing(&format!("You selected: {}", player.gender));
            terminal_utils::prompt_enter_to_continue();

            let options = vec![
                Height::VeryShort,
                Height::Short,
                Height::Average,
                Height::Tall,
                Height::VeryTall,
            ];
            // FIXME: start_timer (so we can capture deliberation time)...
            let height = self.menu_select(
                "And, I know this is maybe a weird question, but I have to ask. How tall are you?:",
                options,
            );

            // decision::capture_decision(...)

            // Update player's height
            player.height = height.clone();
            player.update(&self.conn)?;

            // Update game state, finished with choosing their name and gender
            game_state.current_stage = "book_tutorial".to_string();
            game_state.update(&self.conn);

            // Reload player
            player = Player::load(&self.conn)?.unwrap();

            terminal_utils::simulate_typing(&format!("You selected: {}", player.height));
            terminal_utils::prompt_enter_to_continue();

            terminal_utils::simulate_typing(&format!(
                "Interesting. Sorry for the blunt questions.\n\nI'm sort of hard of seeing. And, well, my eyes often deceive me.",
            ));
            terminal_utils::prompt_enter_to_continue();
        }

        terminal_utils::simulate_typing(
            "Let's explore!\n\n(Move down two spaces, and over to the right one space)",
        );
        terminal_utils::prompt_enter_to_continue();

        let new_inferface_mode = WorldNavigationRoutine::new(game_state).run();
        // let new_inferface_mode = InterfaceMode::BookBuilder;  // swap out for easier development

        // Handle new interface mode (for now this is just BookBuilder)
        if new_inferface_mode == InterfaceMode::BookBuilder {
            BookTutorialRoutine::new(player).run();
        } else {
            panic!("For development purposes, this is just handling BookBuilder")
        }

        Ok(())
    }
}
