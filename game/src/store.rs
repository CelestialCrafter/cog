use ndarray::{Array, Array2};
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;

use crate::world::{Cell, WORLD_SIZE};

pub struct Store {
    pub rng: Xoshiro256PlusPlus,
    pub grid: Array2<Cell>,
}

impl Store {
    pub fn new(seed: u64) -> Self {
        let mut store = Self {
            rng: Xoshiro256PlusPlus::seed_from_u64(seed),
            grid: Array::default((WORLD_SIZE, WORLD_SIZE)),
        };

        store.grid.iter_mut().for_each(|cell| {
            *cell = if store.rng.random_bool(0.3) {
                Cell::Full
            } else {
                Cell::Empty
            }
        });

        store
    }
}
