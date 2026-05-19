use crate::game::{Game, board::Board, boat::{Boat, Direction}};

mod game;

fn main() {
    let boats1 = vec![Boat::new((0, 1), Direction::North, 2)];
    let boats2 = vec![Boat::new((3, 1), Direction::East, 2)];
    let mut game: Game = Game::new(Board::new(boats1), Board::new(boats2));
    println!("{:?}", game.hit((0, 1)));
    println!("{:?}", game.hit((4, 2)));
    println!("{:?}", game.hit((4, 2)));
    println!("{:?}", game.hit((3, 1)));
    println!("{:?}", game.hit((0, 2)));
    println!("{:?}", game.hit((4, 1)));
}
