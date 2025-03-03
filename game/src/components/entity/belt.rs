use hecs::EntityBuilder;
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::components::{
    inventory::{simple::SimpleInventory, Inventory, Operation},
    store::Store,
    world::{Direction, Position},
};

pub struct BeltData(Direction);

pub fn belt_builder(direction: Direction, position: Position) -> EntityBuilder {
    let mut builder = EntityBuilder::new();

    builder
        .add(BeltData(direction))
        .add(Box::new(SimpleInventory::new(None, 1)) as Box<dyn Inventory>)
        .add(position);

    builder
}

pub fn belt_tick(store: &mut Store) {
    let entities: Vec<_> = store
        .entities
        .query_mut::<(&BeltData, &Position)>()
        .with::<&Box<dyn Inventory>>()
        .into_iter()
        .par_bridge()
        .filter_map(|(belt, (BeltData(direction), position))| {
            let mut position = *position;
            position.move_by((*direction, 1));
            Some((belt, *store.world.grid[position].entity()?))
        })
        .collect();

    entities
        .into_iter()
        .filter_map(|(b, t)| {
            let bid = b.clone();
            let [b, t] = store
                .entities
                .query_many_mut::<&mut Box<dyn Inventory>, 2>([b, t]);

            let (b, t) = (b.ok()?, t.ok()?);

            let t_op = t.verify(&Operation::Remove(None, None))?;
            let b_op = b.verify(&Operation::Add(t_op.0 .0, t_op.0 .1))?;

            t.modify(&t_op.1);
            b.modify(&b_op.1);

            Some((bid, b.slots().clone()))
        })
        .for_each(drop);
}
