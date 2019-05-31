use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use itertools::Itertools;
use rand::thread_rng;
use rand::seq::SliceRandom;



fn main() {
    let deck = Deck::from_file("tiles.txt").expect("Unable to create deck from tiles.txt");

}

#[derive(Debug)]
enum Rotation {
    _0,
    _90,
    _180,
    _270,
}

#[derive(Debug)]
enum Tile {
    DragonTile,
    PathTile {
        paths: [u8; 8],
        rotation: Rotation
    }
}

#[derive(Debug)]
struct Deck {
    tiles: Vec<Tile>
}

impl Deck {
    pub fn from_file(filename: &str) -> Result<Deck,String>{
        let file = File::open(filename).or(Err("Can't open file"))?;
        let reader = BufReader::new(file);

        let tiles: Result<Vec<Tile>, _> = reader
            .lines()
            .take_while(|line_result| line_result.is_ok())
            .map(|tile_text| Deck::parse_tile(tile_text.unwrap()))
            .inspect(|tile| println!("{:?}", tile))
            .collect();

        let mut tiles = tiles?;

        let mut rng = thread_rng();
        tiles.shuffle(&mut rng);
        tiles.push(Tile::DragonTile);

        Ok(Deck {tiles})
    }

    pub fn draw_tile(mut self) -> Option<Tile> {
        self.tiles.pop()
    }

    fn parse_tile(tile_text: String) -> Result<Tile, String> {
        let tile_text = tile_text.replace(" ", "");
        let digits: Result<Vec<u32>, _> = tile_text
            .chars()
            .map(|char| char.to_digit(10).ok_or("Tile data must be numeric"))
            .collect();

        let mut paths :[u8;8] = [0;8];

        for (from, to) in digits?.into_iter().tuples() {
            paths[from as usize] = to as u8;
            paths[to as usize] = from as u8;
        }

        let rotation = Rotation::_0;

        Ok(Tile::PathTile {paths, rotation})
    }
}