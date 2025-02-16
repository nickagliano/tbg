use rusqlite::types::FromSql;
use rusqlite::types::{FromSqlResult, ValueRef};
use rusqlite::ToSql;
use std::fmt;

/// Represents the gender of a player in the game.
#[derive(Debug, Clone, PartialEq)]
pub enum Gender {
    Male,
    Female,
    Unspecified,
}

impl ToSql for Gender {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput> {
        let n = match self {
            Gender::Male => 0,
            Gender::Female => 1,
            Gender::Unspecified => 2,
        };
        Ok(n.into())
    }
}

impl FromSql for Gender {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value.as_i64()? {
            0 => Ok(Gender::Male),
            1 => Ok(Gender::Female),
            2 => Ok(Gender::Unspecified),
            _ => Err(rusqlite::types::FromSqlError::InvalidType),
        }
    }
}

impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let gender_str = match self {
            Gender::Male => "Male",
            Gender::Female => "Female",
            Gender::Unspecified => "Unspecified",
        };
        write!(f, "{}", gender_str)
    }
}
