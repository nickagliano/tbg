use crate::db::save::{ensure_save_directory, get_save_path};
use crate::db::seeds;
use crate::db::{
    BOOK_PAGE_TABLE, BOOK_PASSIVE_TABLE, BOOK_TABLE, GAME_STATE_TABLE, NPC_TABLE,
    PAGE_ACTION_TABLE, PAGE_PASSIVE_TABLE, PAGE_TABLE, PLAYER_TABLE,
};
use rusqlite::{Connection, Result};

pub fn get_connection(db_path: Option<&str>) -> Result<Connection> {
    let save_path = get_save_path(db_path);

    // Ensure the save directory exists
    ensure_save_directory().map_err(|_| rusqlite::Error::ExecuteReturnedResults)?;

    let conn = Connection::open(save_path)?;

    // FIXME: Don't always execute create tables! Run a check if seed data exists first?
    //
    seeds::run();

    // Player table
    conn.execute(
        &format!(
            "CREATE TABLE IF NOT EXISTS {} (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                gender INTEGER NOT NULL,
                height INTEGER NOT NULL,
                background INTEGER NOT NULL,
                main_arc INTEGER NOT NULL,
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL
        )",
            PLAYER_TABLE
        ),
        [],
    )?;

    // Game state table
    conn.execute(
        &format!(
            "CREATE TABLE IF NOT EXISTS {} (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                player_id INTEGER NOT NULL,
                interface_mode INTEGER NOT NULL DEFAULT 0, -- enum for mode (dialogue, free-roaming, battle)
                current_epic TEXT NOT NULL,
                current_stage TEXT NOT NULL,
                x INTEGER NOT NULL DEFAULT 0, -- X coordinate of the player
                y INTEGER NOT NULL DEFAULT 0, -- Y coordinate of the player
                direction INTEGER NOT NULL DEFAULT 0, -- enum for last direction of the player
                created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (player_id) REFERENCES players(id) ON DELETE CASCADE
        );",
            GAME_STATE_TABLE
        ),
        [],
    )?;

    // Book passives
    conn.execute(
        &format!(
            "CREATE TABLE IF NOT EXISTS {} (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                description TEXT,
                created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
        );",
            BOOK_PASSIVE_TABLE
        ),
        [],
    )?;

    // Book table
    conn.execute(
        &format!(
            "CREATE TABLE IF NOT EXISTS {} (
                id INTEGER PRIMARY KEY,
                player_id INTEGER NOT NULL,
                book_art_type INTEGER NOT NULL,
                book_passive_id INTEGER NOT NULL,
                front_cover TEXT,
                inside_front_cover TEXT,
                inside_back_cover TEXT,
                back_cover TEXT,
                created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (player_id) REFERENCES players(id),
                FOREIGN KEY (book_passive_id) REFERENCES book_passives(id)
        );",
            BOOK_TABLE
        ),
        [],
    )?;

    // PageAction table
    conn.execute(
        &format!(
            "CREATE TABLE IF NOT EXISTS {} (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                description TEXT,
                created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
        );",
            PAGE_ACTION_TABLE
        ),
        [],
    )?;

    // PagePassive table
    conn.execute(
        &format!(
            "CREATE TABLE IF NOT EXISTS {} (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                description TEXT,
                created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
        );",
            PAGE_PASSIVE_TABLE
        ),
        [],
    )?;

    // FIXME: Add NPC, NPCBook, NPCPage

    // Page table
    // FIXME: This is really the PlayerPage table...
    //        - Need to break this into Page (any page that exists in the game),
    //          and PlayerPage (a page in the player's collection)
    conn.execute(
        &format!(
            "CREATE TABLE IF NOT EXISTS {} (
                id INTEGER PRIMARY KEY,
                player_id INTEGER NOT NULL,
                book_art_type INTEGER NOT NULL,
                primary_action_id INTEGER NOT NULL,
                secondary_action_id INTEGER, -- optional
                page_passive_id INTEGER, -- optional
                front TEXT NOT NULL,
                back TEXT NOT NULL,
                FOREIGN KEY (player_id) REFERENCES players(id),
                FOREIGN KEY (player_id) REFERENCES players(id),
                FOREIGN KEY (primary_action_id) REFERENCES page_actions(id),
                FOREIGN KEY (secondary_action_id) REFERENCES page_actions(id),
                FOREIGN KEY (page_passive_id) REFERENCES page_passives(id)
        );",
            PAGE_TABLE
        ),
        [],
    )?;

    // BookPages table (pages assigned to a book at a position)
    conn.execute(
        &format!(
            "CREATE TABLE IF NOT EXISTS {} (
                book_id INTEGER NOT NULL,
                page_id INTEGER NOT NULL,
                position_index INTEGER NOT NULL,
                PRIMARY KEY (book_id, page_id),
                FOREIGN KEY (book_id) REFERENCES books(id),
                FOREIGN KEY (page_id) REFERENCES pages(id),
                UNIQUE (book_id, position_index)
        );",
            BOOK_PAGE_TABLE
        ),
        [],
    )?;

    // (Non-Player) Character table
    conn.execute(
        &format!(
            "CREATE TABLE IF NOT EXISTS {} (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                gender INTEGER NOT NULL,
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL
        )",
            NPC_TABLE
        ),
        [],
    )?;

    // TODO: Add battle models. Turns, BattleLog, DialogueLogs... everything...

    Ok(conn)
}
