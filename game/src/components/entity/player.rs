use crate::components::{inventory::simple::SimpleInventory, store::Store};

use super::EntityId;

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

pub fn update(store: &mut Store, batch: Vec<EntityId>) {}
