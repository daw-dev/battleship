use rocket::request::FromParam;
use serde::{Deserialize, Serialize};

use crate::game::{board::Board, hit_result::HitResult};

pub mod board;
pub mod grid;
pub mod hit_result;
pub mod boat;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Player {
    Challenger,
    Challenged,
}

impl std::ops::Not for Player {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Player::Challenger => Player::Challenged,
            Player::Challenged => Player::Challenger,
        }
    }
}

impl<'a> FromParam<'a> for Player {
    type Error = ();

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        match param {
            "Challenger" | "challenger" | "CHALLENGER" => Ok(Player::Challenger),
            "Challenged" | "challenged" | "CHALLENGED" => Ok(Player::Challenged),
            _ => Err(())
        }
    }
}

#[derive(Debug)]
pub struct Game<const BOARD_WIDTH: usize = 8, const BOARD_HEIGHT: usize = 8> {
    challenger_board: Board<BOARD_WIDTH, BOARD_HEIGHT>,
    challenged_board: Board<BOARD_WIDTH, BOARD_HEIGHT>,
    turn: Player,
}

impl<const BOARD_WIDTH: usize, const BOARD_HEIGHT: usize> Game<BOARD_WIDTH, BOARD_HEIGHT> {
    pub fn new(challenger_board: Board<BOARD_WIDTH, BOARD_HEIGHT>, challenged_board: Board<BOARD_WIDTH, BOARD_HEIGHT>) -> Self {
        Self {
            challenger_board,
            challenged_board,
            turn: Player::Challenged,
        }
    }

    pub fn hit(&mut self, position: (usize, usize)) -> HitResult {
        let board = match self.turn {
            Player::Challenger => &mut self.challenged_board,
            Player::Challenged => &mut self.challenger_board,
        };

        self.turn = !self.turn;

        board.hit(position)
    }
}
