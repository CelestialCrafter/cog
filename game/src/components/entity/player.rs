use hecs::EntityBuilder;

use crate::components::{
    inventory::{simple::SimpleInventory, Inventory},
    world::Position,
};

#[derive(Default)]
pub struct PlayerData;

pub fn player_builder(position: Position) -> EntityBuilder {
    let mut builder = EntityBuilder::new();

    builder
        .add(PlayerData)
        .add(Box::new(SimpleInventory::new(None, 9)) as Box<dyn Inventory>)
        .add(position);

    builder
}
