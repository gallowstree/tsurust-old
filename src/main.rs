use tsurust::model::*;
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
        players.insert(PlayerColor::ORANGE, Stone {color: PlayerColor::ORANGE, position: (0,0,7)});
        players.insert(PlayerColor::GREEN, Stone {color: PlayerColor::GREEN, position: (0,0,4)});
        players.insert(PlayerColor::RED, Stone {color: PlayerColor::RED, position: (5,5,3)});

        let mut board = Board::with_players(players);


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


