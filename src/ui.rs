use quicksilver::prelude::*;
use quicksilver::graphics::{Drawable, Mesh};
use std::ops::{Deref, Neg};
use crate::model::*;
use crate::model::Tile;

const SCALE: u32 = 2;
pub const SCREEN_WIDTH: u32 = 600 * SCALE;
pub const SCREEN_HEIGHT: u32 = 500 * SCALE;

const TILE_SIDE_LENGTH: u32 = SCREEN_HEIGHT / 8;
const BOARD_BORDER: u32 = TILE_SIDE_LENGTH / 2;
const BOARD_SIDE_LENGTH: u32 = TILE_SIDE_LENGTH * TILES_PER_ROW as u32;
const PATH_EDGE_SEGMENT_LENGTH: u32 = TILE_SIDE_LENGTH / 6;
const PATH_THICKNESS : u8 = 4;
const THIRD : u32 = TILE_SIDE_LENGTH / 3;
const HALF : u32 = TILE_SIDE_LENGTH / 2;

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

        for (y, row) in self.grid.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                match tile {
                    Some(tile) => draw_tile(&tile, x, y, window),
                    None => draw_empty_space(x, y, window),
                }
            }
        }

        self.draw_spawns(window);
        Board::draw_grid_lines(window);
    }
}

fn draw_tile(tile: &Tile, x: usize, y: usize, window:&mut Window) -> () {
    let rect = tile_square_at(x, y);

    window.draw(&rect, Col(Color::from_rgba(121, 35, 20, 1.0)));

    if let Tile::PathTile { paths, rotation } = tile {
        draw_paths(paths, rotation, x, y, window)
    }
}

fn draw_empty_space(x: usize, y: usize, window:&mut Window) -> () {
    window.draw(&tile_square_at(x, y), Col(Color::BLACK));
}



fn draw_paths(paths:&[Path; 4], rotation: &Rotation, x: usize, y: usize, window: &mut Window) {
    paths.iter()
        .map(|path| rotation.apply(path))
        .for_each(|(from, to)| {
            let offset = coords_to_pixels(x, y);
            let transform = Transform::translate(offset);

            let start_segment = path_edge_segment(from);
            let end_segment = path_edge_segment(to);
            let middle_segment = Line::new(start_segment.b, end_segment.b);

            window.draw_ex(&start_segment.with_thickness(PATH_THICKNESS), Col(Color::WHITE), transform, 1);
            window.draw_ex(&end_segment.with_thickness(PATH_THICKNESS), Col(Color::WHITE), transform, 1);
            window.draw_ex(&middle_segment.with_thickness(PATH_THICKNESS), Col(Color::WHITE), transform, 1);
        });
}

fn path_edge_segment(index: PathIndex) -> Line {
    let (start, end) = match index {
        0 => ((THIRD, TILE_SIDE_LENGTH), (THIRD, TILE_SIDE_LENGTH - PATH_EDGE_SEGMENT_LENGTH)),
        1 => ((2 * THIRD, TILE_SIDE_LENGTH), (2 * THIRD, TILE_SIDE_LENGTH - PATH_EDGE_SEGMENT_LENGTH)),
        2 => ((TILE_SIDE_LENGTH, 2 * THIRD), (TILE_SIDE_LENGTH - PATH_EDGE_SEGMENT_LENGTH, 2 * THIRD)),
        3 => ((TILE_SIDE_LENGTH, THIRD), (TILE_SIDE_LENGTH - PATH_EDGE_SEGMENT_LENGTH, THIRD)),
        4 => ((2 * THIRD, 0), (2 * THIRD, PATH_EDGE_SEGMENT_LENGTH)),
        5 => ((THIRD, 0), (THIRD, PATH_EDGE_SEGMENT_LENGTH)),
        6 => ((0, THIRD), (PATH_EDGE_SEGMENT_LENGTH, THIRD)),
        7 => ((0, 2 * THIRD), (PATH_EDGE_SEGMENT_LENGTH, 2 * THIRD)),
        _ => panic!("wtf dude?")
    };

    Line::new(start, end)
}

fn tile_square_at(x: usize, y: usize) -> Rectangle {
    let position = coords_to_pixels(x, y);
    let size = (TILE_SIDE_LENGTH, TILE_SIDE_LENGTH);

    Rectangle::new(position, size)
}

fn coords_to_pixels(x: usize, y: usize) -> (u32, u32) {
    (x as u32 * TILE_SIDE_LENGTH + BOARD_BORDER, y as u32 * TILE_SIDE_LENGTH + BOARD_BORDER)
}

fn coords_to_vec(x: usize, y: usize) -> Vector {
    Vector::new(x as u32 * TILE_SIDE_LENGTH + BOARD_BORDER, y as u32 * TILE_SIDE_LENGTH + BOARD_BORDER)
}

impl Board {
    fn draw_spawns(&self,  window:&mut Window) {
        self.spawns.iter().for_each(|spawn|{
            let &(row, col, i) = spawn;

            let is_at_bottom = i == 0 || i == 1;
            let is_at_top = i == 4 || i == 5;
            let vertical = is_at_bottom || is_at_top;
            let is_first = i == 0 || i == 5 || i == 3 || i == 6; //is this the topmost (when vertical) or leftmost (when horizontal, i.e. !vertical) spawn inside the tile at (row, col)

            let offset = match (vertical, is_first) {
                (true, true) => Vector::new(THIRD, 0),
                (true, false) => Vector::new(2 * THIRD, 0),
                (false, true) => Vector::new(0, THIRD),
                (false, false) => Vector::new(0, 2 * THIRD),
            };

            let is_at_far_right = i == 2 || i == 3;
            let edge_offset = match (is_at_bottom, is_at_far_right) {
                (true, false) => Vector::new(0, TILE_SIDE_LENGTH),
                (false, true) => Vector::new(TILE_SIDE_LENGTH, 0),
                _ => Vector::new(0, 0)
            };

            let size = match vertical {
                true => (PATH_THICKNESS as f32 * 1.5, THIRD as f32 / 4.0 ),
                false => (THIRD as f32 / 4.0, PATH_THICKNESS as f32 * 1.5),
            };

            let center = coords_to_vec(row, col) + offset + edge_offset;
            let rect = Rectangle::new_sized(size).with_center(center.clone());

            window.draw(&rect, Col(Color::WHITE));
        });
    }

    fn draw_grid_lines(window:&mut Window) {
        for x in 0..=TILES_PER_ROW {
            let x = x as u32 * TILE_SIDE_LENGTH;
            let start = (x , 0);
            let end = (x, BOARD_SIDE_LENGTH);
            let line = Line::new(start, end).translate((BOARD_BORDER as u32, BOARD_BORDER as u32));
            window.draw(&line, Col(Color::YELLOW));

            let y = x;
            let start = (0 , y);
            let end = (BOARD_SIDE_LENGTH, y);
            let line = Line::new(start, end).translate((BOARD_BORDER as u32, BOARD_BORDER as u32));
            window.draw(&line, Col(Color::YELLOW));
        }
    }
}