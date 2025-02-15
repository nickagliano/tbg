use crate::world::navigation::Direction;

#[derive(Clone, Copy, Debug)]
pub enum TileType {
    Player,      // 0 - The player's body position
    PlayerFocus, // 1 - The player's "focus" position
    Empty,       // 2 - Empty space
    Wall,        // 3 - Wall or obstacle
    Water,       // 4 - Water (if applicable)
    Grass,       // 5 - Grass or floor
}

pub fn tile_to_char(tile: TileType, player_direction: Option<Direction>) -> char {
    match tile {
        TileType::Player => '█', // Player position
        TileType::PlayerFocus => match player_direction {
            Some(Direction::Up) => '▲',
            Some(Direction::Down) => '▼',
            Some(Direction::Left) => '◀',
            Some(Direction::Right) => '▶',
            _ => '@', // Default or unknown direction
        },
        TileType::Empty => ' ', // Empty space
        TileType::Wall => '#',  // Wall
        TileType::Water => '~', // Water
        TileType::Grass => '.', // Grass or floor
    }
}

impl From<u8> for TileType {
    fn from(n: u8) -> Self {
        match n {
            0 => TileType::Player,
            1 => TileType::PlayerFocus,
            2 => TileType::Empty,
            3 => TileType::Wall,
            4 => TileType::Water,
            5 => TileType::Grass,
            _ => TileType::Empty, // Default fallback
        }
    }
}

/// A map is a 2d grid
pub struct Map {
    width: usize,
    height: usize,
    tiles: Vec<Vec<TileType>>, // 2D grid of tiles
}

impl Map {
    // Creates empty map
    // TODO: Remove this? Sort of useless. Or keep and raise error to
    //       raise usage to developer.
    pub fn new(width: usize, height: usize) -> Self {
        let tiles = vec![vec![TileType::Empty; width]; height];
        Map {
            width,
            height,
            tiles,
        }
    }

    // Creates a map with existing tile data
    pub fn new_with_tiles(tiles: Vec<Vec<TileType>>) -> Self {
        let height = tiles.len();
        let width = if height > 0 { tiles[0].len() } else { 0 };
        Map {
            width,
            height,
            tiles,
        }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_tiles_ref(&self) -> &Vec<Vec<TileType>> {
        &self.tiles
    }

    // Function to set a tile at a given position
    pub fn set_tile(&mut self, x: usize, y: usize, tile: TileType) {
        if x < self.width && y < self.height {
            self.tiles[y][x] = tile;
        }
    }

    // Function to get a tile at a given position
    pub fn get_tile(&self, x: usize, y: usize) -> TileType {
        if x < self.width && y < self.height {
            self.tiles[y][x]
        } else {
            TileType::Empty // Return Empty if out of bounds
        }
    }

    pub fn print(&self, player_x: usize, player_y: usize, player_direction: Direction) {
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, &tile) in row.iter().enumerate() {
                let tile = if x == player_x && y == player_y {
                    TileType::Player
                } else if x as isize == player_x as isize + player_direction.dx()
                    && y as isize == player_y as isize + player_direction.dy()
                {
                    TileType::PlayerFocus
                } else {
                    tile
                };

                print!("{}", tile_to_char(tile, Some(player_direction)));
            }
            println!(); // New line after each row
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_create_map() {}

    // TODO: Add tests! For everything!
    // - Tiles
    //  - tile_to_char
    //  - tile From<u8>
    // - Map
    //  - new_with_tiles
    //  - print
    //  - get_tile
    //  - set_tile
    //  - get_tiles_ref
    //  - get_width
    //  - get_height
}
