use tsurust::model::*;
use tsurust::model::{Tile};
use quicksilver::prelude::*;
use tsurust::ui::{SCREEN_WIDTH, SCREEN_HEIGHT};
use std::collections::HashMap;
use tsurust::model::RotationDirection::{CLOCKWISE, COUNTERCLOCKWISE};

struct Game {
    deck: Deck,
    pub board: Board,
}

impl State for Game {

    fn new() -> quicksilver::Result<Self> {
        let mut deck = Deck::from_file("tiles.txt").expect("Unable to create deck from tiles.txt");
        let mut players = HashMap::new();
        players.insert(PlayerColor::ORANGE, Stone {color: PlayerColor::ORANGE, position: Position {row: 0,col: 0, path_index: 7}});
        players.insert(PlayerColor::GREEN, Stone {color: PlayerColor::GREEN, position: Position {row: 0,col: 0, path_index: 4}});
        players.insert(PlayerColor::RED, Stone {color: PlayerColor::RED, position: Position {row: 5,col: 5, path_index: 3}});


        let mut board = Board::with_players(players);

        // TODO: do we even need a dragon tile?!
/*
        for row in 0..TILES_PER_ROW {
            for col in 0..TILES_PER_ROW {
                deck.pop_tile()
                    .map(|tile| board.place_tile(row, col, tile));
            }
        }*/
        //Todo factory method for more compact path creation
        let tile = Tile::PathTile {paths: [Path {a: 0, b:5}, Path { a:1, b:2 }, Path{ a:3, b:4 }, Path {a: 6,b: 7}]};
        board.place_tile(0,0, tile.rotate(COUNTERCLOCKWISE));

        Ok(Self {deck, board})
    }

    fn update(&mut self, window: &mut Window) -> quicksilver::Result<()> {
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> quicksilver::Result<()> {
        window.clear(Color::from_rgba(40, 40, 40, 1.0));
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


