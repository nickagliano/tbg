use crate::db::PLAYER_TABLE;
use crate::models::game_state::main_arc::MainArc;
use crate::models::player::background::Background;
use crate::models::player::gender::Gender;
use crate::models::player::height::Height;
use chrono::NaiveDateTime;
use rusqlite::Error as RusqliteError;
use rusqlite::{params, Connection, Result};
use sha2::{Digest, Sha256};

// FIXME: Use getters, not pub values
#[derive(Debug, Clone, PartialEq)]
pub struct Player {
    pub id: i32,
    pub name: String,
    pub gender: Gender,
    pub height: Height,
    pub background: Background,
    pub main_arc: MainArc,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Player {
    // Create a new player instance
    pub fn new(name: String, gender: Gender, height: Height) -> Self {
        Player {
            id: 0, // DB will auto-increment this, but we need to pass it
            name,
            gender,
            height,
            background: Background::Undetermined,
            main_arc: MainArc::Undetermined,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }

    pub fn load(conn: &Connection) -> Result<Option<Self>> {
        Self::load_most_recent(conn)
    }

    // Load the most recent player by the updated_at field
    pub fn load_most_recent(conn: &Connection) -> Result<Option<Self>> {
        // FIXME: use player table const
        let mut stmt = conn.prepare(
        &format!("SELECT id, name, gender, height, background, main_arc, created_at, updated_at FROM {} ORDER BY updated_at DESC LIMIT 1", PLAYER_TABLE),
        )?;
        let mut player_iter = stmt.query_map([], |row| {
            let id: i32 = row.get(0)?;
            let name: String = row.get(1)?;
            let gender: Gender = row.get(2)?;
            let height: Height = row.get(3)?;
            let background: Background = row.get(4)?;
            let main_arc: MainArc = row.get(5)?;
            let created_at: NaiveDateTime = row.get(6)?;
            let updated_at: NaiveDateTime = row.get(7)?;
            Ok(Player {
                id,
                name,
                gender,
                height,
                background,
                main_arc,
                created_at,
                updated_at,
            })
        })?;

        if let Some(player) = player_iter.next() {
            return Ok(Some(player?));
        }

        Ok(None)
    }

    // Save the player to the database and return
    pub fn create(&self, conn: &Connection) -> Player {
        conn.execute(
            "INSERT INTO players (name, gender, height, background, main_arc, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            rusqlite::params![
                self.name,
                self.gender,
                self.height,
                self.background,
                self.main_arc,
                self.created_at,
                self.created_at // Default updated at to created at value
            ],
        ).unwrap();

        return Player::load(conn).unwrap().unwrap();
    }

    pub fn update(&self, conn: &Connection) -> Result<()> {
        let rows_updated = conn.execute(
            &format!(
                "UPDATE {} SET gender = ?1, height = ?2, background = ?3, main_arc = ?4, updated_at = ?5 WHERE id = ?6",
                PLAYER_TABLE
            ),
            params![self.gender, self.height, self.background, self.main_arc, chrono::Local::now().naive_local(), self.id],
        )?;

        if rows_updated == 0 {
            // Handle the case where no rows were updated, i.e., no player was found
            return Err(RusqliteError::QueryReturnedNoRows);
        }

        Ok(())
    }

    // TODO: Add height, background, and use this.
    pub fn generate_seed(
        player_name: &str,
        gender: &str,
        height: u8,
        background: &str,
        timestamp: u64,
    ) -> u64 {
        let input = format!(
            "{}:{}:{}:{}:{}",
            player_name, gender, height, background, timestamp
        );
        let hash = Sha256::digest(input.as_bytes());

        // Convert first 8 bytes into a u64 seed
        u64::from_le_bytes(hash[..8].try_into().expect("Hash conversion failed"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_player() {
        let player = Player::new("Test Player".to_string(), Gender::Male, Height::Average);
        assert_eq!(player.name, "Test Player");
        assert_eq!(player.gender, Gender::Male);
    }

    // TODO: Add tests! For everything!
}
