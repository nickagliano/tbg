// mods
pub mod connection;
pub mod save;
pub mod seeds;

// DB name
pub const DEFAULT_DB: &str = "save_file.db";

// Table names
pub const BOOK_PAGE_TABLE: &str = "book_pages";
pub const BOOK_PASSIVE_TABLE: &str = "book_passives";
pub const BOOK_TABLE: &str = "books";
pub const DECISION_TABLE: &str = "decisions";
pub const DIALOGUE_TABLE: &str = "dialogue";
pub const DIALOGUE_RESPONSE_TABLE: &str = "dialogue_responses";
pub const GAME_STATE_TABLE: &str = "game_states";
pub const NPC_TABLE: &str = "non_player_characters";
pub const PAGE_ACTION_TABLE: &str = "page_actions";
pub const PAGE_PASSIVE_TABLE: &str = "page_passives";
pub const PAGE_TABLE: &str = "pages";
pub const PLAYER_TABLE: &str = "players";
pub const SAVE_DIR: &str = "saves";
