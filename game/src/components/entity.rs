use std::collections::HashMap;

use crate::util::partition_n;

use super::{inventory::Inventory, store::Store, world::Position};

macro_rules! register_entities {
    ($(($variant:ident, $module:ident)),*) => {
        $(pub mod $module;)*

        #[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
        pub enum EntityType {
            $($variant,)*
            /// warning: internal - do not use
            __InternalLast
        }

        pub enum EntityData {
            $($variant($module::Data),)*
        }

        impl EntityData {
            pub fn inventory(&self) -> &dyn Inventory {
                match self {
                    $(EntityData::$variant(data) => &data.inventory,)*
                }
            }

            pub fn inventory_mut(&mut self) -> &mut dyn Inventory {
                match self {
                    $(EntityData::$variant(data) => &mut data.inventory,)*
                }
            }


            pub fn update(store: &mut Store, batch: impl Iterator<Item = EntityId>) {
                let partitioned: [Vec<EntityId>; EntityType::__InternalLast as usize] =
                    partition_n(batch, |id| id.1 as usize);

                for partition in partitioned {
                    match partition[..] {
                        $([(_, EntityType::$variant), ..] => $module::update(store, partition),)*
                        _ => (),
                    }
                }
            }
        }
    };
}

register_entities! {(Player, player)}

pub type EntityId = (usize, EntityType);
pub const PLAYER_ID: EntityId = (0, EntityType::Player);

#[derive(Default)]
pub struct EntityRegistry {
    pub data: HashMap<EntityId, EntityData>,
    pub position: HashMap<EntityId, Position>,
}

impl EntityRegistry {
    pub fn register(&mut self, id: EntityId, data: EntityData, position: Position) {
        self.data.insert(id, data);
        self.position.insert(id, position);
    }
}
