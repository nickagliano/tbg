/// Non-Player Character
use crate::db::NPC_TABLE;
use crate::models::player::gender::Gender;
use chrono::NaiveDateTime;
use rusqlite::{Connection, Result};

// FIXME: Use getters, not pub values
#[derive(Debug, Clone, PartialEq)]
pub struct NPC {
    pub id: i32,
    pub name: String,
    pub gender: Gender,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl NPC {
    // Create a new NPC instance
    pub fn new(name: String, gender: Gender) -> Self {
        NPC {
            id: 0, // DB will auto-increment this, but we need to pass it
            name,
            gender,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }

    pub fn load_by_id(conn: &Connection, id: i32) -> Result<Option<Self>> {
        let mut stmt = conn.prepare(
            &format!("SELECT id, name, gender, created_at, updated_at FROM {} WHERE id = {} ORDER BY updated_at DESC LIMIT 1", NPC_TABLE, id),
        )?;
        let mut npc_iter = stmt.query_map([], |row| {
            let id: i32 = row.get(0)?;
            let name: String = row.get(1)?;
            let gender: Gender = row.get(2)?;
            let created_at: NaiveDateTime = row.get(3)?;
            let updated_at: NaiveDateTime = row.get(4)?;
            Ok(NPC {
                id,
                name,
                gender,
                created_at,
                updated_at,
            })
        })?;

        if let Some(npc) = npc_iter.next() {
            return Ok(Some(npc?));
        }

        Ok(None)
    }

    // Save the NPC to the database
    pub fn create(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            &format!(
                "INSERT INTO {} (name, gender, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
                NPC_TABLE
            ),
            rusqlite::params![
                self.name,
                self.gender,
                self.created_at,
                self.created_at // Default updated at to created at value
            ],
        )?;
        Ok(())
    }

    // NOTE: There is notably no update fn for NPC.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_npc() {
        let npc = NPC::new("Test NPC".to_string(), Gender::Male);
        assert_eq!(npc.name, "Test NPC");
        assert_eq!(npc.gender, Gender::Male);
    }

    // TODO: Add tests! For everything!
}
