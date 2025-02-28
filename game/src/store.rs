use std::{cell::RefCell, rc::Rc};

use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;

use crate::components::world::{items::Item, World};

pub struct Store {
    pub rng: Xoshiro256PlusPlus,
    pub world: World,
}

pub type RRStore = Rc<RefCell<Store>>;

impl Store {
    pub fn new(seed: u64) -> Self {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(seed);
        let mut world = World::new(150);

        world
            .grid
            .iter_mut()
            .filter_map(|cell| {
                let rand = rng.random_range(..4 as u8);

                Some((
                    if rand == 0 {
                        Item::RawSilver
                    } else if rand == 1 {
                        Item::RawGold
                    } else {
                        Item::Empty
                    },
                    cell,
                ))
            })
            .for_each(|(random, cell)| *cell = random);

        Store { rng, world }
    }
}
