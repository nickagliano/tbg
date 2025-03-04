use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ValueRef};
use rusqlite::ToSql;
use serde::de::Error;
use serde::{self, Deserialize, Deserializer, Serialize};

// Define the CharacterType enum
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum CharacterType {
    Player,
    NonPlayerCharacter, // sometimes abbreviated to NPC
}

impl FromSql for CharacterType {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let s = value.as_str()?;
        match s {
            "player" => Ok(CharacterType::Player),
            "non_player_character" => Ok(CharacterType::NonPlayerCharacter),
            _ => Err(FromSqlError::OutOfRange(-1)),
        }
    }
}

impl ToSql for CharacterType {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput> {
        let n = match self {
            CharacterType::Player => "player",
            CharacterType::NonPlayerCharacter => "non_player_character",
        };
        Ok(n.into())
    }
}

// Implement ToString for CharacterType to easily convert it to a string
impl ToString for CharacterType {
    fn to_string(&self) -> String {
        match self {
            CharacterType::Player => String::from("player"),
            CharacterType::NonPlayerCharacter => String::from("non_player_character"),
        }
    }
}

// You can also implement a method to parse the string back into an enum
impl CharacterType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "player" => Some(CharacterType::Player),
            "non_player_character" => Some(CharacterType::NonPlayerCharacter),
            _ => None,
        }
    }
}

// Implement custom deserialization for CharacterType
pub fn deserialize_character_type<'de, D>(deserializer: D) -> Result<CharacterType, D::Error>
where
    D: Deserializer<'de>,
{
    // Use a string as input
    let s = String::deserialize(deserializer)?;

    // Convert the string to a CharacterType, returning an error if it fails
    CharacterType::from_str(&s).ok_or_else(|| D::Error::unknown_variant(&s, &["player", "npc"]))
}
