use crate::db;
use crate::models::dialogue::dialogue::{process_dialogue_text, Dialogue};
use crate::terminal_utils;
use crossterm::terminal;
use rusqlite::Connection;
use std::collections::HashMap;

pub struct DialogueRoutine {
    conn: Connection,
    root_node: Dialogue,
}

/// At a high-level, the DialogueRoutine is responsible for:
/// - Starting a dialogue tree and running it to "completion" (until an end-node is reached)
/// - Capturing and recording all decisions made in the dialogue tree (saved as `Decision`s in the database)
/// - Returning any value(s) that was/were determined by this dialogue tree
impl DialogueRoutine {
    pub fn new(root_node: Dialogue) -> Self {
        // TODO: I am opening a new conn here because I don't want to deal with lifetimes. Not great.
        let conn = db::connection::get_connection(None)
            .expect("Failed to initialize database connection in dialogue routine");
        DialogueRoutine { conn, root_node }
    }

    pub fn run(&mut self) -> Vec<String> {
        terminal::enable_raw_mode().unwrap();

        let mut current_node = self.root_node.clone(); // Clone the root node to start (not great)
        let mut args: Vec<String> = Vec::new(); // Vector to collect user inputs

        // Whitelisted player data (can come from DB)
        let mut whitelisted_data: HashMap<String, String> = HashMap::new();
        whitelisted_data.insert("player.name".to_string(), "Nick".to_string());
        whitelisted_data.insert("player.height".to_string(), "Tall".to_string());

        // Loop Steps:
        // 1) Print message
        // 2) Handle action (either get input, choice, or prompt enter to continue)
        // 3) Repeat (or end)
        loop {
            // Grab the dialogue node's text
            let unprocessed_text = current_node.get_text();

            // Replace templates in the text (e.g., {args[0], or {player.name})
            let processed_text = process_dialogue_text(&unprocessed_text, &args, &whitelisted_data);

            // TODO: Make `simulate_typing` a setting/parameter?
            if true {
                terminal_utils::simulate_typing(&format!("{}", processed_text));
            }

            if current_node.is_choices_node() {
                // TODO: Handle choices (gender selection)
                let _choice = 2;
            } else if current_node.is_input_node() {
                // Input!
                // Loop! need to store the "on invalid input" message in YAML?
                // Need to model the loops better...
                let mut input = terminal_utils::get_input();

                while input.trim().is_empty() {
                    terminal_utils::simulate_typing("Please enter a valid name.");
                    input = terminal_utils::get_input();
                }

                // Append user input to the args
                args.push(input.trim().to_string());
            } else {
                terminal_utils::prompt_enter_to_continue();
            }

            // End loop if we've reached the end of the dialogue tree
            if current_node.is_end_node() {
                terminal_utils::prompt_enter_to_continue();
                break;
            } else {
                current_node = current_node.next(&self.conn).unwrap().unwrap();
            }
        }

        terminal::disable_raw_mode().unwrap();

        args
    }
}
