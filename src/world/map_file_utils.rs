use crate::world::map::{Map, TileType};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

// Load a Map from a .map (CSV-style) file
pub fn load_map_from_file(file_path: &str) -> Map {
    let file = File::open(file_path).expect("Failed to open map file");
    let reader = BufReader::new(file);

    let mut tiles = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let row: Vec<TileType> = line
            .split(',')
            .map(|num| match num.parse::<u8>() {
                Ok(n) => TileType::from(n),
                Err(_) => TileType::Empty, // Default fallback
            })
            .collect();

        tiles.push(row);
    }

    Map::new_with_tiles(tiles)
}

// Save a Map to a .map (CSV-style) file
pub fn save_map_to_file(map: &Map, file_path: &str) {
    let file = File::create(file_path).expect("Failed to create map file");
    let mut writer = BufWriter::new(file);

    for row in map.get_tiles_ref() {
        let line: Vec<String> = row
            .iter()
            .map(|tile| (tile.clone() as u8).to_string())
            .collect();
        writeln!(writer, "{}", line.join(",")).expect("Failed to write to file");
    }
}

pub fn generate_small_demo_map() -> Map {
    let mut map = Map::new(10, 10);

    // Add walls around the border
    for x in 0..10 {
        map.set_tile(x, 0, TileType::Wall);
        map.set_tile(x, 9, TileType::Wall);
        map.set_tile(0, x, TileType::Wall);
        map.set_tile(9, x, TileType::Wall);
    }

    // Add some grass
    map.set_tile(1, 1, TileType::Grass);
    map.set_tile(1, 2, TileType::Grass);
    map.set_tile(2, 1, TileType::Grass);
    map.set_tile(2, 2, TileType::Grass);

    // Add water
    map.set_tile(3, 3, TileType::Water);

    map
}

pub fn generate_demo_map(width: usize, height: usize) -> Map {
    let mut tiles = vec![vec![TileType::Empty; width]; height];

    // Walls around edges
    for x in 0..width {
        tiles[0][x] = TileType::Wall;
        tiles[height - 1][x] = TileType::Wall;
    }
    for y in 0..height {
        tiles[y][0] = TileType::Wall;
        tiles[y][width - 1] = TileType::Wall;
    }

    // Add a couple of paths with walls on both sides
    let path_start_y = height / 4;
    let path_end_y = height * 3 / 4;

    for y in path_start_y..path_end_y {
        tiles[y][10] = TileType::Wall;
        tiles[y][11] = TileType::Empty;
        tiles[y][12] = TileType::Empty;
        tiles[y][13] = TileType::Wall;
    }

    let second_path_x = width / 2;
    for x in (second_path_x - 2)..(second_path_x + 2) {
        tiles[path_start_y][x] = TileType::Wall;
        tiles[path_start_y + 1][x] = TileType::Empty;
        tiles[path_start_y + 2][x] = TileType::Empty;
        tiles[path_start_y + 3][x] = TileType::Wall;
    }

    Map::new_with_tiles(tiles)
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_load_map_from_file() {}

    #[test]
    fn test_save_map_to_file() {}

    // TODO: Add tests! For everything!
    // - generate_small_demo_map
    // - generate_demo_map
}
