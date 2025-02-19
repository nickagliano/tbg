use tbg::game_engine::InterfaceMode;
#[cfg(test)]
use tbg::test_utils;
use tbg::{world::navigation::Direction, GameState, Player};

// Test saving a new game state
#[test]
fn test_save_game_state() {
    let conn = &test_utils::setup_test_db().conn;

    // Setup player for gamestate to belong to
    let player = Player::new(
        "Test Player".to_string(),
        tbg::models::player::gender::Gender::Female,
        tbg::models::player::height::Height::Average,
    );

    let loaded_player = player.create(&conn);

    let game_state = GameState::new(loaded_player.id).create(&conn);

    assert_eq!(game_state.player_id, 1);
    assert_eq!(game_state.current_stage, "character_creation");
    assert_eq!(game_state.x, 0);
    assert_eq!(game_state.y, 0);
}

#[test]
fn test_load_no_game_state() {
    let conn = &test_utils::setup_test_db().conn;

    let loaded_game_state = GameState::load_for_player(&conn, 999).unwrap(); // Player ID that doesn't exist
    assert!(loaded_game_state.is_none());
}

#[test]
fn test_update_game_state() {
    let conn = &test_utils::setup_test_db().conn;

    // Setup player for gamestate to belong to
    let player = Player::new(
        "Test Player".to_string(),
        tbg::models::player::gender::Gender::Female,
        tbg::models::player::height::Height::Tall,
    );

    let created_player = player.create(&conn);

    let mut game_state = GameState::new(created_player.id);

    // Save initial game state
    game_state.create(&conn);

    // Modify game state
    game_state.current_stage = "level_2".to_string();
    game_state.x = 42;
    game_state.y = 84;

    // Update in database
    let updated_game_state = game_state.update(&conn);

    // Verify updates persisted
    assert_eq!(updated_game_state.current_stage, "level_2");
    assert_eq!(updated_game_state.x, 42);
    assert_eq!(updated_game_state.y, 84);
}

#[test]
fn test_load_game_state() {
    let conn = &test_utils::setup_test_db().conn;

    let player = Player::new(
        "Test Player".to_string(),
        tbg::models::player::gender::Gender::Female,
        tbg::models::player::height::Height::Tall,
    );

    let loaded_player = player.create(&conn);

    let game_state = GameState {
        player_id: loaded_player.id,
        interface_mode: InterfaceMode::Dialogue,
        current_epic: "level_1".to_string(),
        current_stage: "dungeon_1".to_string(),
        x: 10,
        y: 20,
        direction: Direction::Up,
        created_at: chrono::Local::now().naive_local(),
        updated_at: chrono::Local::now().naive_local(),
    };

    // Save game state
    game_state.create(&conn);

    // Load the saved game state
    let loaded_game_state = GameState::load_for_player(&conn, loaded_player.id).unwrap();
    assert!(loaded_game_state.is_some());
    let loaded_game_state = loaded_game_state.unwrap();
    assert_eq!(loaded_game_state.player_id, loaded_player.id);
    assert_eq!(loaded_game_state.interface_mode, InterfaceMode::Dialogue);
    assert_eq!(loaded_game_state.current_epic, "level_1");
    assert_eq!(loaded_game_state.current_stage, "dungeon_1");
    assert_eq!(loaded_game_state.x, 10);
    assert_eq!(loaded_game_state.y, 20);
}
