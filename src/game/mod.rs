use crate::game::{board::Board, hit_result::HitResult};

pub mod board;
pub mod grid;
pub mod hit_result;
pub mod boat;

#[derive(Clone, Copy)]
enum Turn {
    Challenger,
    Challenged,
}

impl std::ops::Not for Turn {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Turn::Challenger => Turn::Challenged,
            Turn::Challenged => Turn::Challenger,
        }
    }
}

pub struct Game<const BOARD_WIDTH: usize = 8, const BOARD_HEIGHT: usize = 8> {
    challenger_board: Board<BOARD_WIDTH, BOARD_HEIGHT>,
    challenged_board: Board<BOARD_WIDTH, BOARD_HEIGHT>,
    turn: Turn,
}

impl<const BOARD_WIDTH: usize, const BOARD_HEIGHT: usize> Game<BOARD_WIDTH, BOARD_HEIGHT> {
    pub fn new(challenger_board: Board<BOARD_WIDTH, BOARD_HEIGHT>, challenged_board: Board<BOARD_WIDTH, BOARD_HEIGHT>) -> Self {
        Self {
            challenger_board,
            challenged_board,
            turn: Turn::Challenged,
        }
    }

    pub fn hit(&mut self, position: (usize, usize)) -> HitResult {
        let board = match self.turn {
            Turn::Challenger => &mut self.challenged_board,
            Turn::Challenged => &mut self.challenger_board,
        };

        self.turn = !self.turn;

        board.hit(position)
    }
}
