use hecs::EntityBuilder;

use crate::components::{
    inventory::{player::PlayerInventory, Inventory},
    world::Position,
};

#[derive(Default)]
pub struct PlayerData;

pub fn player_builder(position: Position) -> EntityBuilder {
    let mut builder = EntityBuilder::new();

    builder
        .add(PlayerData)
        .add(Box::new(PlayerInventory::new(9, 128)) as Box<dyn Inventory>)
        .add(position);

    builder
}
