use crate::models::dialogue::dialogue::{self, Dialogue};
use rusqlite::Connection;
use std::collections::HashMap;

pub fn run(conn: &Connection) {
    println!("Seeding dialogue from fixtures...");

    let fixtures = ["character_creation", "first_quest"];
    let mut dialogue_id_map: HashMap<u32, u32> = HashMap::new();

    // First pass: Create all dialogues
    for fixture_name in &fixtures {
        println!("Creating dialogue from fixtures at {:?}", fixture_name);
        let dialogue_tree: Vec<Dialogue> = dialogue::load_from_fixture(fixture_name).unwrap();

        for dialogue in &dialogue_tree {
            let dialogue_id = dialogue.create(&conn);
            dialogue_id_map.insert(dialogue.id, dialogue_id);
            println!("Created Dialogue with ID: {}", dialogue_id);
        }
    }

    // Second pass: Create all responses (now that all dialogue IDs exist, otherwise fk errors)
    for fixture_name in &fixtures {
        println!("Creating responses from fixtures at {:?}", fixture_name);

        let mut dialogue_tree: Vec<Dialogue> = dialogue::load_from_fixture(fixture_name).unwrap();

        for dialogue in &mut dialogue_tree {
            if let Some(responses) = dialogue.get_responses_mut() {
                for response in responses.iter_mut() {
                    if let Some(&mapped_next_id) = dialogue_id_map.get(&response.next_id) {
                        response.next_id = mapped_next_id;
                    } else {
                        println!(
                            "Warning: Could not find dialogue with ID: {}",
                            response.next_id
                        );
                    }
                    response.create(&conn).unwrap();
                }
            }
        }
    }
}
