use rusqlite::types::FromSql;
use rusqlite::types::{FromSqlResult, ValueRef};
use rusqlite::ToSql;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Height {
    VeryShort,
    Short,
    Average,
    Tall,
    VeryTall,
}

impl ToSql for Height {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput> {
        let n = match self {
            Height::VeryShort => 0,
            Height::Short => 1,
            Height::Average => 2,
            Height::Tall => 3,
            Height::VeryTall => 4,
        };
        Ok(n.into())
    }
}

impl FromSql for Height {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value.as_i64()? {
            0 => Ok(Height::VeryShort),
            1 => Ok(Height::Short),
            2 => Ok(Height::Average),
            3 => Ok(Height::Tall),
            4 => Ok(Height::VeryTall),
            _ => Err(rusqlite::types::FromSqlError::InvalidType),
        }
    }
}

impl fmt::Display for Height {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let height_str = match self {
            Height::VeryShort => "Very Short",
            Height::Short => "Short",
            Height::Average => "Average",
            Height::Tall => "Tall",
            Height::VeryTall => "Very Tall",
        };
        write!(f, "{}", height_str)
    }
}
