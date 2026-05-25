use std::{collections::BTreeMap, sync::RwLock};

use crate::game::{Game, boat::Boat};

#[derive(Debug)]
pub enum GameState {
    Setup {
        challenger_boats: Vec<Boat>,
        challenged_boats: Vec<Boat>,
    },
    Playing(Game),
}

pub type GamesRef = RwLock<Games>;

#[derive(Debug)]
pub struct Games {
    pub(super) running_games: BTreeMap<usize, GameState>,
    pub(super) next_game: Option<usize>,
}

impl Games {
    pub fn new() -> GamesRef {
        GamesRef::new(Self {
            running_games: BTreeMap::new(),
            next_game: None,
        })
    }
}
