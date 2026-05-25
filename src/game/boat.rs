use serde::Deserialize;

use crate::game::hit_result::HitResult;

#[derive(Debug, Deserialize)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Deserialize)]
pub struct Boat {
    pub(super) starting_position: (usize, usize),
    pub(super) direction: Direction,
    hits: Box<[bool]>,
}

impl Boat {
    pub fn new(starting_position: (usize, usize), direction: Direction, len: usize) -> Self {
        Self {
            starting_position,
            direction,
            hits: vec![false; len].into_boxed_slice(),
        }
    }

    pub fn hit(&mut self, position: (usize, usize)) -> HitResult {
        let index = match self.direction {
            Direction::North => {
                if position.0 != self.starting_position.0 {
                    return HitResult::Water;
                }
                assert!(position.1 >= self.starting_position.1);
                position.1 - self.starting_position.1
            }
            Direction::East => {
                if position.1 != self.starting_position.1 {
                    return HitResult::Water;
                }
                assert!(position.0 >= self.starting_position.0);
                position.0 - self.starting_position.0
            }
            Direction::South => {
                if position.0 != self.starting_position.0 {
                    return HitResult::Water;
                }
                assert!(position.1 <= self.starting_position.1);
                self.starting_position.1 - position.1
            }
            Direction::West => {
                if position.1 != self.starting_position.1 {
                    return HitResult::Water;
                }
                assert!(position.0 <= self.starting_position.0);
                self.starting_position.0 - position.0
            }
        };
        self.hits[index] = true;
        if self.hits.iter().copied().all(std::convert::identity) {
            HitResult::Sunk
        } else {
            HitResult::Hit
        }
    }

    pub fn len(&self) -> usize {
        self.hits.len()
    }
}
