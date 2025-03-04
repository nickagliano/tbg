use rusqlite::Connection;

/// Seeds (must be run in certain order!)
///
// TODO:
// - Books
// - Pages
// - Characters
//  - CharacterPage
// - Epics and stages
// - More dialogue
mod books;
mod dialogue;
mod non_player_characters;
mod pages;

pub fn run(conn: &Connection) {
    println!("Running seeds!");

    pages::run(conn);
    non_player_characters::run(conn);
    books::run(conn);

    dialogue::run(conn);
}
