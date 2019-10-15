use quicksilver::prelude::*;
use quicksilver::graphics::{Drawable};
use std::ops::Deref;
use crate::model::*;
use crate::model::Tile;

const SCALE: u32 = 2;
pub const SCREEN_WIDTH: u32 = 600 * SCALE;
pub const SCREEN_HEIGHT: u32 = 500 * SCALE;

const TILE_SIDE_LENGTH: u32 = SCREEN_HEIGHT / 8;
const BOARD_BORDER: u32 = TILE_SIDE_LENGTH / 2;
const BOARD_SIDE_LENGTH: u32 = TILE_SIDE_LENGTH * TILES_PER_ROW as u32;


pub trait UI {
    fn draw(&self, window:&mut Window) -> ();

    fn draw_ex(&self, window:&mut Window, trans: Transform, col: Color) {
        self.draw(window);
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
                    Some(tile) => draw_tile(&tile, x, y, window),
                    None => draw_empty_space(x, y, window),
                }
            }
        }
    }
}


const PATH_THICKNESS : u8 = 4;

fn draw_tile(tile: &Tile, x: usize, y: usize, window:&mut Window) -> () {
    let rect = tile_square_at(x, y);

    window.draw(&rect, Col(Color::BLACK));

    if let Tile::PathTile { paths, rotation } = tile {
        draw_paths(paths, rotation, x, y, window)
    }
}

fn draw_empty_space(x: usize, y: usize, window:&mut Window) -> () {
    let rect = tile_square_at(x, y);
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
        .map(|path| (normalize_path(path), path))
        .for_each(|(original_path, normalized_path)| {
            let offset = coords_to_pixels(x, y);
            let transform = Transform::translate(offset);
            let path_drawable = get_path_drawable(normalized_path);

            path_drawable.deref().draw_ex(window, transform, Color::WHITE);
        });
}

fn get_path_drawable(normalized_path: &Path) -> Box<dyn UI> {
    match normalized_path {
        (0,5) => Box::new(vertical_left()),
        (1,4) => Box::new(vertical_right()),
        (3,6) => Box::new(horizontal_hi()),
        (2,7) => Box::new(horizontal_lo()),
        _ => Box::new(Line::new((0,0), (0,0)))
    }
}

fn vertical_left() -> Line {
    let xo = TILE_SIDE_LENGTH / 3;
    let yo = 0;
    let start = (xo, yo);
    let end = (xo, yo + TILE_SIDE_LENGTH);

    Line::new(start, end ).with_thickness(PATH_THICKNESS)
}

fn vertical_right() -> Line {
    vertical_left().translate((TILE_SIDE_LENGTH/3, 0))
}

fn horizontal_hi() -> Line {
    let xo = 0;
    let yo = TILE_SIDE_LENGTH / 3;
    let start = (xo, yo);
    let end = (xo + TILE_SIDE_LENGTH, yo);

    Line::new(start, end ).with_thickness(PATH_THICKNESS)
}

fn horizontal_lo() -> Line {
    horizontal_hi().translate((0, TILE_SIDE_LENGTH/3))
}

fn tile_square_at(x: usize, y: usize) -> Rectangle {
    let position = coords_to_pixels(x, y);
    let size = (TILE_SIDE_LENGTH, TILE_SIDE_LENGTH);

    Rectangle::new(position, size)
}

fn coords_to_pixels(x: usize, y: usize) -> (u32, u32) {
    (x as u32 * TILE_SIDE_LENGTH + BOARD_BORDER, y as u32 * TILE_SIDE_LENGTH + BOARD_BORDER)
}

