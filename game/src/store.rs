use std::{cell::RefCell, rc::Rc};

use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;

pub struct Store {
    pub rng: Xoshiro256PlusPlus,
}

pub type RRStore = Rc<RefCell<Store>>;

impl Store {
    pub fn new(seed: u64) -> Self {
        let rng = Xoshiro256PlusPlus::seed_from_u64(seed);

        Store {
            rng,
        }
    }
}
