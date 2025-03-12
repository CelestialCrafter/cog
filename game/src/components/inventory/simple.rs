use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
};

use crate::components::world::items::Item;

use super::{After, Amount, Before, Inventory, ModifyOperation, Slot, VerifyOperation};

fn hash(v: impl Hash) -> u64 {
    let mut hasher = DefaultHasher::new();
    v.hash(&mut hasher);
    hasher.finish()
}

pub struct SimpleInventory {
    slots: HashMap<Slot, (Item, Amount)>,
    preferred: Option<Item>,
    limit: usize,
}

impl SimpleInventory {
    pub fn new(preferred: Option<Item>, limit: usize) -> Self {
        Self {
            slots: HashMap::default(),
            preferred,
            limit,
        }
    }
}

impl Inventory for SimpleInventory {
    fn slots(&self) -> Vec<&(Item, u64)> {
        self.slots.values().collect()
    }

    fn verify(&self, operation: VerifyOperation) -> Option<(ModifyOperation, Before, After)> {
        let slot;
        let item;
        let before;
        let after;

        match operation {
            VerifyOperation::Add(op_item, amount) => {
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
            VerifyOperation::Remove(op_item, amount) => {
                item = match op_item {
                    Some(i) => i,
                    None => self.preferred?,
                };
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
            SimpleInventory::new(None, 0)
                .verify(VerifyOperation::Add(Item::RawSilver, 1))
                .is_none(),
            "add operation should not verify with slot limit exceeded"
        );
    }

    #[test]
    fn test_add() {
        // test add
        assert!(
            SimpleInventory::new(None, 1)
                .verify(VerifyOperation::Add(Item::RawSilver, 1))
                .is_some(),
            "add operation should verify"
        );

        // test add over max
        let mut inventory = SimpleInventory::new(Some(Item::RawSilver), 1);
        inventory.modify(ModifyOperation {
            item: ITEM,
            slot: *SLOT,
            amount: Amount::MAX,
        });
        assert!(
            inventory
                .verify(VerifyOperation::Add(Item::RawSilver, 1))
                .is_none(),
            "should not be able to add beyond Amount::MAX"
        );
    }

    #[test]
    fn test_remove() {
        // test remove with none
        assert!(
            SimpleInventory::new(None, 1)
                .verify(VerifyOperation::Remove(Some(ITEM), Some(1)))
                .is_none(),
            "remove operation should not verify with no items"
        );

        // test remove preferred/max
        let mut inventory = SimpleInventory::new(Some(ITEM), 1);
        inventory.modify(ModifyOperation {
            item: ITEM,
            slot: *SLOT,
            amount: 5,
        });
        if let Some((op, _, _)) = inventory.verify(VerifyOperation::Remove(None, None)) {
            assert_eq!(op.item, ITEM, "removed item should be the preferred item");
            assert_eq!(op.amount, 0, "removed amount was not max amount");
        } else {
            panic!("remove operation should verify");
        };

        // test slot cleanup
        let mut inventory = SimpleInventory::new(Some(ITEM), 1);
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
