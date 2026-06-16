use std::{
    cell::RefCell,
    sync::{Arc, RwLock},
};

use crate::game::{
    boat::{Boat, Direction},
    grid::Grid,
    hit_result::HitResult,
};

type BoatRef = Arc<RwLock<Boat>>;

#[derive(Debug)]
pub struct Board<const W: usize = 8, const H: usize = 8> {
    hits: Grid<W, H, bool>,
    boat_refs: Grid<W, H, Option<BoatRef>>,
    boats: Vec<BoatRef>,
}

impl<const W: usize, const H: usize> Board<W, H> {
    pub fn shoot(&mut self, position: (usize, usize)) -> HitResult {
        self.hits[position] = true;
        self.boat_refs[position]
            .as_ref()
            .map(|boat| boat.write().unwrap().hit(position))
            .unwrap_or(HitResult::Water)
    }

    pub fn new(boats: Vec<Boat>) -> Self {
        let boats: Vec<_> = boats.into_iter().map(RwLock::new).map(Arc::new).collect();
        let mut boat_refs = Grid::new();
        for boat in boats.iter() {
            let boat_borrow = boat.read().unwrap();
            let positions: Vec<_> = match boat_borrow.direction {
                Direction::North => (0..boat_borrow.len())
                    .map(|i| {
                        let mut position = boat_borrow.starting_position;
                        position.1 += i;
                        position
                    })
                    .collect(),
                Direction::East => (0..boat_borrow.len())
                    .map(|i| {
                        let mut position = boat_borrow.starting_position;
                        position.0 += i;
                        position
                    })
                    .collect(),
                Direction::South => (0..boat_borrow.len())
                    .map(|i| {
                        let mut position = boat_borrow.starting_position;
                        position.1 -= i;
                        position
                    })
                    .collect(),
                Direction::West => (0..boat_borrow.len())
                    .map(|i| {
                        let mut position = boat_borrow.starting_position;
                        position.0 -= i;
                        position
                    })
                    .collect(),
            };

            for position in positions.into_iter() {
                boat_refs[position] = Some(boat.clone());
            }
        }

        Self {
            hits: Grid::new(),
            boat_refs,
            boats,
        }
    }

    pub fn safe_boats_count(&self) -> usize {
        self.boats
            .iter()
            .filter_map(|boat| {
                let boat = boat.read().unwrap();
                boat.is_safe().then_some(())
            })
            .count()
    }
}
