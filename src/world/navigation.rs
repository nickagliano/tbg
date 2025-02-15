use crossterm::event::{self, Event, KeyCode, MouseEvent};
use rusqlite::types::FromSql;
use rusqlite::types::{FromSqlResult, ValueRef};
use rusqlite::ToSql;
use std::{
    io::{self, Write},
    time::Duration,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn dx(&self) -> isize {
        match self {
            Direction::Left => -1,
            Direction::Right => 1,
            _ => 0,
        }
    }

    pub fn dy(&self) -> isize {
        match self {
            Direction::Up => -1,
            Direction::Down => 1,
            _ => 0,
        }
    }
}

impl ToSql for Direction {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput> {
        let n = match self {
            Direction::Up => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Right => 3,
        };
        Ok(n.into())
    }
}

impl FromSql for Direction {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value.as_i64()? {
            0 => Ok(Direction::Up),
            1 => Ok(Direction::Down),
            2 => Ok(Direction::Left),
            3 => Ok(Direction::Right),
            _ => Err(rusqlite::types::FromSqlError::InvalidType),
        }
    }
}

// FIXME: Might need to separate some of these actions out that aren't very navigation focused,
//        Like exit or resize. Or, rename NavigationAction to something more generic.
#[derive(Debug)]
pub enum NavigationAction {
    Up,
    Down,
    Left,
    Right,
    Inspect,           // For future expansion, e.g., pressing 'x'
    Exit,              // Exit action (when pressing esc)
    Resize,            // When player resizes viewport, we have to handle that as an action
    None,              // Default case for no action
    FocusGained,       // Add FocusGained
    FocusLost,         // Add FocusLost
    Mouse(MouseEvent), // Add Mouse event handling
}

pub fn action_to_direction(action: NavigationAction) -> Option<Direction> {
    match action {
        NavigationAction::Up => Some(Direction::Up),
        NavigationAction::Down => Some(Direction::Down),
        NavigationAction::Left => Some(Direction::Left),
        NavigationAction::Right => Some(Direction::Right),
        _ => None, // No direction for Inspect, Exit, or None
    }
}

pub fn prompt_for_action() -> Option<NavigationAction> {
    let mut stdout = io::stdout();
    // let prompt = "\rMove around the world... ";

    // write!(stdout, "{}", prompt).expect("Failed to print prompt to continue");
    stdout.flush().unwrap();

    loop {
        if let Ok(true) = event::poll(Duration::from_millis(100)) {
            if let Ok(Event::Key(key_event)) = event::read() {
                return match key_event.code {
                    KeyCode::Up => Some(NavigationAction::Up),
                    KeyCode::Down => Some(NavigationAction::Down),
                    KeyCode::Left => Some(NavigationAction::Left),
                    KeyCode::Right => Some(NavigationAction::Right),
                    KeyCode::Char('x') => Some(NavigationAction::Inspect),
                    KeyCode::Esc => Some(NavigationAction::Exit), // Allow exit with Esc
                    _ => break,
                };
            }
        }
    }
    None
}

pub fn get_player_sprite(direction: &NavigationAction) -> String {
    match direction {
        NavigationAction::Up => format!("▲\n █"),
        NavigationAction::Down => format!("█\n▼"),
        NavigationAction::Left => format!("◀ █"),
        NavigationAction::Right => format!("█ ▶"),
        _ => format!("█"), // Default sprite
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_create_viewport() {}

    // TODO: Add tests! For everything!
    // - Direction
    //  - to sql
    //  - from sql
    //  - dx
    //  - dy
    // - NavigationAction
    // - get_player_sprite
}
