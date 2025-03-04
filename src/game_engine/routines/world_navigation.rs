use crate::game_engine::interface_mode::InterfaceMode;
use crate::models::game_state::game_state::GameState;
use crate::world::map_file_utils::generate_demo_map;
use crate::world::navigation::{prompt_for_action, Direction, NavigationAction};
use crate::world::viewport::Viewport;
use crossterm::terminal;

pub struct WorldNavigationRoutine {
    game_state: GameState,
}

impl WorldNavigationRoutine {
    pub fn new(game_state: GameState) -> Self {
        WorldNavigationRoutine { game_state }
    }

    // Runs the world navigation routine until the player:
    // - Enters a battle
    // - Enters the book builder
    // - Enters the settings?
    // - Exits the game
    //
    // FIXME: Is InterfaceMode the best result type to tell
    //        GameEngine what to do next?
    //
    //        Might want to define a "RoutineResult" or "RoutineAction"
    //        or something that is more clear.
    pub fn run(&mut self) -> InterfaceMode {
        terminal::enable_raw_mode().unwrap();

        // FIXME: Load map based on some conditional
        //        - Might need to add "current_map" to
        //          GameState.
        let map = generate_demo_map(500, 100);

        // Instantiate viewport!
        let mut viewport = Viewport::new();

        loop {
            // Update size dynamically
            viewport.update_size();
            viewport.render(
                &map,
                self.game_state.x,
                self.game_state.y,
                self.game_state.direction,
            );

            // Arbitrary location for development purposes
            // If player moves to the right 1 and down 2, exit and
            // tell GameEngine to launch BookBuilder
            if self.game_state.x == 1 && self.game_state.y == 2 {
                return InterfaceMode::BookBuilder;
            }

            // FIXME: Should this *save* the player? And how often?
            //
            // TODO: Implement world interactions
            //       - Pressing x to interact
            //       - Crossing thresholds, loading new maps
            //       - If the player crosses
            //
            if let Some(action) = prompt_for_action() {
                match action {
                    NavigationAction::Up => {
                        if self.game_state.y > 0 && self.game_state.direction == Direction::Up {
                            self.game_state.y -= 1; // Move up (decrease y) only if we're not at the top
                        }
                        self.game_state.direction = Direction::Up;
                    }
                    NavigationAction::Down => {
                        if self.game_state.y < map.get_height() - 1
                            && self.game_state.direction == Direction::Down
                        {
                            self.game_state.y += 1; // Move down (increase y) only if we're not at the bottom
                        }
                        self.game_state.direction = Direction::Down;
                    }
                    NavigationAction::Left => {
                        if self.game_state.x > 0 && self.game_state.direction == Direction::Left {
                            self.game_state.x -= 1; // Move left (decrease x) only if we're not at the left edge
                        }
                        self.game_state.direction = Direction::Left;
                    }
                    NavigationAction::Right => {
                        if self.game_state.x < map.get_width() - 1
                            && self.game_state.direction == Direction::Right
                        {
                            self.game_state.x += 1; // Move right (increase x) only if we're not at the right edge
                        }
                        self.game_state.direction = Direction::Right;
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
        // FIXME: This isn't a descriptive or very logical return type.
        //        This is another indication that we shouldn't be returning an InterfaceMode
        return InterfaceMode::WorldNavigation;
    }
}
