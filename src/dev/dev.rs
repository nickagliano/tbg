use crate::world::map_file_utils::generate_demo_map;
use crate::world::navigation::{prompt_for_action, Direction, NavigationAction};
use crate::world::viewport::Viewport;
use crossterm::terminal;

/// WIP implementation of world navigation!
pub fn run() {
    // Enable raw mode
    terminal::enable_raw_mode().unwrap();

    // (w, h)
    let map = generate_demo_map(500, 100);

    // FIXME: Grab this from the Player struct, from the db
    let mut player_x = 5; // Initial player position
    let mut player_y = 5; // Initial player position
    let mut player_direction = Direction::Up; // default, Player is facing up

    // Instantiate viewport!
    let mut viewport = Viewport::new();

    loop {
        // Update size dynamically
        viewport.update_size();
        viewport.render(&map, player_x, player_y, player_direction);

        if let Some(action) = prompt_for_action() {
            match action {
                NavigationAction::Up => {
                    if player_y > 0 && player_direction == Direction::Up {
                        player_y -= 1; // Move up (decrease y) only if we're not at the top
                    }
                    player_direction = Direction::Up;
                }
                NavigationAction::Down => {
                    if player_y < map.get_height() - 1 && player_direction == Direction::Down {
                        player_y += 1; // Move down (increase y) only if we're not at the bottom
                    }
                    player_direction = Direction::Down;
                }
                NavigationAction::Left => {
                    if player_x > 0 && player_direction == Direction::Left {
                        player_x -= 1; // Move left (decrease x) only if we're not at the left edge
                    }
                    player_direction = Direction::Left;
                }
                NavigationAction::Right => {
                    if player_x < map.get_width() - 1 && player_direction == Direction::Right {
                        player_x += 1; // Move right (increase x) only if we're not at the right edge
                    }
                    player_direction = Direction::Right;
                }
                NavigationAction::Resize => {
                    println!("Updating size!!");
                    viewport.update_size();
                }
                _ => break,
            }
        }
    }

    terminal::disable_raw_mode().unwrap();
}
