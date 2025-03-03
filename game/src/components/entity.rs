use belt::belt_tick;
use hecs::{Entity, Query, World};
use player::PlayerData;
use pod::pod_tick;

use super::store::Store;

pub mod belt;
pub mod player;
pub mod pod;

pub fn get_player<Q: Query>(entities: &mut World) -> (Entity, <Q as Query>::Item<'_>) {
    entities
        .query_mut::<Q>()
        .with::<&PlayerData>()
        .into_iter()
        .next()
        .expect("player should exist")
}

pub fn tick(store: &mut Store) {
    pod_tick(store);
    belt_tick(store);
}
