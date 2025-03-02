use std::{cell::RefCell, rc::Rc};

use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;

use crate::components::world::{items::Item, World};

use super::entity::{player, pod, EntityData, EntityRegistry, EntityType, PLAYER_ID};

pub struct Store {
    pub rng: Xoshiro256PlusPlus,
    pub world: World,
    pub entities: EntityRegistry,
}

pub type RRStore = Rc<RefCell<Store>>;

impl Store {
    pub fn new(seed: u64) -> Self {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(seed);
        let mut world = World::new();
        let mut entities = EntityRegistry::default();

        world
            .grid
            .iter_mut()
            .filter_map(|cell| {
                let rand = rng.random_range(..4 as u8);

                Some((
                    if rand == 0 {
                        let id = (rng.random_range(..usize::MAX), EntityType::Pod);
                        entities.register(
                            id,
                            EntityData::Pod(pod::Data::new(match rng.random_range(..5_usize) {
                                0 => Item::RawCopper,
                                1 => Item::RawGold,
                                2 => Item::RawSilver,
                                3 => Item::RawIron,
                                4 => Item::RawTin,
                                _ => unreachable!(),
                            })),
                            rng.random(),
                            true,
                        );

                        Item::Pod(id)
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
            rng.random(),
            false,
        );

        Store {
            rng,
            world,
            entities,
        }
    }
}
