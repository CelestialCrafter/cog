use hecs::EntityBuilder;

use crate::components::{
    inventory::{Inventory, VerifyOperation},
    store::Store,
    world::{Direction, Position},
};

pub struct PusherData;

pub fn pusher_builder(direction: Direction, position: Position) -> EntityBuilder {
    let mut builder = EntityBuilder::new();
    builder.add(PusherData).add(direction).add(position);
    builder
}

pub fn pusher_tick(store: &mut Store) {
    let entities: Vec<_> = store
        .entities
        .query_mut::<(&Direction, &Position)>()
        .with::<&PusherData>()
        .into_iter()
        .filter_map(|(_, (direction, position))| {
            let get = |dir: Direction| store.world.grid[position.move_by(dir, 1)?].entity();
            Some([*get(direction.flip())?, *get(*direction)?])
        })
        .collect();

    entities
        .into_iter()
        .filter_map(|ba| {
            let [b, a] = store
                .entities
                .query_many_mut::<&mut Box<dyn Inventory>, 2>(ba);
            b.ok()?.swap(a.ok()?, VerifyOperation::Remove(None, None));

            Some(())
        })
        .for_each(drop);
}
