/// Seeds (must be run in certain order!)
///
// TODO:
// - Books
// - Pages
// - Characters
//  - CharacterPage
// - Epics and stages
mod books;
mod characters;
mod pages;

pub fn run() {
    println!("Running seeds!");
    pages::run();
    characters::run();
    books::run();
}
