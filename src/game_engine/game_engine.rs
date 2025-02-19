use crate::db;
use crate::game_engine::routines;
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
    db_conn: Connection,
}

impl GameEngine {
    pub fn new() -> Self {
        let music_player = MusicPlayer::new();
        let db_conn = db::connection::get_connection(None)
            .expect("Failed to initialize database connection in game engine constructor");

        Self {
            music_player,
            db_conn,
        }
    }

    pub fn start(&mut self) {
        self.music_player.play();
        self.start_game().expect("Failed to start game");
    }

    // FIXME: Need to implement some of the concepts I've been working on in this main flow
    //        - Epics and stages
    //        - GameEngine::Routines and GameEngine::Interactions
    pub fn start_game(&mut self) -> Result<(), Box<dyn Error>> {
        // Run title screen and menu routine
        // - Show title screen
        // - TODO: Let user select save file
        TitleMenuRoutine.new().run();

        // FIXME: Need to figure out how to use Dialogue to drive this
        //        instead of hard-coding it all into the game engine
        //
        //        DialogueRoutine should take the dialogue as input then
        //        drive for a while?
        //
        //        The complexity of dialogue, and dialogue routine is
        //        sort of blocking me... So I'm just going to ignore
        //        that for a while and get some of the game actually
        //        implemented before I try to abstract too much.
        //
        //        Maybe if I just start building the story out,
        //        the dialogue abstraction will become more clear.
        //
        //        TLDR; hard-coded dialogue is fine for now.
        //
        let _dialogue = dialogue::load_dialogue("character_creation");

        // Start player create
        let mut is_new_player = false;

        // Start game by loading player, or
        // guiding them through the character creation intro
        let mut player = match Player::load(&self.db_conn)? {
            Some(player) => player,
            None => {
                // New player is being created
                is_new_player = true;

                terminal_utils::simulate_typing("Welcome to the wonderful world of The Book Game!");

                terminal_utils::prompt_enter_to_continue();

                terminal_utils::simulate_typing("What is your name?");
                let mut name = terminal_utils::get_input();

                // Loop until the name is not blank
                while name.trim().is_empty() {
                    terminal_utils::simulate_typing("Please enter a valid name.");
                    name = terminal_utils::get_input();
                }

                // We save with a default Gender and Height. These get overwritten in the next steps.
                let mut new_player = Player::new(name.clone(), Gender::Male, Height::Average);
                new_player = new_player.create(&self.db_conn);

                // Grab the newly created player's id from the database
                // and create the player's game state
                GameState::new(new_player.id).create(&self.db_conn);

                terminal_utils::simulate_typing(&format!("Hello, {}!", new_player.name));
                terminal_utils::prompt_enter_to_continue();

                new_player
            }
        };

        // Reload game state
        let mut game_state = GameState::load_for_player(&self.db_conn, player.id)?.unwrap();

        // Give special message if player is returning, but never completed character creation
        if !is_new_player && game_state.current_stage == "character_creation" {
            terminal_utils::simulate_typing("Looks like you're still creating your character.");
            terminal_utils::prompt_enter_to_continue();
        }

        if game_state.current_stage == "character_creation" {
            // Start gender selection experience
            let options = vec![Gender::Male, Gender::Female, Gender::Unspecified];
            let gender = self.menu_select("Please select your gender:", options);

            // Update player's gender
            player.gender = gender.clone();
            player.update(&self.db_conn)?;

            // Update game state, finished with choosing their name and gender
            game_state.current_stage = "book_tutorial".to_string();
            game_state.update(&self.db_conn);

            // Reload player
            player = Player::load(&self.db_conn)?.unwrap();

            terminal_utils::simulate_typing(&format!("You selected: {}", player.gender));
            terminal_utils::prompt_enter_to_continue();
        }

        terminal_utils::simulate_typing("Let's resume the adventure!");
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
