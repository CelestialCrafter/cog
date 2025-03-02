use std::collections::{HashMap, HashSet};

use crate::components::{
    inventory::{simple::SimpleInventory, Inventory, Operation},
    store::Store,
    world::items::Item,
};

use super::{EntityData, EntityId};

#[derive(Debug)]
pub struct Data {
    pub inventory: SimpleInventory,
    pub resource: Item,
}

impl Data {
    pub fn new(resource: Item) -> Self {
        Self {
            inventory: SimpleInventory::new(None, 1),
            resource,
        }
    }
}

fn tick_one(data_map: &mut HashMap<EntityId, EntityData>, id: &EntityId) -> Option<()> {
    let inner = if let EntityData::Pod(inner) = data_map.get_mut(id)? {
        inner
    } else {
        unreachable!()
    };
    inner
        .inventory
        .modify(&Operation::Add(inner.resource), 1)?
        .1();
    Some(())
}

pub fn tick(store: &mut Store, batch: HashSet<EntityId>) {
    let data_map = &mut store.entities.data;
    for id in batch.iter() {
        tick_one(data_map, id);
    }
}
