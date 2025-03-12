use std::collections::{HashMap, VecDeque};

use hecs::{Entity, EntityBuilder};
use topological_sort::TopologicalSort;

use crate::components::{
    inventory::{Inventory, VerifyOperation, simple::SimpleInventory},
    store::Store,
    world::{Direction, Position},
};

pub struct TunnelData;

pub fn tunnel_builder(direction: Direction, position: Position) -> EntityBuilder {
    let mut builder = EntityBuilder::new();

    builder
        .add(TunnelData)
        .add(direction)
        .add(Box::new(SimpleInventory::new(None, 1)) as Box<dyn Inventory>)
        .add(position);

    builder
}

pub fn sorted_tunnels(
    store: &mut Store,
) -> Option<(HashMap<Entity, Vec<Entity>>, VecDeque<Entity>)> {
    let mut topo = TopologicalSort::new();
    let mut dependants: HashMap<Entity, Vec<_>> = HashMap::new();

    for (entity, (direction, tunnel_position)) in store
        .entities
        .query_mut::<(&Direction, &Position)>()
        .with::<&TunnelData>()
        .into_iter()
    {
        if let Some(&other) = tunnel_position
            .move_by(direction.flip(), 1)
            .and_then(|pos| store.world.grid[pos].entity())
        {
            dependants.entry(other).or_default().push(entity);
            topo.add_dependency(entity, other);
        }
    }

    let mut order = VecDeque::with_capacity(topo.len());
    while let Some(popped) = topo.pop() {
        order.push_front(popped);
    }

    // cycle detected
    if topo.len() != 0 {
        return None;
    }

    Some((dependants, order))
}

pub fn tunnel_tick(store: &mut Store) {
    let (dependants, tunnels) = if let Some(t) = sorted_tunnels(store) {
        t
    } else {
        return;
    };

    tunnels
        .iter()
        .filter_map(|&t| {
            let [t, d] = store
                .entities
                .query_many_mut::<&mut Box<dyn Inventory>, 2>([
                    t,
                    *dependants.get(&t)?.first().unwrap(),
                ]);
            t.ok()?.swap(d.ok()?, VerifyOperation::Remove(None, None))?;

            Some(())
        })
        .for_each(drop);
}
