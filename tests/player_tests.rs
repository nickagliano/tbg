#[cfg(test)]
use tbg::test_utils;
use tbg::Player;

// Test saving a new player
#[test]
fn test_save_player() {
    let conn = &test_utils::setup_test_db().conn;

    let player = Player::new(
        "Test Player".to_string(),
        tbg::models::player::gender::Gender::Male,
        tbg::models::player::height::Height::Tall,
    );

    let created_player = player.create(&conn);

    assert_eq!(created_player.name, "Test Player");
}

// Test loading a player when there is no player in the DB
#[test]
fn test_load_no_player() {
    let conn = &test_utils::setup_test_db().conn;

    let loaded_player = Player::load(&conn).unwrap();

    assert!(loaded_player.is_none());
}

// Test loading a player when one is saved
#[test]
fn test_load_player() {
    let conn = &test_utils::setup_test_db().conn;

    let player = Player::new(
        "Test Player W Gender".to_string(),
        tbg::models::player::gender::Gender::Female,
        tbg::models::player::height::Height::Short,
    );

    let created_player = player.create(&conn);

    assert_eq!(created_player.name, "Test Player W Gender");
    assert_eq!(
        created_player.gender,
        tbg::models::player::gender::Gender::Female
    );
    assert_eq!(
        created_player.height,
        tbg::models::player::height::Height::Short
    );
}
