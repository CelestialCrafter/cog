use std::collections::HashSet;

use crate::components::{inventory::simple::SimpleInventory, store::Store};

use super::EntityId;

#[derive(Debug)]
pub struct Data {
    pub inventory: SimpleInventory,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            inventory: SimpleInventory::new(None, 9)
        }
    }
}

pub fn tick(_store: &mut Store, _batch: HashSet<EntityId>) {
    unreachable!("Player is not tickable");
}
