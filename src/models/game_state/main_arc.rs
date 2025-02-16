use rusqlite::types::FromSql;
use rusqlite::types::{FromSqlResult, ValueRef};
use rusqlite::ToSql;

/// MainArc
/// - At some point in the story, the player will make a decision
///   that will entangle them with one of the 4 protagonists of TBG
/// - Once the MainArc value is set, it will never change for the rest of that
///   player's save file
#[derive(Debug, Clone, PartialEq)]
pub enum MainArc {
    Undetermined,
    ThomasMoss,
    RachelRamsayer,
    LaureliDuval,
    GeneParker,
}

impl ToSql for MainArc {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput> {
        let n = match self {
            MainArc::Undetermined => 0,
            MainArc::ThomasMoss => 1,
            MainArc::RachelRamsayer => 2,
            MainArc::LaureliDuval => 3,
            MainArc::GeneParker => 4,
        };
        Ok(n.into())
    }
}

impl FromSql for MainArc {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value.as_i64()? {
            0 => Ok(MainArc::Undetermined),
            1 => Ok(MainArc::ThomasMoss),
            2 => Ok(MainArc::RachelRamsayer),
            3 => Ok(MainArc::LaureliDuval),
            4 => Ok(MainArc::GeneParker),
            _ => Err(rusqlite::types::FromSqlError::InvalidType),
        }
    }
}
