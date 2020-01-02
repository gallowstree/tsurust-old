use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::result::Result;
use arrayvec::ArrayVec;
use std::collections::HashMap;

pub const TILES_PER_ROW: usize = 6;
pub const SPAWN_COUNT : usize = TILES_PER_ROW * 2 * 4;

/* A tile has 8 path indices, when rotation = 0:
# 5 ## 4 #
6        3
#        #
7        2
# 0 ## 1 #
*/
pub type PathIndex = u8;
pub type Path = (PathIndex, PathIndex); // (from, to)
pub type Position = (usize, usize, PathIndex); // (row, column, path_index)
//TODO perhaps use usize for all? or u8 for all?

pub struct Tsurust {
    deck: Deck,
    pub board: Board,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum PlayerColor {
    WHITE, RED, YELLOW,
    BLUE, GRAY, ORANGE,
    GREEN, BLACK
}

pub struct Board {
    pub grid: [[Option<Tile> ; TILES_PER_ROW] ; TILES_PER_ROW],
    pub spawns: ArrayVec<[Position; SPAWN_COUNT]>,
    pub stones: HashMap<PlayerColor, Stone>
}

#[derive(Debug)]
pub struct Deck {
    tiles: Vec<Tile>,
}

#[derive(Debug, Copy, Clone)]
pub enum Tile {
    DragonTile,
    PathTile { paths: [Path; 4] },
}

impl Tile {
    fn get_other_end(&self, from_index: PathIndex) -> PathIndex {
        if let Tile::PathTile {paths} = self {
            let &(a, b) = paths
                .iter()
                .find(|(from, to)| *from == from_index as u8|| *to == from_index as u8)
                .expect("malformed tile");

            return if a != from_index {a} else {b}
        }

        panic!("trying to move across dragon tile")
    }

