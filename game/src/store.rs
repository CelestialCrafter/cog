use ndarray::{Array, Array2};
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;

use crate::{
    inventory::Inventory,
    world::{cells, SIZE},
};

pub struct Store {
    pub rng: Xoshiro256PlusPlus,
    pub position: [usize; 2],
    pub grid: Array2<cells::Cell>,
    pub inventory: Inventory,
}

impl Store {
    pub fn new(seed: u64) -> Self {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(seed);

        let position = [rng.random_range(..SIZE), rng.random_range(..SIZE)];

        let mut grid = Array::default((SIZE, SIZE));
        grid.iter_mut()
            .filter_map(|cell| {
                let rand = rng.random_range(..4 as u8);

                Some((
                    if rand == 0 {
                        cells::BELT
                    } else if rand == 1 {
                        cells::POD
                    } else {
                        cells::EMPTY
                    },
                    cell,
                ))
            })
            .for_each(|(random, cell)| *cell = random);

        Store {
            rng,
            position,
            grid,
            inventory: Inventory::default(),
        }
    }
}
