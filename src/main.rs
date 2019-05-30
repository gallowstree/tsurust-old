use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let file = File::open("tiles.txt").expect("Can't open tiles.txt");
    let reader = BufReader::new(file);
    let tiles = reader
        .lines()
        .map(|line_result| line_result.expect("failed to read tile data"))
        .map(|tile_text| parse_tile(tile_text));
}

fn parse_tile(tile_text: String) -> Option<Tile> {
    let mut paths :[u8;8] = [0;8];

    let mut digits = tile_text.replace(" ", "")
        .chars()
        .map(|char| char.to_digit(10).expect("Tile data is not a digit"));


    while digits.peekable().peek().is_some() {
        let path_1 = digits.next().unwrap();
        let path_2 = digits.next().unwrap();
        paths[path_1] = path_2;
        paths[path_2] = path_1;
    }

    let rotation = Rotation::_0;
    Some(Tile {paths, rotation})
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