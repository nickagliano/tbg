use super::character_type::{deserialize_character_type, CharacterType};
use super::dialogue_response::DialogueResponse;
use crate::db::DIALOGUE_TABLE;
use regex::Regex;
use rusqlite::params;
use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

const DIALOGUE_SEEDS_ROOT: &str = "src/db/seeds/dialogue_fixtures/";

// Represents a single dialogue entry / node of converation
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dialogue {
    pub id: u32,

    // Polymorphic, FK to either Player or NPC
    character_id: u32,

    // Polymorphic, "player" or "non_player_character"
    #[serde(deserialize_with = "deserialize_character_type")]
    character_type: CharacterType,

    // Flag to mark root dialogue nodes.
    root: Option<bool>,

    // Optional, a name for the root node, to allow querying for specific root nodes.
    root_name: Option<String>,

    // The main text of the dialogue.
    //
    // Oftentimes `terminal_utils::simulate_typing` is used--this is the message that will be written
    // with that style.
    text: String,

    // Optional, because not all dialogues will have responses. (e.g., press enter to continue)
    responses: Option<Vec<DialogueResponse>>,

    // Indicates whether the dialogue requires free-form input from the player (e.g, entering their name).
    has_input: Option<bool>,
    input_type: Option<String>,
    next_id: Option<u32>, // The ID of the next dialogue entry to show after this one.
                          // FIXME: Add created at, updated at
}

impl Dialogue {
    pub fn new(
        id: u32,
        character_id: u32,
        character_type: CharacterType,
        root: Option<bool>,
        root_name: Option<String>,
        text: String,
        responses: Option<Vec<DialogueResponse>>,
        has_input: Option<bool>,
        input_type: Option<String>,
        next_id: Option<u32>,
    ) -> Self {
        Dialogue {
            id,
            character_id,
            character_type,
            root,
            root_name,
            text,
            responses,
            has_input,
            input_type,
            next_id,
        }
    }

    // Save the dialogue to the database and insert responses into the dialogue responses table
    pub fn create(&self, conn: &Connection) -> u32 {
        // FIXME: Do this in a transaction (so we can grab the dialogue id reliably?)

        // Insert the Dialogue
        conn.execute(
                &format!(
                    "INSERT INTO {} (character_id, character_type, text, root, root_name, has_input, input_type, next_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                    DIALOGUE_TABLE
                ),
                params![
                    self.character_id,
                    self.character_type,
                    self.text,
                    self.root,
                    self.root_name,
                    self.has_input,
                    self.input_type,
                    self.next_id,
                ],
            ).unwrap();

        // Retrieve the id of the inserted Dialogue
        // FIXME: This is a little scary
        let dialogue_id: u32 = conn.last_insert_rowid() as u32;

        // If there are responses, insert them into the dialogue_responses table
        // if let Some(responses) = &self.responses {
        //     for dialogue_response in responses {
        //         let mut dialogue_response = dialogue_response.clone(); // Clone so we can mutate
        //         dialogue_response.dialogue_id = dialogue_id; // set dialogue id to parent's id
        //         println!("{:?}", dialogue_response);
        //         dialogue_response.create(conn).unwrap();
        //     }
        // }

        dialogue_id
    }

    // Method to check if the dialogue entry is an end node
    pub fn is_end_node(&self) -> bool {
        // If there is no `next_id`, it's the end of the dialogue tree
        self.next_id.is_none()
    }

    // Method to check if the dialogue entry is an end node
    pub fn is_input_node(&self) -> bool {
        self.has_input.unwrap_or(false)
    }

    // Method to check if the dialogue entry is a root node
    pub fn is_root_node(&self) -> bool {
        self.root.unwrap_or(false)
    }

    // Method to check if the dialogue entry has choices
    // If the there is Some(Vec<DialogueResponse>), we check to see if it's non-empty.
    pub fn is_choices_node(&self) -> bool {
        self.responses.as_ref().map_or(false, |r| !r.is_empty())
    }

