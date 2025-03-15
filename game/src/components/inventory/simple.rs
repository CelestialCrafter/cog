use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
};

use crate::components::world::items::Item;

use super::{After, Amount, Before, Inventory, ModifyOperation, PrepareOperation, Slot};

fn hash(v: impl Hash) -> u64 {
    let mut hasher = DefaultHasher::new();
    v.hash(&mut hasher);
    hasher.finish()
}

pub struct SimpleInventory {
    pub slots: HashMap<Slot, (Item, Amount)>,
    pub limit: usize,
}

impl SimpleInventory {
    pub fn new(limit: usize) -> Self {
        Self {
            slots: HashMap::default(),
            limit,
        }
    }
}

impl Inventory for SimpleInventory {
    fn slots(&self) -> Box<[&(Item, u64)]> {
        self.slots.values().collect()
    }

    fn prepare(&self, operation: PrepareOperation) -> Option<(ModifyOperation, Before, After)> {
        let slot;
        let item;
        let before;
        let after;

        match operation {
            PrepareOperation::Add(op_item, amount) => {
                item = op_item;
                slot = hash(item);

                before = match self.slots.get(&slot) {
                    Some((_, a)) => *a,
                    None => {
                        if self.slots.len() >= self.limit {
                            return None;
                        } else {
                            0
                        }
                    }
                };
                after = before.checked_add(amount)?;
            }
            PrepareOperation::Remove(op_item, amount) => {
                item = op_item?;
                slot = hash(item);

                before = self.slots.get(&slot)?.1;
                after = match amount {
                    Some(v) => before.checked_sub(v)?,
                    None => 0,
                };
            }
        };

        Some((
            ModifyOperation {
                slot,
                item,
                amount: after,
            },
            Before(before),
            After(after),
        ))
    }

    fn modify(&mut self, operation: ModifyOperation) {
        self.slots
            .entry(operation.slot)
            .or_insert((operation.item, 0))
            .1 = operation.amount;

        // clean up slot if no items left
        if operation.amount < 1 {
            self.slots.remove(&operation.slot);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use super::*;

    static ITEM: Item = Item::RawSilver;
    static SLOT: LazyLock<Slot> = LazyLock::new(|| hash(ITEM));

    #[test]
    fn test_limit() {
        // test add past limit
        assert!(
            SimpleInventory::new(0)
                .prepare(PrepareOperation::Add(Item::RawSilver, 1))
                .is_none(),
            "add operation should not prepare with slot limit exceeded"
        );
    }

    #[test]
    fn test_add() {
        // test add
        assert!(
            SimpleInventory::new(1)
                .prepare(PrepareOperation::Add(Item::RawSilver, 1))
                .is_some(),
            "add operation should prepare"
        );
    }

    #[test]
    fn test_remove() {
        // test remove with none
        assert!(
            SimpleInventory::new(1)
                .prepare(PrepareOperation::Remove(Some(ITEM), Some(1)))
                .is_none(),
            "remove operation should not prepare with no items"
        );

        // test remove max
        let mut inventory = SimpleInventory::new(1);
        inventory.modify(ModifyOperation {
            item: ITEM,
            slot: *SLOT,
            amount: 5,
        });

        let (ModifyOperation { amount, .. }, _, _) = inventory
            .prepare(PrepareOperation::Remove(Some(ITEM), None))
            .expect("remove operation should prepare");
        assert_eq!(amount, 0, "removed amount was not max amount");

        // test slot cleanup
        let mut inventory = SimpleInventory::new(1);
        inventory.modify(ModifyOperation {
            item: ITEM,
            slot: *SLOT,
            amount: 0,
        });

        assert!(
            !inventory.slots.contains_key(&SLOT),
            "inventory should not contain item after none left",
        );
    }
}
