use std::{cell::RefCell, rc::Rc};

use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;

use crate::components::world::{items::Item, World};

use super::entity::{belt::belt_builder, player::player_builder, pod::pod_builder};

pub struct Store {
    pub rng: Xoshiro256PlusPlus,
    pub world: World,
    pub entities: hecs::World,
}

pub type RRStore = Rc<RefCell<Store>>;

impl Store {
    pub fn new(seed: u64) -> Self {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(seed);
        let mut world = World::new();
        let mut entities = hecs::World::new();

        world
            .grid
            .iter_mut()
            .filter_map(|cell| {
                let rand = rng.random_range(..5 as u8);
                Some((
                    match rand {
                        0 => Item::Pod(
                            entities.spawn(
                                pod_builder(
                                    match rng.random_range(..5_usize) {
                                        0 => Item::RawCopper,
                                        1 => Item::RawGold,
                                        2 => Item::RawSilver,
                                        3 => Item::RawIron,
                                        4 => Item::RawTin,
                                        _ => unreachable!(),
                                    },
                                    rng.random(),
                                )
                                .build(),
                            ),
                        ),
                        1 => Item::Belt(
                            entities.spawn(belt_builder(rng.random(), rng.random()).build()),
                        ),
                        2 => Item::RawGold,
                        _ => Item::Empty,
                    },
                    cell,
                ))
            })
            .for_each(|(random, cell)| *cell = random);

        entities.spawn(player_builder(rng.random()).build());

        Store {
            rng,
            world,
            entities,
        }
    }
}
