use tsurust::model::*;
use quicksilver::prelude::*;
use tsurust::ui::{SCREEN_WIDTH, SCREEN_HEIGHT};

impl State for Game {

    fn new() -> quicksilver::Result<Self> {
        let mut deck = Deck::from_file("tiles.txt").expect("Unable to create deck from tiles.txt");
        let mut board = Board::default();

        for row in 0..TILES_PER_ROW {
            for col in 0..TILES_PER_ROW {
                deck.pop_tile()
                    .map(|tile| board.place_tile(row, col, tile));
            }
        }

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


