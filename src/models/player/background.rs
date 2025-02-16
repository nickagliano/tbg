use rusqlite::types::FromSql;
use rusqlite::types::{FromSqlResult, ValueRef};
use rusqlite::ToSql;

/// Background
/// - Character creation ends with the user choosing their background
#[derive(Debug, Clone, PartialEq)]
pub enum Background {
    Undetermined,
    Farmer,
    Solider,
    Trader,
    Politician,
    Craftsman,
}

impl ToSql for Background {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput> {
        let n = match self {
            Background::Undetermined => 0,
            Background::Farmer => 1,
            Background::Solider => 2,
            Background::Trader => 3,
            Background::Politician => 4,
            Background::Craftsman => 4,
        };
        Ok(n.into())
    }
}

impl FromSql for Background {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value.as_i64()? {
            0 => Ok(Background::Undetermined),
            1 => Ok(Background::Farmer),
            2 => Ok(Background::Solider),
            3 => Ok(Background::Trader),
            4 => Ok(Background::Politician),
            5 => Ok(Background::Craftsman),
            _ => Err(rusqlite::types::FromSqlError::InvalidType),
        }
    }
}
