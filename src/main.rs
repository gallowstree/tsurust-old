use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use quicksilver::prelude::*;
use std::result::Result;
use arrayvec::ArrayVec;
use crate::Tile::PathTile;
use std::ops::Deref;
use quicksilver::graphics::{Drawable, Mesh};

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

        let normalized_pound_sign_tile = PathTile {paths: [(0,5), (1,4), (2,7), (3,6)], rotation: Rotation::_0};
        board.place_tile(1,2, normalized_pound_sign_tile);

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
/*
    let mut deck = Deck::from_file("tiles.txt").expect("Unable to create deck from tiles.txt");

    while let Some(tile) = deck.pop_tile() {
        if let Tile::PathTile {paths, rotation} = tile {
            println!("{:?}", tile);
            paths.iter().
                map(|p| (p, Board::normalize_path(p)))
                .for_each(|(before, after)|print!("\t{:?} => {:?}", before, &after))
        }
        println!()
    }
*/
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


const TILES_PER_ROW: usize = 6;
const TILE_SIDE_LENGTH: u32 = SCREEN_HEIGHT / 8;
const BOARD_BORDER: u32 = TILE_SIDE_LENGTH / 2;
const BOARD_SIDE_LENGTH: u32 = TILE_SIDE_LENGTH * TILES_PER_ROW as u32;

struct Board {
    grid: [[Option<Tile> ; TILES_PER_ROW] ; TILES_PER_ROW],
}

trait UI {
    fn draw(&self, window:&mut Window) -> ();

    fn draw_ex(&self, window:&mut Window, trans: Transform, col: Color) {
        self.draw(window);
    }
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

impl UI for Line {
    fn draw(&self, window: &mut Window) -> () {
        window.draw(self, Col(Color::BLACK));
    }

    fn draw_ex(&self, window: &mut Window, trans: Transform, col: Color) {
        window.draw_ex(self, Col(col), trans, 1);
    }
}


impl UI for Board {
    fn draw(&self, window:&mut Window) -> () {
        let board_position = (BOARD_BORDER, BOARD_BORDER);
        let board_size = (BOARD_SIDE_LENGTH, BOARD_SIDE_LENGTH);
        let board_rect = Rectangle::new(board_position, board_size);

        window.draw(&board_rect, Col(Color::BLACK));
        //TODO draw grid lines here? or do it in the empty spaces?

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
    const PATH_THICKNESS : u8 = 4;

    fn draw_tile(tile: &Tile, x: usize, y: usize, window:&mut Window) -> () {
        let rect = Board::tile_square_at(x, y);

        window.draw(&rect, Col(Color::BLACK));

        if let Tile::PathTile { paths, rotation } = tile {
            Board::draw_paths(paths, rotation, x, y, window)
        }
    }

    fn draw_empty_space(x: usize, y: usize, window:&mut Window) -> () {
        let rect = Board::tile_square_at(x, y);
        let bkg_col = Col(Color::from_rgba(127, 127, 127, 1.0));

        window.draw(&rect, bkg_col);
    }

    fn normalize_path(path: &Path) -> Path {
        let (from, to) = path;
        let new_from = from % 2;
        let new_to = new_from + (to - from);
        (new_from, new_to)
    }

    fn draw_paths(paths:&[Path; 4], rotation: &Rotation, x: usize, y: usize, window: &mut Window) {
        paths.iter()
            .map(|path| (Board::normalize_path(path), path))
            .for_each(|(original_path, normalized_path)| {
                let offset = Board::coords_to_pixels(x, y);
                let transform = Transform::translate(offset);
                let path_drawable = Board::get_path_drawable(normalized_path);

                path_drawable.deref().draw_ex(window, transform, Color::WHITE);
            });
    }

    fn get_path_drawable(normalized_path: &Path) -> Box<dyn UI> {
        match normalized_path {
            (0,5) => Box::new(Board::vertical_left()),
            (1,4) => Box::new(Board::vertical_right()),
            (3,6) => Box::new(Board::horizontal_hi()),
            (2,7) => Box::new(Board::horizontal_lo()),
            _ => Box::new(Line::new((0,0), (0,0)))
        }
    }

    fn vertical_left() -> Line {
        let xo = TILE_SIDE_LENGTH / 3;
        let yo = 0;
        let start = (xo, yo);
        let end = (xo, yo + TILE_SIDE_LENGTH);

        Line::new(start, end ).with_thickness(Board::PATH_THICKNESS)
    }

    fn vertical_right() -> Line {
        Board::vertical_left().translate((TILE_SIDE_LENGTH/3, 0))
    }

    fn horizontal_hi() -> Line {
        let xo = 0;
        let yo = TILE_SIDE_LENGTH / 3;
        let start = (xo, yo);
        let end = (xo + TILE_SIDE_LENGTH, yo);

        Line::new(start, end ).with_thickness(Board::PATH_THICKNESS)
    }

    fn horizontal_lo() -> Line {
        Board::horizontal_hi().translate((0, TILE_SIDE_LENGTH/3))
    }

    fn tile_square_at(x: usize, y: usize) -> Rectangle {
        let position = Board::coords_to_pixels(x, y);
        let size = (TILE_SIDE_LENGTH, TILE_SIDE_LENGTH);

        Rectangle::new(position, size)
    }

    fn coords_to_pixels(x: usize, y: usize) -> (u32, u32) {
        (x as u32 * TILE_SIDE_LENGTH + BOARD_BORDER, y as u32 * TILE_SIDE_LENGTH + BOARD_BORDER)
    }

}
