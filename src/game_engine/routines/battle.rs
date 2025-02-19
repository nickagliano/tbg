use crate::models::non_player_character::non_player_character::NPC;
use crate::models::player::player::Player;
use crossterm::terminal;

pub struct BattleRoutine {
    player: Player,
    enemy: NPC,
}

impl BattleRoutine {
    /// Creates a new `Battle` instance with a given `Player` and `NPC` (enemy).
    ///
    /// # Arguments
    /// * `player` - The player character involved in the battle.
    /// * `enemy` - The non-player character (NPC) who will be the opponent.
    ///
    /// # Returns
    /// A new `Battle` instance.
    pub fn new(player: Player, enemy: NPC) -> Self {
        BattleRoutine { player, enemy }
    }

    /// Runs the battle loop until a winner is determined.
    pub fn run(&mut self) {
        terminal::enable_raw_mode().unwrap();

        loop {
            println!("Do something");
            println!("{:?}", self.player);
            println!("{:?}", self.enemy);
            break;
        }

        terminal::disable_raw_mode().unwrap();
    }
}
