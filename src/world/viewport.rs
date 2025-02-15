use crate::terminal_utils;
use crate::world::map::{tile_to_char, Map, TileType};
use crate::world::navigation::Direction;
use crossterm::terminal::{self, size as terminal_size};

pub struct Viewport {
    pub width: usize,
    pub height: usize,
}

const HEIGHT_ADJUSTMENT: usize = 2;

impl Viewport {
    // FIXME: store the -2 adjustment we have to do to height as a constant
    pub fn new() -> Self {
        let (term_width, term_height) = terminal_size().unwrap_or((80, 24)); // Default to 80x24 if it fails
        let max_size = 2000;

        let width = std::cmp::min(term_width as usize, max_size);
        let height = std::cmp::min(term_height as usize, max_size) - HEIGHT_ADJUSTMENT as usize;

        Viewport { width, height }
    }

    pub fn update_size(&mut self) {
        terminal_utils::clear_console(None);

        if let Ok((w, h)) = terminal_size() {
            let max_size = 2000;
            self.width = w.min(max_size) as usize;
            self.height = (h.min(max_size) - HEIGHT_ADJUSTMENT as u16) as usize;
        }
    }

    pub fn render(&self, map: &Map, player_x: usize, player_y: usize, player_direction: Direction) {
        terminal::disable_raw_mode().unwrap();

        let start_x = player_x.saturating_sub(self.width / 2);
        let start_y = player_y.saturating_sub(self.height / 2);
        let end_x = (start_x + self.width).min(map.get_width());
        let end_y = (start_y + self.height).min(map.get_height());

        for y in start_y..end_y {
            for x in start_x..end_x {
                let tile = if x == player_x && y == player_y {
                    TileType::Player
                } else if x as isize == player_x as isize + player_direction.dx()
                    && y as isize == player_y as isize + player_direction.dy()
                {
                    TileType::PlayerFocus
                } else {
                    map.get_tile(x, y)
                };

                print!("{}", tile_to_char(tile, Some(player_direction)));
            }
            println!();
        }

        terminal::enable_raw_mode().unwrap();
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_create_viewport() {}

    // TODO: Add tests! For everything!
    // - create viewport
    // - render
    // - update size
}
