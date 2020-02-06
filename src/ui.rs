use quicksilver::prelude::*;
use quicksilver::graphics::{Drawable, Mesh};
use std::ops::{Deref, Neg};
use crate::model::*;
use crate::model::Tile;
use std::collections::HashMap;
use std::borrow::Borrow;

const SCALE: u32 = 2;
pub const SCREEN_WIDTH: u32 = 600 * SCALE;
pub const SCREEN_HEIGHT: u32 = 500 * SCALE;

// Length of a tile's side
const TILE_SIDE_LENGTH: u32 = SCREEN_HEIGHT / 8;
const THIRD : u32 = TILE_SIDE_LENGTH / 3;
const BOARD_SIDE_LENGTH: u32 = TILE_SIDE_LENGTH * TILES_PER_ROW as u32;

// Space between the window's edge and the board
const BOARD_BORDER: u32 = TILE_SIDE_LENGTH / 2;
const PATH_THICKNESS : u8 = 4;

// A path is drawn as two outer segments at the edge of the tile connected by a middle segment
// this is the length of the outer edge segments
const PATH_EDGE_SEGMENT_LENGTH: u32 = TILE_SIDE_LENGTH / 6;

const STONE_RADIUS: u32 = (PATH_EDGE_SEGMENT_LENGTH as f32 * 0.75) as u32;
const STONE_BORDER: u32 = 2;

impl Board {
    pub fn draw(&self, window:&mut Window) -> () {
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
        draw_grid_lines(window);
        draw_stones(&self.stones, window);
    }

    fn draw_spawns(&self, window:&mut Window) {
        self.spawns.iter().for_each(|spawn: &Position|{
            let thickness = PATH_THICKNESS as f32 * 1.5;
            let length = THIRD as f32 / 4.0;

            let center = path_index_position(spawn.path_index as PathIndex);

            let size = match spawn.path_index {
                0 | 1 | 4 | 5 => (thickness, length),
                2 | 3 | 6 | 7 => (length, thickness),
                _ => panic!("non existent path index {}", spawn.path_index)
            };

            let rect = Rectangle::new_sized(size)
                                 .with_center(center)
                                 .translate(coords_to_vec(spawn.row, spawn.col));

            window.draw(&rect, Col(Color::WHITE));
        });
    }
}


fn draw_tile(tile: &Tile, x: usize, y: usize, window:&mut Window) -> () {
    let rect = tile_square_at(x, y);

    window.draw(&rect, Col(Color::from_rgba(121, 45, 27, 1.0)));

    if let Tile::PathTile { paths } = tile {
        draw_paths(paths, x, y, window)
    }
}

fn draw_empty_space(x: usize, y: usize, window:&mut Window) -> () {
    window.draw(&tile_square_at(x, y), Col(Color::BLACK));
}

fn draw_paths(paths:&[Path; 4], x: usize, y: usize, window: &mut Window) {
    paths.iter()
        .for_each(|&Path {a: from, b: to}| {
            let start_segment = path_edge_segment(from);
            let end_segment = path_edge_segment(to);
            let middle_segment = Line::new(start_segment.b, end_segment.b);

            let offset = coords_to_pixels(x, y);
            let transform = Transform::translate(offset);

            window.draw_ex(&start_segment.with_thickness(PATH_THICKNESS), Col(Color::WHITE.with_alpha(0.75)), transform, 1);
            window.draw_ex(&middle_segment.with_thickness(PATH_THICKNESS), Col(Color::WHITE.with_alpha(0.75)), transform, 1);
            window.draw_ex(&end_segment.with_thickness(PATH_THICKNESS), Col(Color::WHITE.with_alpha(0.75)), transform, 1);
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

fn draw_stones(stones: &HashMap<PlayerColor, Stone>, window: &mut Window) {
    stones.values().for_each(|stone| {
        let center = to_pixels(stone.position);
        let circle = Circle::new(center, STONE_RADIUS);
        let border = Circle::new(center, STONE_RADIUS + STONE_BORDER);
        let color = stone.color.to_color();

        window.draw_ex(&circle, Col(color), Transform::IDENTITY, 3);

        let border_color = if 255.0 * (color.r * 0.299 + color.g * 0.587 + color.b * 0.114) > 149.0 {
            Color::BLACK
        } else {
            Color::WHITE
        }.with_alpha(0.5);

        window.draw_ex(&border, Col(border_color), Transform::IDENTITY, 2);
    })
}

fn path_index_position(i: PathIndex) -> (u32, u32) {
    match i {
        0 => (THIRD, TILE_SIDE_LENGTH),
        1 => (2 * THIRD, TILE_SIDE_LENGTH),
        2 => (TILE_SIDE_LENGTH, 2 * THIRD),
        3 => (TILE_SIDE_LENGTH, THIRD),
        4 => (2 * THIRD, 0),
        5 => (THIRD, 0),
        6 => (0, THIRD),
        7 => (0, 2 * THIRD),
        _ => panic!("non existent path index {}", i)
    }
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
        _ => panic!("non existent path index {}", index)
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

fn to_pixels(pos: Position) -> (u32, u32) {
    let (x, y) = coords_to_pixels(pos.col, pos.row);
    let (offset_x, offset_y) = path_index_position(pos.path_index as PathIndex);
    (x + offset_x, y + offset_y)
}

impl PlayerColor {
    fn to_color(&self) -> Color {
        match *self {
            PlayerColor::WHITE => Color::WHITE,
            PlayerColor::RED => Color::RED,
            PlayerColor::BLACK => Color::BLACK,
            PlayerColor::YELLOW => Color::YELLOW,
            PlayerColor::BLUE => Color::BLUE,
            PlayerColor::GREEN => Color::GREEN,
            PlayerColor::ORANGE => Color::ORANGE,
            PlayerColor::GRAY => Color::from_rgba(127, 127,127,1.0)
        }
    }
}



