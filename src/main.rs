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
        let n_tile = PathTile {paths: [(0, 1), (2, 7), (3, 6), (4, 5)], rotation: Rotation::_0};
        let cc_tile = PathTile {paths: [(0, 1), (2, 3), (4, 5), (6, 7)], rotation: Rotation::_0};
        board.place_tile(0,0, n_tile);
        board.place_tile(0,1, cc_tile);
        board.place_tile(1,0, normalized_pound_sign_tile);

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

    /*

    let mut deck = Deck::from_file("tiles.txt").expect("Unable to create deck from tiles.txt");
    while let Some(tile) = deck.pop_tile() {
        if let Tile::PathTile {paths, rotation} = tile {
            paths.iter().
                map(|p| (p, ui::normalize_path(p)))
                .filter(|(&o, n)| o != *n)
                .unique()
                .for_each(|(before, after)|print!("\t{:?} => {:?}", before, &after))
        }
        println!()
    }
    */
}


