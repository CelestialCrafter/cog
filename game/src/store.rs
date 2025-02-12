use ndarray::{Array, Array2};
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;

use crate::world::{Cell, WORLD_SIZE};

pub struct Store {
    pub rng: Xoshiro256PlusPlus,
    pub position: [usize; 2],
    pub grid: Array2<Cell>,
}

impl Store {
    pub fn new(seed: u64) -> Self {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(seed);

        let position = [
            rng.random_range(..WORLD_SIZE),
            rng.random_range(..WORLD_SIZE),
        ];

        let mut grid = Array::default((WORLD_SIZE, WORLD_SIZE));
        grid.iter_mut().for_each(|cell| {
            *cell = if rng.random_bool(0.3) {
                Cell::Full
            } else {
                Cell::Empty
            }
        });

        Store {
            rng,
            position,
            grid,
        }
    }
}
