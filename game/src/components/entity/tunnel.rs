use std::collections::{HashMap, VecDeque};

use hecs::{Entity, EntityBuilder, World};
use topological_sort::TopologicalSort;

use crate::components::{
    inventory::{simple::SimpleInventory, Inventory, Operation},
    store::Store,
    world::{Direction, Position},
};

pub struct TunnelData(Direction);

pub fn tunnel_builder(direction: Direction, position: Position) -> EntityBuilder {
    let mut builder = EntityBuilder::new();

    builder
        .add(TunnelData(direction))
        .add(Box::new(SimpleInventory::new(None, 1)) as Box<dyn Inventory>)
        .add(position);

    builder
}

pub fn sorted_tunnels(
    entities: &mut World,
) -> Option<(HashMap<Entity, Vec<Entity>>, VecDeque<Entity>)> {
    let mut topo = TopologicalSort::new();
    let entity_positions: HashMap<_, _> = entities
        .query_mut::<&Position>()
        .with::<&Box<dyn Inventory>>()
        .into_iter()
        .map(|(entity, pos)| (*pos, entity))
        .collect();
    let mut dependants: HashMap<Entity, Vec<_>> = HashMap::new();

    for (entity, (TunnelData(direction), tunnel_position)) in
        entities.query_mut::<(&TunnelData, &Position)>().into_iter()
    {
        if let Some(&other) = tunnel_position
            .move_by(direction.flip(), 1)
            .and_then(|pos| entity_positions.get(&pos))
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
    let (dependants, tunnels) = if let Some(t) = sorted_tunnels(&mut store.entities) {
        t
    } else {
        return;
    };

    tunnels
        .iter()
        .filter_map(|entity| Some((entity, dependants.get(entity)?.first().unwrap())))
        .filter_map(|(tunnel, dep)| {
            let [tunnel, dep] = store
                .entities
                .query_many_mut::<&mut Box<dyn Inventory>, 2>([*tunnel, *dep]);

            let (tunnel, dep) = (tunnel.ok()?, dep.ok()?);

            let t_op = tunnel.verify(&Operation::Remove(None, None))?;
            let d_op = dep.verify(&Operation::Add(t_op.0 .0, t_op.0 .1))?;

            tunnel.modify(&t_op.1);
            dep.modify(&d_op.1);

            Some(())
        })
        .for_each(drop);
}
