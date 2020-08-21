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

#[derive(Debug, Copy, Clone)]
pub struct Path {
    pub a: PathIndex,
    pub b: PathIndex
}

impl Path {
    fn offset_by(&self, offset: PathIndex) -> Path {
        let (new_a, new_b) = (self.a + offset, self.b + offset);
        Path {a: new_a % 8, b: new_b % 8}
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Position {
    pub row: usize,
    pub col: usize,
    pub path_index: PathIndex//TODO perhaps use usize for all? or u8 for all?
}

impl Position {
    fn with_path_index(&self, path_index: PathIndex) -> Position {
        Position {row: self.row, col: self.col, path_index}
    }
}

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

#[derive(Debug, Copy, Clone)]
pub enum RotationDirection {
    CLOCKWISE,
    COUNTERCLOCKWISE
}

impl RotationDirection {
    fn offset(&self) -> PathIndex {
        match *self {
            RotationDirection::CLOCKWISE => 2,
            RotationDirection::COUNTERCLOCKWISE => 6, //should be -2 but % is not equivalent to mod with negative numbers
        }
    }
}

impl Tile {
    //TODO rename this with a better name
    fn get_other_end(&self, from_index: PathIndex) -> PathIndex {
        if let Tile::PathTile {paths} = self {
            let &path = paths
                .iter()
                .find(|&path| path.a == from_index || path.b == from_index)
                .expect("malformed tile");

            return if path.a != from_index {path.a} else {path.b}
        }

        panic!("trying to move across dragon tile")
    }

    pub fn rotate(&self, direction: RotationDirection) -> Tile {
        match *self {
            Tile::DragonTile => *self,
            Tile::PathTile {paths} => {
                let circular_offset = direction.offset();

                let rotated_paths: ArrayVec<[Path; 4]>= paths.into_iter()
                    .map(|path| path.offset_by(circular_offset))
                    .collect();

                Tile::PathTile { paths: rotated_paths.into_inner().unwrap() }
            }
        }
    }

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
        self.position.row == row && self.position.col == col
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
            if let Some(Position {row, col, path_index}) = get_facing_position(current_pos) {

                match grid[row][col] {
                    Some(tile @ Tile::PathTile {paths: _}) => {

                        let next_pos = Position {row, col, path_index: tile.get_other_end(path_index)};

                        route.push(next_pos);

                        calculate(next_pos, route, grid)
                    },
                    _ => route
                }
            } else { route }
        };

        let mut route = Vec::new();
        let current_pos= if is_at_initial_pos {
            let initial_pos = stone.position;
            let curr_tile = self.grid[initial_pos.row][initial_pos.col].expect("tile should be placed");
            let advanced_position = initial_pos.with_path_index(curr_tile.get_other_end(initial_pos.path_index));

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
    if stone_position.row == row && stone_position.col == col {
        return true;
    }

    match get_facing_position(stone_position) {
        Some(facing_pos) => facing_pos.col == col && facing_pos.row == row,
        None => false
    }
}

fn get_facing_position(current: Position) -> Option<Position> {
    let (current_row, current_col) = (current.row as i8, current.col as i8);
    let (facing_row, facing_col) = match current.path_index {
        0 | 1 => (current_row + 1, current_col),
        2 | 3 => (current_row, current_col + 1),
        4 | 5 => (current_row - 1, current_col),
        6 | 7 => (current_row, current_col - 1),
        _ => panic!("non existent path index {}",  current.path_index)
    };

    if facing_row < 0 || facing_col < 0 { return None }

    let facing_index = match current.path_index {
        0 => 5,
        1 => 4,
        2 => 7,
        3 => 6,
        4 => 1,
        5 => 0,
        6 => 3,
        7 => 2,
        _ => panic!("non existent path index {}",  current.path_index)
    };

    Some(Position {row: facing_row as usize, col: facing_col as usize, path_index: facing_index })
}

fn make_spawns() -> ArrayVec<[Position; SPAWN_COUNT]> {
    let mut result: ArrayVec<[Position; SPAWN_COUNT]> = ArrayVec::new();
    let max = TILES_PER_ROW - 1;

    for x in 0..TILES_PER_ROW {
        result.push(Position { row: x, col: 0, path_index: 4 });
        result.push(Position { row: x, col: 0, path_index: 5 });

        result.push(Position { row: x, col: max, path_index: 0 });
        result.push(Position { row: x, col: max, path_index: 1 });
    }

    for y in 0..TILES_PER_ROW {
        result.push(Position { row: 0, col: y, path_index: 6 });
        result.push(Position { row: 0, col: y, path_index: 7 });

        result.push(Position { row: max, col: y, path_index: 2 });
        result.push(Position { row: max, col: y, path_index: 3 });
    }

    result
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
            .map(|(a,b)| Path {a, b})
            .collect();

        let paths = paths.into_inner().unwrap();

        Ok(Tile::PathTile { paths })
    }
}
