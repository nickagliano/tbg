use crate::db::PLAYER_TABLE;
use chrono::NaiveDateTime;
use rusqlite::Error as RusqliteError;
use rusqlite::{params, Connection, Result};
use sha2::{Digest, Sha256};
use std::fmt;
use std::str::FromStr; // Alias for rusqlite::Error
use uuid::Uuid; // For handling timestamps

#[derive(Debug, Clone, PartialEq)]
pub struct Player {
    pub id: i32,
    pub uuid: Uuid,
    pub name: String,
    pub gender: Gender,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Represents the gender of a player in the game.
#[derive(Debug, Clone, PartialEq)]
pub enum Gender {
    Male,
    Female,
    Unspecified,
}

impl Gender {
    /// Converts the `Gender` enum to a string representation for storing in a database.
    ///
    /// # Returns
    /// A string slice representing the gender as stored in the database.
    ///
    /// # Example
    /// ```
    /// use tbg::Gender;
    ///
    /// let gender = Gender::Male;
    /// assert_eq!(gender.to_db_string(), "male");
    /// ```
    pub fn to_db_string(&self) -> &str {
        match self {
            Gender::Male => "male",
            Gender::Female => "female",
            Gender::Unspecified => "unspecified",
        }
    }

    /// Converts a user-facing string to a `Gender` enum.
    ///
    /// # Arguments
    /// * `s` - A string slice representing the gender.
    ///
    /// # Errors
    /// Returns `GenderParseError` if the string does not match a valid gender.
    ///
    /// # Example
    /// ```
    /// use tbg::Gender;
    ///
    /// let gender = Gender::from_string("Male");
    /// assert_eq!(gender.to_db_string(), "male");
    /// assert_eq!(Gender::from_string("ddnvalid"), Gender::Unspecified);
    /// ```
    pub fn from_string(s: &str) -> Self {
        match s {
            "Male" => Gender::Male,
            "Female" => Gender::Female,
            "Choose not to specify" => Gender::Unspecified,
            _ => Gender::Unspecified, // FIXME: I'd rather panic here.
        }
    }

    /// Converts a database string to a `Gender` enum.
    ///
    /// # Arguments
    /// * `s` - A string slice representing the gender in the database.
    ///
    /// # Errors
    /// Returns `GenderParseError` if the string is not a valid database gender value.
    ///
    /// # Example
    /// ```
    /// use tbg::Gender;
    ///
    /// let gender = Gender::from_db_string("male");
    /// assert_eq!(gender, Gender::Male);
    ///
    /// assert_eq!(Gender::from_string("Invalid"), Gender::Unspecified);
    /// ```
    pub fn from_db_string(s: &str) -> Self {
        match s {
            "male" => Gender::Male,
            "female" => Gender::Female,
            "unspecified" => Gender::Unspecified,
            _ => Gender::Unspecified, // FIXME: I'd rather panic here.
        }
    }
}

impl FromStr for Gender {
    type Err = GenderParseError;

    /// Parses a string into a `Gender` enum.
    ///
    /// # Arguments
    /// * `s` - A string slice representing the gender.
    ///
    /// # Errors
    /// Returns `GenderParseError` if the string is invalid.
    ///
    /// # Example
    /// ```
    /// use tbg::Gender;
    /// use std::str::FromStr;
    ///
    /// let gender = Gender::from_str("male");
    /// assert_eq!(gender.unwrap(), Gender::Male);
    ///
    /// assert!(Gender::from_str("invalid").is_err());
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "male" => Ok(Gender::Male),
            "female" => Ok(Gender::Female),
            "unspecified" => Ok(Gender::Unspecified),
            e => Err(GenderParseError::InvalidGender(e.to_string())),
        }
    }
}

/// Represents an error when parsing a gender string.
#[derive(Debug)]
pub enum GenderParseError {
    /// Represents an invalid gender input.
    InvalidGender(String),
}

impl std::error::Error for GenderParseError {}

impl fmt::Display for GenderParseError {
    /// Formats the `Gender` enum as a human-readable string.
    ///
    /// # Example
    /// ```
    /// use tbg::Gender;
    ///
    /// let gender = Gender::Female;
    /// assert_eq!(gender.to_string(), "Female");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid gender: {:?}", self)
    }
}

impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let gender_str = match *self {
            Gender::Male => "Male",
            Gender::Female => "Female",
            Gender::Unspecified => "Choose not to specify",
        };
        write!(f, "{}", gender_str)
    }
}

impl Player {
    // Create a new player instance
    pub fn new(name: String, gender: Gender) -> Self {
        Player {
            id: 0, // DB will auto-increment this
            uuid: Uuid::new_v4(),
            name,
            gender,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }

    pub fn load(conn: &Connection) -> Result<Option<Self>> {
        Self::load_most_recent(conn)
    }

    // Load the most recent player by the updated_at field
    pub fn load_most_recent(conn: &Connection) -> Result<Option<Self>> {
        let mut stmt = conn.prepare(
            "SELECT id, uuid, name, gender, created_at, updated_at FROM players ORDER BY updated_at DESC LIMIT 1",
        )?;
        let mut player_iter = stmt.query_map([], |row| {
            let id: i32 = row.get(0)?;
            let uuid: String = row.get(1)?;
            let name: String = row.get(2)?;
            let gender: String = row.get(3)?;
            let created_at: NaiveDateTime = row.get(4)?;
            let updated_at: NaiveDateTime = row.get(5)?;
            Ok(Player {
                id,
                uuid: Uuid::parse_str(&uuid).unwrap(),
                name,
                gender: Gender::from_db_string(&gender),
                created_at,
                updated_at,
            })
        })?;

        if let Some(player) = player_iter.next() {
            return Ok(Some(player?));
        }

        Ok(None)
    }

    // Load a player by UUID
    pub fn load_by_uuid(conn: &Connection, player_uuid: &Uuid) -> Result<Option<Self>> {
        let mut stmt = conn.prepare(
            "SELECT id, uuid, name, gender, created_at, updated_at FROM players WHERE uuid = ?1",
        )?;
        let player_iter = stmt.query_map([player_uuid.to_string()], |row| {
            let id: i32 = row.get(0)?;
            let uuid: String = row.get(1)?;
            let name: String = row.get(2)?;
            let gender: String = row.get(3)?;
            let created_at: NaiveDateTime = row.get(4)?;
            let updated_at: NaiveDateTime = row.get(5)?;
            Ok(Player {
                id,
                uuid: Uuid::parse_str(&uuid).unwrap(),
                name,
                gender: Gender::from_db_string(&gender), // Assuming you have a method to convert from DB string to Gender
                created_at,
                updated_at,
            })
        })?;

        for player in player_iter {
            return Ok(Some(player?));
        }

        Ok(None)
    }

    // Save the player to the database
    pub fn create(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            "INSERT INTO players (uuid, name, gender, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![
                self.uuid.to_string(),
                self.name,
                self.gender.to_db_string(),
                self.created_at,
                self.created_at // Default to created at
            ],
        )?;
        Ok(())
    }

    pub fn update(&self, conn: &Connection) -> Result<()> {
        let rows_updated = conn.execute(
            &format!(
                "UPDATE {} SET gender = ?1, updated_at = ?2 WHERE name = ?3",
                PLAYER_TABLE
            ),
            params![
                self.gender.to_db_string(),
                chrono::Local::now().naive_local(),
                self.name
            ],
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
        let player = Player::new("Test Player".to_string(), Gender::Male);
        assert_eq!(player.name, "Test Player");
        assert_eq!(player.gender.to_db_string(), "male");
    }

    // TODO: Add tests! For everything!
}
