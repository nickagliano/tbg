use rusqlite::Connection;

use crate::models::non_player_character::non_player_character::NPC;
use crate::models::player::gender::Gender;

pub fn run(conn: &Connection) {
    println!("Seeding non-player characters...");

    // NOTE: It's important that the Narrator's id is 1 for the dialogue fixtures
    let npc = NPC::new("Narrator".to_string(), Gender::Male);
    npc.create(conn).unwrap();

    // TODO: Seed their books after creation? Narrator doesn't have a book obviously
}
