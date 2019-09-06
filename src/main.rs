use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use quicksilver::prelude::*;
use std::result::Result;
use arrayvec::ArrayVec;

const SCALE: u32 = 2;
const SCREEN_WIDTH: u32 = 600 * SCALE;
const SCREEN_HEIGHT: u32 = 500 * SCALE;

type Path = (u8, u8);

struct Game {
    deck: Deck,
    board: Board,
}

impl State for Game {

    fn new() -> quicksilver::Result<Self> {
        let mut deck = Deck::from_file("tiles.txt").expect("Unable to create deck from tiles.txt");
        let mut board = Board::default();

        deck.pop_tile().map(|tile| {
            board.place_tile(0, 0, tile);
        });

        deck.pop_tile().map(|tile| {
            board.place_tile(3, 0, tile);
        });

        deck.pop_tile().map(|tile| {
            board.place_tile(1, 3, tile);
        });

        Ok(Self {deck, board})
    }

    fn update(&mut self, window: &mut Window) -> quicksilver::Result<()> {
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> quicksilver::Result<()> {

        window.clear(Color::ORANGE);
        self.board.draw(window);

        Ok(())
    }
}

fn main() {
    let settings = Settings {
        ..Default::default()
    };
    run::<Game>("TsuRusT", Vector::new(SCREEN_WIDTH, SCREEN_HEIGHT), settings);
}

#[derive(Debug, Copy, Clone)]
enum Rotation {
    _0,
    _90,
    _180,
    _270,
}

#[derive(Debug, Copy, Clone)]
enum Tile {
    DragonTile,
    PathTile {
        paths: [Path; 4],
        rotation: Rotation
    },
}

#[derive(Debug)]
struct Deck {
    tiles: Vec<Tile>,
}

impl Deck {
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

    pub fn pop_tile(&mut self) -> Option<Tile> {
        self.tiles.pop()
    }

    fn parse_tile(tile_text: &str) -> Result<Tile, String> {
        let tile_text = tile_text.replace(" ", "");
        let digits: Result<Vec<u8>, _> = tile_text
            .chars()
            .map(|char| char.to_digit(10).ok_or("Tile data must be numeric"))
            .map(|digit| digit.map(|d| d as u8))
            .collect();

        let paths : ArrayVec<[_; 4]> = digits?.into_iter()
            .tuples()
            .collect();

        let paths = paths.into_inner().unwrap();

        let rotation = Rotation::_0;

        Ok(Tile::PathTile { paths, rotation })
    }
}


const TILES_PER_ROW: usize = 6;
const TILE_SIDE_LENGTH: u32 = SCREEN_HEIGHT / 8;
const BOARD_BORDER: u32 = TILE_SIDE_LENGTH / 2;
const BOARD_SIDE_LENGTH: u32 = TILE_SIDE_LENGTH * TILES_PER_ROW as u32;

struct Board {
    grid: [[Option<Tile> ; TILES_PER_ROW] ; TILES_PER_ROW],
}

trait Drawable {
    fn draw(&self, window:&mut Window) -> ();
}

impl Board {
    fn place_tile(&mut self, row: usize, col: usize, tile: Tile) -> () {
        self.grid[row][col] = Some(tile);
    }
}

impl Default for Board {
    fn default() -> Board {
        Board {grid: [ [None; TILES_PER_ROW] ; TILES_PER_ROW]}
    }
}

impl Drawable for Board {
    fn draw(&self, window:&mut Window) -> () {
        window.draw(&Rectangle::new((BOARD_BORDER, BOARD_BORDER), (BOARD_SIDE_LENGTH, BOARD_SIDE_LENGTH)), Col(Color::BLACK));

        for (y, row) in self.grid.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                match tile {
                    Some(tile) => Board::draw_tile(&tile, x, y, window),
                    None => Board::draw_empty_space(x, y, window),
                }
            }
        }
    }

}

impl Board {
    fn draw_tile(tile: &Tile, x: usize, y: usize, window:&mut Window) -> () {
        let rect = Rectangle::new((x as u32 * TILE_SIDE_LENGTH, y as u32 * TILE_SIDE_LENGTH), (TILE_SIDE_LENGTH, TILE_SIDE_LENGTH));
        window.draw(&rect.translate((BOARD_BORDER, BOARD_BORDER)), Col(Color::BLUE));

   /*     match tile {
            Tile::PathTile{paths, rotation} => {
                paths.iter()
                    .for_each(|(from, to)| {

                    })
            },
            _ => {}
        }*/

    }

    fn draw_empty_space(x: usize, y: usize, window:&mut Window) -> () {
        let rect = Rectangle::new((x as u32 * TILE_SIDE_LENGTH, y as u32 * TILE_SIDE_LENGTH), (TILE_SIDE_LENGTH, TILE_SIDE_LENGTH));
        window.draw(&rect.translate((BOARD_BORDER, BOARD_BORDER)), Col(Color::from_rgba(127, 127, 127, 1.0)));
    }

/*

    fn normalize_path(from: u8, to: u8) -> (u8, u8) {
        let new_from = from % 2;
        let new_to = new_from + (to - from);
    }
*/


}


