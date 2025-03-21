use tunnel::tunnel_tick;
use hecs::{Entity, Query, World};
use player::PlayerData;
use pod::pod_tick;

use super::store::Store;

pub mod tunnel;
pub mod player;
pub mod pod;

pub fn get_player<Q: Query>(entities: &mut World) -> Option<(Entity, <Q as Query>::Item<'_>)> {
    entities
        .query_mut::<Q>()
        .with::<&PlayerData>()
        .into_iter()
        .next()
}

pub fn tick(store: &mut Store) {
    pod_tick(store);
    tunnel_tick(store);
}
