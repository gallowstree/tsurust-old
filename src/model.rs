use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::result::Result;
use arrayvec::ArrayVec;
use std::ops::Deref;

pub const TILES_PER_ROW: usize = 6;

pub type Path = (u8, u8);

pub struct Board {
    pub grid: [[Option<Tile> ; TILES_PER_ROW] ; TILES_PER_ROW],
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

impl Board {
    pub fn place_tile(&mut self, row: usize, col: usize, tile: Tile) -> () {
        self.grid[row][col] = Some(tile);
    }
}

impl Default for Board {
    fn default() -> Board {
        Board {grid: [ [None; TILES_PER_ROW] ; TILES_PER_ROW]}
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
            .inspect(|tile| println!("{:?}", tile))
            .collect();

        let mut tiles = tiles?;

        let mut rng = thread_rng();
        tiles.shuffle(&mut rng);
        tiles.push(Tile::DragonTile);

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