    pub fn next(&self, conn: &Connection) -> Result<Option<Dialogue>> {
        // Check if next_id exists, and panic if not
        let next_id = self.next_id.expect("No next_id found, panicking");

        // Now that we have the next_id, load the next dialogue
        load(conn, next_id)
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn get_responses(&self) -> &Option<Vec<DialogueResponse>> {
        &self.responses
    }

    pub fn get_responses_mut(&mut self) -> Option<&mut Vec<DialogueResponse>> {
        self.responses.as_mut()
    }
}

pub fn load(conn: &Connection, id: u32) -> Result<Option<Dialogue>> {
    let mut stmt = conn.prepare(
    &format!("SELECT id, character_id, character_type, root, root_name, text, has_input, input_type, next_id
                FROM {}
                WHERE id = {}
                LIMIT 1;",
            DIALOGUE_TABLE,
            id),
    )?;
    let mut dialogue_iter = stmt.query_map([], |row| {
        let id: u32 = row.get(0)?;
        let character_id: u32 = row.get(1)?;
        let character_type: CharacterType = row.get(2)?;
        let root: Option<bool> = row.get(3)?;
        let root_name: Option<String> = row.get(4)?;
        let text: String = row.get(5)?;
        let has_input: Option<bool> = row.get(6)?;
        let input_type: Option<String> = row.get(7)?;
        let next_id: Option<u32> = row.get(8)?;

        // Fetch DialogueResponses based on the dialogue_id
        let mut resp_stmt = conn.prepare(
            "SELECT id, dialogue_id, text, next_id FROM dialogue_responses WHERE dialogue_id = ?",
        )?;
        let responses_iter = resp_stmt.query_map([id], |resp_row| {
            let resp_id: u32 = resp_row.get(0)?;
            let dialogue_id: u32 = resp_row.get(1)?;
            let text: String = resp_row.get(2)?;
            let next_id: u32 = resp_row.get(3)?;
            Ok(DialogueResponse {
                id: resp_id,
                dialogue_id,
                text,
                next_id,
            })
        })?;

        // Collect DialogueResponses if any exist, otherwise return None
        let responses: Option<Vec<DialogueResponse>> = {
            let responses_vec: Vec<DialogueResponse> =
                responses_iter.filter_map(Result::ok).collect();
            if responses_vec.is_empty() {
                None
            } else {
                Some(responses_vec)
            }
        };

        Ok(Dialogue {
            id,
            character_id,
            character_type,
            root,
            root_name,
            text,
            responses,
            has_input,
            input_type,
            next_id,
        })
    })?;

    if let Some(dialogue) = dialogue_iter.next() {
        return Ok(Some(dialogue?));
    }

    Ok(None)
}

pub fn load_by_root_name(conn: &Connection, root_name: &str) -> Result<Option<Dialogue>> {
    let mut stmt = conn.prepare(
    &format!("SELECT id, character_id, character_type, root, root_name, text, has_input, input_type, next_id
                FROM {}
                WHERE root_name = '{}'
                LIMIT 1;",
            DIALOGUE_TABLE,
            root_name),
    )?;

    let mut dialogue_iter = stmt.query_map([], |row| {
        let id: u32 = row.get(0)?;
        let character_id: u32 = row.get(1)?;
        let character_type: CharacterType = row.get(2)?;
        let root: Option<bool> = row.get(3)?;
        let root_name: Option<String> = row.get(4)?;
        let text: String = row.get(5)?;
        let has_input: Option<bool> = row.get(6)?;
        let input_type: Option<String> = row.get(7)?;
        let next_id: Option<u32> = row.get(8)?;

        // Fetch DialogueResponses based on the dialogue_id
        let mut resp_stmt = conn.prepare(
            "SELECT id, dialogue_id, text, next_id FROM dialogue_responses WHERE dialogue_id = ?",
        )?;
        let responses_iter = resp_stmt.query_map([id], |resp_row| {
            let resp_id: u32 = resp_row.get(0)?;
            let dialogue_id: u32 = resp_row.get(1)?;
            let text: String = resp_row.get(2)?;
            let next_id: u32 = resp_row.get(3)?;
            Ok(DialogueResponse {
                id: resp_id,
                dialogue_id,
                text,
                next_id,
            })
        })?;

        // Collect DialogueResponses if any exist, otherwise return None
        let responses: Option<Vec<DialogueResponse>> = {
            let responses_vec: Vec<DialogueResponse> =
                responses_iter.filter_map(Result::ok).collect();
            if responses_vec.is_empty() {
                None
            } else {
                Some(responses_vec)
            }
        };

        Ok(Dialogue {
            id,
            character_id,
            character_type,
            root,
            root_name,
            text,
            responses,
            has_input,
            input_type,
            next_id,
        })
    })?;

    if let Some(dialogue) = dialogue_iter.next() {
        return Ok(Some(dialogue?));
    }

    Ok(None)
}

/// Function to load a YAML dialogue file into a <Vec<Dialogue>
/// This is used exclusively to seed the database.
pub fn load_from_fixture<P: AsRef<Path>>(
    filename: P,
) -> Result<Vec<Dialogue>, Box<dyn std::error::Error>> {
    // Combine the root path with the filename
    let full_path = Path::new(DIALOGUE_SEEDS_ROOT)
        .join(filename)
        .with_extension("yaml");

    let yaml_str = fs::read_to_string(full_path)?;
    let dialogues: Vec<Dialogue> = serde_yaml::from_str(&yaml_str)?;
    Ok(dialogues)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_dialogue() {
        let filename = "test_dialogue";
        let result = load_from_fixture(filename);

        assert!(result.is_ok(), "Failed to load dialogue from YAML.");
        let dialogues = result.unwrap();
        assert!(!dialogues.is_empty(), "Dialogue list is empty.");
    }

    #[test]
    fn test_dialogue_structure() {
        let filename = "test_dialogue";
        let dialogues = load_from_fixture(filename).expect("Failed to load test dialogue");

        assert_eq!(dialogues[0].id, 999999993);
        assert_eq!(
            dialogues[0].character_type,
            CharacterType::NonPlayerCharacter
        );
        assert_eq!(dialogues[0].text, "Hello, traveler!");
        assert!(dialogues[0]
            .responses
            .as_ref()
            .map_or(true, |r| r.is_empty()));
    }

    #[test]
    fn test_dialogue_with_input() {
        let filename = "test_dialogue";
        let dialogues = load_from_fixture(filename).expect("Failed to load test dialogue");

        let input_dialogue = dialogues
            .iter()
            .find(|d| d.id == 999999994)
            .expect("Missing input dialogue");

        assert_eq!(input_dialogue.text, "What is your name?");
        assert_eq!(input_dialogue.root, None);
        assert_eq!(input_dialogue.is_root_node(), false);
        assert_eq!(input_dialogue.has_input, Some(true));
        assert_eq!(input_dialogue.is_input_node(), true);
        assert_eq!(input_dialogue.input_type, Some("string".to_string()));
        assert_eq!(input_dialogue.next_id, Some(999999995));
    }

    #[test]
    fn test_dialogue_with_placeholder() {
        let filename = "test_dialogue";
        let dialogues = load_from_fixture(filename).expect("Failed to load test dialogue");

        let personalized_dialogue = dialogues
            .iter()
            .find(|d| d.id == 999999995)
            .expect("Missing personalized dialogue");

        assert!(
            personalized_dialogue.text.contains("{player_name}"),
            "Placeholder {{player_name}} not found in text"
        );
    }

    #[test]
    fn test_dialogue_with_responses() {
        let filename = "test_dialogue";
        let dialogues = load_from_fixture(filename).expect("Failed to load test dialogue");

        // Find the dialogue with responses (with id 5)
        let response_dialogue = dialogues
            .iter()
            .find(|d| d.id == 999999997)
            .expect("Dialogue with ID 999999997 not found");

        assert!(
            response_dialogue.responses.is_some(),
            "Expected responses to be Some(Vec), but found None."
        );
        let responses = response_dialogue.responses.as_ref().unwrap();
        assert_eq!(
            responses.len(),
            2,
            "Expected 2 responses, but found {}",
            responses.len()
        );
        assert_eq!(
            responses[0].text, "Go left",
            "First response text doesn't match."
        );
        assert_eq!(
            responses[0].next_id, 999999998,
            "First response next_id doesn't match."
        );
        assert_eq!(
            responses[1].text, "Go right",
            "Second response text doesn't match."
        );
        assert_eq!(
            responses[1].next_id, 999999999,
            "Second response next_id doesn't match."
        );
    }

    // Test for the is_end_node method on the Dialogue struct
    #[test]
    fn test_is_end_node() {
        // Case 1: Dialogue with no next_id (end of conversation)
        let dialogue_end = Dialogue {
            id: 1,
            character_id: 1,
            character_type: CharacterType::NonPlayerCharacter,
            root: Some(false),
            root_name: None,
            text: String::from("Welcome to the adventure!"),
            responses: None,
            has_input: Some(true),
            input_type: Some(String::from("string")),
            next_id: None, // No next dialogue, so this is an end node.
        };

        // Assert that this dialogue is the end node
        assert!(
            dialogue_end.is_end_node(),
            "Expected dialogue_end to be an end node"
        );

        // Case 2: Dialogue with a next_id (not the end of the conversation)
        let dialogue_continue = Dialogue {
            id: 2,
            character_id: 1,
            character_type: CharacterType::NonPlayerCharacter,
            root: Some(false),
            root_name: None,
            text: String::from("Do you want to continue?"),
            responses: None,
            has_input: Some(false),
            input_type: None,
            next_id: Some(3), // This dialogue leads to another one, so it's not an end node.
        };

        // Assert that this dialogue is not the end node
        assert!(
            !dialogue_continue.is_end_node(),
            "Expected dialogue_continue to not be an end node"
        );
    }
}

/// args is a &Vec<String> which represents all of the user input collected
/// from the current DialogueRoutine.run() execution. This can be thought of as
/// one execution of a "dialogue tree".
///
/// Alternatively, some variables, like {player.name} require loading data from
/// the database--if these are encountered, they must be located in the whitelisted_data
/// HashMap. `process_dialogue_text` does not actually query the database.
pub fn process_dialogue_text(
    text: &str,
    args: &Vec<String>,
    whitelisted_data: &HashMap<String, String>,
) -> String {
    let mut result = text.to_string();

    // Regular expression to match placeholders like {args[0]}, {player.name}, etc.
    let re = Regex::new(r"\{([a-zA-Z0-9._\[\]]+)\}").unwrap();

    // Iterate through all the placeholders in the text
    for cap in re.captures_iter(text) {
        let placeholder = &cap[1]; // Capture the content inside the curly braces

        // Check if it's an {args[N]} placeholder
        if let Some(index_str) = placeholder.strip_prefix("args[") {
            if let Some(index_end) = index_str.strip_suffix("]") {
                if let Ok(index) = index_end.parse::<usize>() {
                    if index < args.len() {
                        result = result.replace(&cap[0], &args[index]);
                    }
                }
            }
        }
        // Check if it's a {player.name} or other whitelisted attribute
        else if placeholder.starts_with("player.") {
            if let Some(value) = whitelisted_data.get(placeholder) {
                result = result.replace(&cap[0], value);
            }
        }
        // Add additional logic for more placeholder types as needed
    }

    result
}
