use std::{fmt::Display, str::FromStr};
use serde::{Deserialize, Serialize};
use crate::game::{board::Board, grid::Grid, hit_result::HitResult};

pub mod board;
pub mod boat;
pub mod grid;
pub mod hit_result;

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
            _ => Err(format!("{s} is not a role")),
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

#[derive(Debug, Clone, Copy, Serialize)]
pub enum GameStatus {
    Playing { turn: Role },
    GameOver { winner: Role },
}

#[derive(Debug)]
pub struct Game<const BOARD_WIDTH: usize = 8, const BOARD_HEIGHT: usize = 8> {
    host_board: Board<BOARD_WIDTH, BOARD_HEIGHT>,
    guest_board: Board<BOARD_WIDTH, BOARD_HEIGHT>,
    status: GameStatus,
}

impl<const BOARD_WIDTH: usize, const BOARD_HEIGHT: usize> Game<BOARD_WIDTH, BOARD_HEIGHT> {
    pub fn new(
        host_board: Board<BOARD_WIDTH, BOARD_HEIGHT>,
        guest_board: Board<BOARD_WIDTH, BOARD_HEIGHT>,
    ) -> Self {
        Self {
            host_board,
            guest_board,
            status: GameStatus::Playing { turn: Role::Guest },
        }
    }

    pub fn shoot(&mut self, position: (usize, usize)) -> HitResult {
        let (turn, board) = match self.status {
            GameStatus::Playing { turn: Role::Host } => (Role::Host, &mut self.guest_board),
            GameStatus::Playing { turn: Role::Guest } => (Role::Guest, &mut self.host_board),
            GameStatus::GameOver { .. } => unreachable!("shooting after game over"),
        };

        let hit_result = board.shoot(position);
        
        if hit_result == HitResult::Sunk && board.safe_boats_count() == 0 {
            self.status = GameStatus::GameOver { winner: turn };
        } else {
            self.status = GameStatus::Playing { turn };
        }

        hit_result
    }

    pub fn status(&self) -> GameStatus {
        self.status
    }

    pub fn spec_info(&self) -> SpectatorInfo<BOARD_WIDTH, BOARD_HEIGHT> {
        SpectatorInfo {
            host_hits: self.host_board.hits().clone(),
            guest_hits: self.guest_board.hits().clone(),
            status: self.status,
        }
    }
}

#[derive(Serialize)]
pub struct SpectatorInfo<const BOARD_WIDTH: usize = 8, const BOARD_HEIGHT: usize = 8> {
    host_hits: Grid<BOARD_WIDTH, BOARD_HEIGHT, Option<HitResult>>,
    guest_hits: Grid<BOARD_WIDTH, BOARD_HEIGHT, Option<HitResult>>,
    status: GameStatus,
}
