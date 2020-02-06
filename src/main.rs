use tsurust::model::*;
use tsurust::model::{Tile};
use quicksilver::prelude::*;
use tsurust::ui::{SCREEN_WIDTH, SCREEN_HEIGHT};
use std::collections::HashMap;

struct Game {
    deck: Deck,
    pub board: Board,
}

impl State for Game {

    fn new() -> quicksilver::Result<Self> {
        let mut deck = Deck::from_file("tiles.txt").expect("Unable to create deck from tiles.txt");
        let mut players = HashMap::new();
       /* players.insert(PlayerColor::ORANGE, Stone {color: PlayerColor::ORANGE, position: (0,0,7)});
        players.insert(PlayerColor::GREEN, Stone {color: PlayerColor::GREEN, position: (0,0,4)});
        players.insert(PlayerColor::RED, Stone {color: PlayerColor::RED, position: (5,5,3)});
        players.insert(PlayerColor::YELLOW, Stone {color: PlayerColor::YELLOW, position: (5,0,0)});
        players.insert(PlayerColor::BLUE, Stone {color: PlayerColor::BLUE, position: (0,3,4)});
        players.insert(PlayerColor::GRAY, Stone {color: PlayerColor::GRAY, position: (3,0,6)});
        players.insert(PlayerColor::BLACK, Stone {color: PlayerColor::BLACK, position: (4,5,2)});
        players.insert(PlayerColor::WHITE, Stone {color: PlayerColor::WHITE, position: (2,0,6)});*/

        let mut board = Board::with_players(players);


        for row in 0..TILES_PER_ROW {
            for col in 0..TILES_PER_ROW {
                deck.pop_tile()
                    .map(|tile| board.place_tile(row, col, tile));
            }
        }
        //Todo factory method for more compact path creation
        let tile = Tile::PathTile {paths: [Path {a: 0, b:5}, Path { a:1, b:2 }, Path{ a:3, b:4 }, Path {a: 6,b: 7}]};


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


