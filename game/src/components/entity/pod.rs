use hecs::EntityBuilder;
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::components::{
    inventory::{simple::SimpleInventory, Inventory, Operation},
    store::Store,
    world::{items::Item, Position},
};

pub struct PodData(pub Item);

pub fn pod_builder(resource: Item, position: Position) -> EntityBuilder {
    let mut builder = EntityBuilder::new();

    builder
        .add(PodData(resource))
        .add(Box::new(SimpleInventory::new(None, 1)) as Box<dyn Inventory>)
        .add(position);

    builder
}

pub fn pod_tick(store: &mut Store) {
    store
        .entities
        .query_mut::<(&PodData, &mut Box<dyn Inventory>)>()
        .with::<&Position>()
        .into_iter()
        .par_bridge()
        .for_each(|(_, (PodData(resource), inventory))| {
            if let Some(op) = inventory.verify(&Operation::Add(*resource, 1)) {
                inventory.modify(&op.1);
            }
        });
}
