use crate::db::save::{ensure_save_directory, get_save_path};
use crate::db::seeds;
use crate::db::{
    BOOK_PAGE_TABLE, BOOK_PASSIVE_TABLE, BOOK_TABLE, DECISION_TABLE, DIALOGUE_RESPONSE_TABLE,
    DIALOGUE_TABLE, GAME_STATE_TABLE, NPC_TABLE, PAGE_ACTION_TABLE, PAGE_PASSIVE_TABLE, PAGE_TABLE,
    PLAYER_TABLE,
};
use rusqlite::{params, Connection, Result};

pub fn get_connection(db_path: Option<&str>) -> Result<Connection> {
    let save_path = get_save_path(db_path);

    // Ensure the save directory exists
    ensure_save_directory().map_err(|_| rusqlite::Error::ExecuteReturnedResults)?;

    let conn = Connection::open(save_path)?;

    if !is_db_setup(&conn).unwrap() {
        setup_db(&conn).unwrap();
        seeds::run(&conn);
    }

    Ok(conn)
}

/// Checks if the database is set up by verifying the existence of the "buses" table.
/// Since all tables are created together in `setup_database`, this is sufficient.
fn is_db_setup(conn: &Connection) -> Result<bool> {
    table_exists(conn, &format!("{}", PLAYER_TABLE))
}

/// Checks if a specific table exists in the database.
fn table_exists(conn: &Connection, table_name: &str) -> Result<bool> {
    let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name=?")?;
    let exists = stmt.exists(params![table_name])?;
    Ok(exists)
}

/// This fn creates the database tables
fn setup_db(conn: &Connection) -> Result<&Connection> {
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
                total_play_time INTEGER NOT NULL DEFAULT 0, -- total play time in seconds
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
        );",
            NPC_TABLE
        ),
        [],
    )?;

    // Dialogue table
    conn.execute(
        &format!(
            "CREATE TABLE IF NOT EXISTS {} (
                id INTEGER PRIMARY KEY,
                character_id INTEGER NOT NULL, -- Polymorphic, references either a player or NPC
                character_type TEXT NOT NULL, -- 'player' or 'non_player_character'
                root BOOLEAN DEFAULT 0, -- Indicates if the dialogue is a root node
                root_name TEXT, -- Optional root name for specific root nodes
                text TEXT NOT NULL,
                has_input BOOLEAN DEFAULT 0,
                input_type TEXT,
                next_id INTEGER,
                CHECK (character_type IN ('player', 'non_player_character'))
            );",
            DIALOGUE_TABLE
        ),
        [],
    )?;

    // Dialogue responses table
    // For storing dialogue responses / choices
    conn.execute(
        &format!(
            "CREATE TABLE IF NOT EXISTS {} (
                id INTEGER PRIMARY KEY,
                dialogue_id INTEGER NOT NULL,
                text TEXT NOT NULL,
                next_id INTEGER NOT NULL, -- Dialogue response must have a next_id
                FOREIGN KEY (dialogue_id) REFERENCES {}(id)
            );",
            DIALOGUE_RESPONSE_TABLE, DIALOGUE_TABLE,
        ),
        [],
    )?;

    // Decisions table
    // For tracking player decisions
    conn.execute(
        &format!(
            "CREATE TABLE IF NOT EXISTS {} (
                id INTEGER PRIMARY KEY,
                player_id INTEGER NOT NULL,  -- Tracks which player made the decision
                dialogue_id INTEGER NOT NULL,  -- Links to the dialogue where the decision was made
                response_id INTEGER, -- Links to the chosen response (nullable for input-based dialogues)
                deliberation_time INTEGER NOT NULL,  -- Time taken to decide (stored as milliseconds)
                FOREIGN KEY (dialogue_id) REFERENCES {}(id),
                FOREIGN KEY (response_id) REFERENCES {}(id)
            );",
            DECISION_TABLE,
            DIALOGUE_TABLE,
            DIALOGUE_RESPONSE_TABLE,
        ),
        [],
    )?;

    // TODO: Add battle models. Turns, BattleLog,... everything...

    Ok(conn)
}
