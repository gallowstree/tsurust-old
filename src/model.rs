use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::result::Result;
use arrayvec::ArrayVec;

pub const TILES_PER_ROW: usize = 6;
pub const SPAWN_COUNT : usize = TILES_PER_ROW * 2 * 4;

pub type PathIndex = u8;
pub type Path = (PathIndex, PathIndex);
pub type Position = (usize, usize, usize); // row, column, path_index

pub struct Board {
    pub grid: [[Option<Tile> ; TILES_PER_ROW] ; TILES_PER_ROW],
    pub spawns: ArrayVec<[Position; SPAWN_COUNT]>
}

#[derive(Debug)]
pub struct Deck {
    tiles: Vec<Tile>,
}

#[derive(Debug, Copy, Clone)]
pub enum Tile {
    DragonTile,
    PathTile {
        paths: [Path; 4],
        rotation: Rotation
    },
}

#[derive(Debug, Copy, Clone)]
pub enum Rotation {
    _0,
    _90,
    _180,
    _270,
}

impl Default for Board {
    fn default() -> Board {

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
        };

        Board {
            grid: [ [None; TILES_PER_ROW] ; TILES_PER_ROW],
            spawns: make_spawns()
        }
    }
}

impl Board {
    pub fn place_tile(&mut self, row: usize, col: usize, tile: Tile) -> () {
        self.grid[row][col] = Some(tile);
    }
}

impl Rotation {
    pub fn apply(&self, (from, to): &Path) -> Path {
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

        let rotation = Rotation::_0;

        Ok(Tile::PathTile { paths, rotation })
    }
}
