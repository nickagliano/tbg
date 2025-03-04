use rusqlite::Connection;

pub fn run(_conn: &Connection) {
    println!("Seeding books...");

    // FIXME: Is seeding books needed..?
    //        - Maybe seeding the "starter" books?
    //        - Books == PlayerBook ?
}
