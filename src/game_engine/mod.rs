pub mod game_engine;
pub mod game_event;
use rusqlite::types::FromSql;
use rusqlite::types::{FromSqlResult, ValueRef};
use rusqlite::ToSql;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InterfaceMode {
    Dialogue,
    WorldNavigation,
    Battle,
}

impl ToSql for InterfaceMode {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput> {
        let n = match self {
            InterfaceMode::Dialogue => 0,
            InterfaceMode::WorldNavigation => 1,
            InterfaceMode::Battle => 2,
        };
        Ok(n.into())
    }
}

impl FromSql for InterfaceMode {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value.as_i64()? {
            0 => Ok(InterfaceMode::Dialogue),
            1 => Ok(InterfaceMode::WorldNavigation),
            2 => Ok(InterfaceMode::Battle),
            _ => Err(rusqlite::types::FromSqlError::InvalidType),
        }
    }
}
