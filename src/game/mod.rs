use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::game::{board::Board, hit_result::HitResult};

pub mod board;
pub mod grid;
pub mod hit_result;
pub mod boat;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Host,
    Guest,
}

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Host => write!(f, "host"),
            Self::Guest => write!(f, "guest"),
        }
    }
}

impl FromStr for Role {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Host" | "host" | "HOST" => Ok(Self::Host),
            "Guest" | "guest" | "GUEST" => Ok(Self::Guest),
            _ => Err(format!("{s} is not a role"))
        }
    }
}

impl std::ops::Not for Role {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Role::Host => Role::Guest,
            Role::Guest => Role::Host,
        }
    }
}

#[derive(Debug)]
pub struct Game<const BOARD_WIDTH: usize = 8, const BOARD_HEIGHT: usize = 8> {
    host_board: Board<BOARD_WIDTH, BOARD_HEIGHT>,
    guest_board: Board<BOARD_WIDTH, BOARD_HEIGHT>,
    turn: Role,
}

impl<const BOARD_WIDTH: usize, const BOARD_HEIGHT: usize> Game<BOARD_WIDTH, BOARD_HEIGHT> {
    pub fn new(host_board: Board<BOARD_WIDTH, BOARD_HEIGHT>, guest_board: Board<BOARD_WIDTH, BOARD_HEIGHT>) -> Self {
        Self {
            host_board,
            guest_board,
            turn: Role::Guest,
        }
    }

    pub fn shoot(&mut self, position: (usize, usize)) -> HitResult {
        let board = match self.turn {
            Role::Host => &mut self.guest_board,
            Role::Guest => &mut self.host_board,
        };

        self.turn = !self.turn;

        board.shoot(position)
    }

    pub fn turn(&self) -> Role {
        self.turn
    }
}