    fn rotate(&self) -> Tile {
        *self //TODO: me
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Rotation {
    _0,
    _90,
    _180,
    _270,
}

#[derive(Debug, Copy, Clone)]
pub struct Stone {
    pub color:PlayerColor,
    pub position: Position
}

impl Stone {
    fn with_position(&self, position: Position) -> Stone {
        Stone {color: self.color, position}
    }

    fn is_at_coords(&self, row: usize, col: usize) -> bool {
        let (curr_row, curr_col, _) = self.position;
        curr_row == row && curr_col == col
    }
}

impl Default for Board {
    fn default() -> Board {
        Board {
            grid: [ [None; TILES_PER_ROW] ; TILES_PER_ROW],
            spawns: make_spawns(),
            stones: HashMap::new()
        }
    }
}

impl Board {
    pub fn with_players(players: HashMap<PlayerColor, Stone>) -> Board {
        Board {
            grid: [ [None; TILES_PER_ROW] ; TILES_PER_ROW],
            spawns: make_spawns(),
            stones: players
        }
    }

    pub fn place_tile(&mut self, row: usize, col: usize, tile: Tile) -> () {
        self.grid[row][col] = Some(tile);

        let routes_by_stone: Vec<(Stone, Vec<Position>)> = self.stones.values()
            .into_iter()
            .filter(|&stone| is_affected(stone.position, row, col))
            .map(|&stone| (stone, self.calculate_path(stone, stone.is_at_coords(row, col)))) //todo maybe good to keep routes in stones
            .collect();

        for (stone, route) in routes_by_stone {
            let &position = route.last().unwrap_or_else(|| {&stone.position});
            self.stones.insert(stone.color, Stone {color: stone.color, position}); //todo create stone.with_position
        }

    }


    fn calculate_path(&self, stone: Stone, is_at_initial_pos: bool) -> Vec<Position> {

        fn calculate(current_pos: Position, mut route: Vec<Position>, grid: [[Option<Tile> ; TILES_PER_ROW] ; TILES_PER_ROW]) -> Vec<Position> {
            if let Some((next_row, next_col, next_from_index)) = get_facing_position(current_pos) {

                match grid[next_row][next_col] {
                    Some(tile @ Tile::PathTile {paths: _}) => {

                        let next_pos = (next_row, next_col, tile.get_other_end(next_from_index));

                        route.push(next_pos);

                        calculate(next_pos, route, grid)
                    },
                    _ => route
                }
            } else { route }
        };

        let mut route = Vec::new();
        let current_pos= if is_at_initial_pos {
            let (row, col, i) = stone.position;

            let curr_tile = self.grid[row][col].expect("tile should be placed");
            let advanced_position = (row, col, curr_tile.get_other_end(i));

            route.push(stone.position); //not sure if needed
            route.push(advanced_position);
            advanced_position
        } else {
            stone.position
        };

        calculate(current_pos, route, self.grid)
    }
}

// Returns true if a stone at the specified position is affected by placing a tile at row, col
fn is_affected(stone_position: Position, row: usize, col: usize) -> bool {
    let (stone_row, stone_col, _) = stone_position;
    if stone_row == row && stone_col == col {
        return true;
    }

    match get_facing_position(stone_position) {
        Some((facing_row, facing_col, _)) => facing_col == col && facing_row == row,
        None => false
    }
}

fn get_facing_position((stone_row, stone_col, index): Position) -> Option<Position> {
    let (stone_row, stone_col) = (stone_row as i8, stone_col as i8);
    let (next_row, next_col) = match index {
        0 | 1 => (stone_row + 1, stone_col),
        2 | 3 => (stone_row, stone_col + 1),
        4 | 5 => (stone_row - 1, stone_col),
        6 | 7 => (stone_row, stone_col - 1),
        _ => panic!("non existent path index {}", index)
    };

    if next_row < 0 || next_col < 0 { return None }

    let next_index = match index {
        0 => 5,
        1 => 4,
        2 => 7,
        3 => 6,
        4 => 1,
        5 => 0,
        6 => 3,
        7 => 2,
        _ => panic!("non existent path index {}", index)
    };

    Some((next_row as usize, next_col as usize, next_index))
}

fn make_spawns() -> ArrayVec<[Position; SPAWN_COUNT]> {
    let mut result: ArrayVec<[Position; SPAWN_COUNT]> = ArrayVec::new();
    let max = TILES_PER_ROW - 1;

    for x in 0..TILES_PER_ROW {
        result.push((x, 0, 4));
        result.push((x, 0, 5));

        result.push((x, max, 0));
        result.push((x, max, 1));
    }

    for y in 0..TILES_PER_ROW {
        result.push((0, y, 6));
        result.push((0, y, 7));

        result.push((max, y, 2));
        result.push((max, y, 3));
    }

    result
}

impl Rotation {
    pub fn rotate_tile(&self, tile: &Tile) -> Tile {
        match *tile {
            Tile::DragonTile => *tile,
            Tile::PathTile {paths} => {
                let rotated_paths: ArrayVec<[Path; 4]>= paths.into_iter()
                    .map(|path| self.rotate_path(path))
                    .collect();

                Tile::PathTile { paths: rotated_paths.into_inner().unwrap() }
            }
        }
    }

    fn rotate_path(&self, (from, to): &Path) -> Path {
        let offset = match *self {
            Rotation::_0   => 0,
            Rotation::_90  => 2,
            Rotation::_180 => 4,
            Rotation::_270 => 6
        };

        let (new_from, new_to) = (from + offset, to + offset);
        (new_from % 8, new_to % 8)
    }
}


impl Deck {
    pub fn pop_tile(&mut self) -> Option<Tile> {
        self.tiles.pop()
    }

    pub fn from_file(filename: &str) -> Result<Deck, String> {
        let file = File::open(filename).or(Err("Can't open file"))?;
        let reader = BufReader::new(file);

        let tiles_as_text: Result<Vec<String>, _> = reader
            .lines()
            .map(|line_result| line_result.or(Err("Can't read tile data")))
            .collect();

        let tiles: Result<Vec<Tile>, _> = tiles_as_text?
            .iter()
            .map(|tile_text| Deck::parse_tile(tile_text))
            .collect();

        let mut tiles = tiles?;

        let mut rng = thread_rng();
        tiles.shuffle(&mut rng);
        tiles.insert(0, Tile::DragonTile);

        Ok(Deck { tiles })
    }

    fn parse_tile(tile_text: &str) -> Result<Tile, String> {
        let tile_text = tile_text.replace(" ", "");
        let digits: Result<Vec<u8>, _> = tile_text
            .chars()
            .map(|char| char.to_digit(10).ok_or("Tile data must be numeric"))
            .map(|digit| digit.map(|d| d as u8))
            .collect();

        let paths : ArrayVec<[Path; 4]> = digits?.into_iter()
            .tuples()
            .collect();

        let paths = paths.into_inner().unwrap();

        Ok(Tile::PathTile { paths })
    }
}
