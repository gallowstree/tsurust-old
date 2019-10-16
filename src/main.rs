use tsurust::model::*;
use quicksilver::prelude::*;
use tsurust::model::Tile::PathTile;
use tsurust::ui::{SCREEN_WIDTH, SCREEN_HEIGHT, UI};

struct Game {
    deck: Deck,
    board: Board,
}

impl State for Game {

    fn new() -> quicksilver::Result<Self> {
        let mut deck = Deck::from_file("tiles.txt").expect("Unable to create deck from tiles.txt");
        let mut board = Board::default();

        let normalized_pound_sign_tile = PathTile {paths: [(0,5), (1,4), (2,7), (3,6)], rotation: Rotation::_0};
        let d_tile = PathTile {paths: [(6,7), (1,4), (2,3), (0,5)], rotation: Rotation::_0};
        board.place_tile(1,2, d_tile);
        board.place_tile(0,0, normalized_pound_sign_tile);
        board.place_tile(1,1, normalized_pound_sign_tile);

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


