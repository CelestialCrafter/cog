use std::{cell::RefCell, rc::Rc};

use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;

use crate::components::world::{World, items::Item};

use super::entity::{player::player_builder, pod::pod_builder, tunnel::tunnel_builder};

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
            .indexed_iter_mut()
            .filter_map(|(pos, cell)| {
                let rand = rng.random_range(..5 as u8);
                let pos = pos.into();

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
                                    pos,
                                )
                                .build(),
                            ),
                        ),
                        1 => {
                            Item::Tunnel(entities.spawn(tunnel_builder(rng.random(), pos).build()))
                        }
                        _ => Item::Empty,
                    },
                    cell,
                ))
            })
            .for_each(|(random, cell)| *cell = random);

        {
            let position = rng.random();
            entities.spawn(player_builder(position).build());
            world.cursor = position;
        }

        Store {
            rng,
            world,
            entities,
        }
    }
}
