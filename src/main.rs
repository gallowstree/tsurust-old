use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use itertools::Itertools;


fn main() {
    let file = File::open("tiles.txt").expect("Can't open tiles.txt");
    let reader = BufReader::new(file);
    let tiles = reader
        .lines()
        .map(|line_result| line_result.expect("failed to read tile data"))
        .map(|tile_text| parse_tile(tile_text))
        .for_each(|tile| println!("{:?}", tile));
}

fn parse_tile(tile_text: String) -> Tile {
    let tile_text = tile_text.replace(" ", "");
    let digits = tile_text
        .chars()
        .map(|char| char.to_digit(10).unwrap());

    let mut paths :[u8;8] = [0;8];

    for (from, to) in digits.tuples() {
        paths[from as usize] = to as u8;
        paths[to as usize] = from as u8;
    }

    let rotation = Rotation::_0;

    Tile {paths, rotation}
}

#[derive(Debug)]
enum Rotation {
    _0,
    _90,
    _180,
    _270,
}

#[derive(Debug)]
struct Tile {
    paths: [u8; 8],
    rotation: Rotation
}