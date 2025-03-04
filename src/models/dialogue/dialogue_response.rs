use crate::db::DIALOGUE_RESPONSE_TABLE;
use rusqlite::params;
use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};

// Represents a possible response to a dialogue, where the player can choose one option
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DialogueResponse {
    pub id: u32,
    pub dialogue_id: u32,
    pub text: String,
    pub next_id: u32,
    // FIXME: Add created at, updated at
}

impl DialogueResponse {
    // Constructor for DialogueResponse
    pub fn new(dialogue_id: u32, text: String, next_id: u32) -> Self {
        DialogueResponse {
            id: 0, // This will be set by the database when it's inserted
            dialogue_id,
            text,
            next_id,
        }
    }

    // Create method for DialogueResponse
    pub fn create(&self, conn: &Connection) -> Result<u32, rusqlite::Error> {
        conn.execute(
            &format!(
                "INSERT INTO {} (dialogue_id, text, next_id) VALUES (?1, ?2, ?3)",
                DIALOGUE_RESPONSE_TABLE
            ),
            params![self.dialogue_id, self.text, self.next_id],
        )?;

        // Return the id of the new response (usually returned by the database after insertion)
        let last_inserted_id: u32 = conn.last_insert_rowid() as u32;
        Ok(last_inserted_id)
    }
}
