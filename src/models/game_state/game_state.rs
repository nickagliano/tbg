use crate::db::GAME_STATE_TABLE;
use crate::game_engine::interface_mode::InterfaceMode;
use crate::world::navigation::Direction;
use chrono::NaiveDateTime;
use rusqlite::types::{FromSql, FromSqlResult, ToSqlOutput, ValueRef};
use rusqlite::ToSql;
use rusqlite::{Connection, Result};
use std::time::Duration;

#[derive(Debug, Clone, Copy)]
pub struct SqlDuration(pub Duration); // Wrapper type (so we can implement ToSql and FromSql)

impl ToSql for SqlDuration {
    fn to_sql(&self) -> Result<ToSqlOutput> {
        Ok(ToSqlOutput::from(self.as_secs() as i64)) // Store as integer (seconds)
    }
}

impl FromSql for SqlDuration {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        let seconds = value.as_i64()?; // Read integer (seconds)
        Ok(SqlDuration(Duration::from_secs(seconds as u64)))
    }
}

impl SqlDuration {
    pub fn as_secs(&self) -> u64 {
        self.0.as_secs() // Forward to inner Duration
    }

    pub fn from_secs(secs: u64) -> Self {
        SqlDuration(Duration::from_secs(secs))
    }
}

// FIXME: There's an argument that some of the things being stored shouldn't be
//        persisted in the database. Or, player x, y, and direction should be updated in memory,
//        and only on some event or schedule dumped into the DB.
#[derive(Debug, Clone)]
pub struct GameState {
    pub player_id: i32,                // Foreign key to the player
    pub interface_mode: InterfaceMode, // Track the state of UI / interface, its mode (Dialogue, WorldNavigation, Battle)
    pub current_epic: String,          // Represents a larger story arc of the game
    pub current_stage: String,         // Represents the current stage of the epic
    pub x: usize,                      // Player's X coordinate
    pub y: usize,                      // Player's Y coordinate
    pub direction: Direction,          // Track last movement
    pub total_play_time: SqlDuration,  // Total playtime // TODO: Implement Duration to_sql
    pub created_at: NaiveDateTime,     // Timestamp when the game state was created
    pub updated_at: NaiveDateTime,     // Timestamp when the game state was last updated
}

impl GameState {
    pub fn new(player_id: i32) -> Self {
        GameState {
            interface_mode: InterfaceMode::Dialogue,
            current_epic: "intro".to_string(),
            current_stage: "character_creation".to_string(),
            player_id,
            x: 0,
            y: 0,
            direction: Direction::Up,
            total_play_time: SqlDuration(Duration::new(0, 0)),
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
            // session_start: Instant::now(),
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "current_epic:{}, current_stage:{} -- x:{}, y:{}",
            self.current_epic, self.current_stage, self.x, self.y
        )
    }

    pub fn create(&self, conn: &Connection) -> GameState {
        conn.execute(
            &format!(
                "INSERT INTO {} (interface_mode, current_epic, current_stage, player_id, x, y, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                GAME_STATE_TABLE
            ),
            rusqlite::params![self.interface_mode, self.current_epic, self.current_stage, self.player_id, self.x, self.y, self.created_at, self.created_at],
        ).unwrap();

        return GameState::load_for_player(&conn, self.player_id)
            .unwrap()
            .unwrap();
    }

    pub fn update(&self, conn: &Connection) -> GameState {
        conn.execute(
            &format!(
                "UPDATE {}
                    SET current_epic = ?1, current_stage = ?2, x = ?3, y = ?4, updated_at = ?5
                    WHERE player_id = ?6",
                GAME_STATE_TABLE
            ),
            rusqlite::params![
                self.current_epic,
                self.current_stage,
                self.x,
                self.y,
                chrono::Local::now().naive_local(),
                self.player_id
            ],
        )
        .unwrap();

        return GameState::load_for_player(&conn, self.player_id)
            .unwrap()
            .unwrap();
    }

    pub fn load_for_player(conn: &Connection, player_id: i32) -> Result<Option<Self>> {
        let mut stmt = conn.prepare(&format!(
            "SELECT interface_mode, current_epic, current_stage, x, y, direction, total_play_time, created_at, updated_at FROM {} WHERE player_id = ?1",
            GAME_STATE_TABLE
        ))?;

        let mut game_state_iter = stmt.query_map([player_id], |row| {
            Ok(GameState {
                interface_mode: row.get(0)?,
                current_epic: row.get(1)?,
                current_stage: row.get(2)?,
                player_id,
                x: row.get(3)?,
                y: row.get(4)?,
                direction: row.get(5)?,
                total_play_time: row.get(6)?,
                created_at: row.get(7)?,
                updated_at: row.get(8)?,
            })
        })?;

        if let Some(game_state) = game_state_iter.next() {
            return Ok(Some(game_state?));
        }

        Ok(None)
    }
}
