use std::{cell::RefCell, rc::Rc};

use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;

use crate::components::world::{items::Item, World};

use super::entity::{player, EntityData, EntityRegistry, PLAYER_ID};

pub struct Store {
    pub rng: Xoshiro256PlusPlus,
    pub world: World,
    pub entities: EntityRegistry,
}

pub type RRStore = Rc<RefCell<Store>>;

impl Store {
    pub fn new(seed: u64) -> Self {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(seed);
        let mut world = World::new(150);
        let mut entities = EntityRegistry::default();

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

        entities.register(
            PLAYER_ID,
            EntityData::Player(player::Data::default()),
            (
                rng.random_range(..world.size()),
                rng.random_range(..world.size()),
            ),
        );

        Store {
            rng,
            world,
            entities,
        }
    }
}
