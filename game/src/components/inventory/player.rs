use crate::components::world::items::Item;
use std::iter::{once, repeat_n};

use super::{After, Amount, Before, Inventory, ModifyOperation, PrepareOperation, Slot};

pub struct PlayerInventory {
    pub slots: Box<[(Item, Amount)]>,
    pub preferred: Slot,
    item_limit: Amount,
}

impl PlayerInventory {
    pub fn new(slot_limit: usize, item_limit: Amount) -> Self {
        assert!(slot_limit > 0, "at least 1 slot should be available");

        Self {
            slots: repeat_n((Item::Empty, 0), slot_limit).collect(),
            preferred: 0,
            item_limit,
        }
    }
}

impl Inventory for PlayerInventory {
    fn slots(&self) -> Box<[&(Item, Amount)]> {
        self.slots.iter().collect()
    }

    fn prepare(&self, operation: PrepareOperation) -> Option<(ModifyOperation, Before, After)> {
        let slot;
        let item;
        let before;
        let after;

        let mut iter = once((
            self.preferred as usize,
            &self.slots[self.preferred as usize],
        ))
        .chain(self.slots.iter().enumerate());

        match operation {
            PrepareOperation::Add(op_item, amount) => {
                item = op_item;
                (slot, before, after) = iter.find_map(|(i, slot)| {
                    if slot.0 != item && slot.0 != Item::Empty  {
                        return None;
                    }

                    let after = slot.1.checked_add(amount)?;
                    if after > self.item_limit {
                        return None;
                    }

                    Some((i as Slot, slot.1, after))
                })?;
            }
            PrepareOperation::Remove(op_item, amount) => {
                (slot, item, before, after) = iter.find_map(|(i, slot)| {
                    if let Some(item) = op_item {
                        if slot.0 != item {
                            return None;
                        }
                    }

                    let after = match amount {
                        Some(amount) => slot.1.checked_sub(amount)?,
                        None => 0,
                    };

                    Some((i as Slot, slot.0, slot.1, after))
                })?;
            }
        }

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

    fn modify(&mut self, mut operation: ModifyOperation) {
        let slot = &mut self.slots[operation.slot as usize];

        if operation.amount < 1 {
            operation.item = Item::Empty;
        }
        *slot = (operation.item, operation.amount);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static ITEM: Item = Item::RawSilver;

    #[test]
    fn test_limit() {
        // test add past item limit
        assert!(
            PlayerInventory::new(1, 1)
                .prepare(PrepareOperation::Add(ITEM, 2))
                .is_none(),
            "add operation should not prepare with item limit exceeded"
        );

        // test add past slot limit
        let mut inventory = PlayerInventory::new(1, 1);
        inventory.modify(ModifyOperation {
            slot: 0,
            item: ITEM,
            amount: 1,
        });
        assert!(
            inventory.prepare(PrepareOperation::Add(ITEM, 1)).is_none(),
            "add operation should not prepare with no slots available"
        );
    }

    #[test]
    fn test_add() {
        // test add
        assert!(
            PlayerInventory::new(1, 1)
                .prepare(PrepareOperation::Add(ITEM, 1))
                .is_some(),
            "add operation should prepare"
        );
    }

    #[test]
    fn test_remove() {
        // test remove with none
        assert!(
            PlayerInventory::new(1, 1)
                .prepare(PrepareOperation::Remove(Some(ITEM), Some(1)))
                .is_none(),
            "remove operation should not prepare with no items"
        );

        // test remove max
        let mut inventory = PlayerInventory::new(1, 5);
        inventory.modify(ModifyOperation {
            item: ITEM,
            slot: 0,
            amount: 5,
        });
        let (ModifyOperation { amount, .. }, _, _) = inventory
            .prepare(PrepareOperation::Remove(Some(ITEM), None))
            .expect("reomve operation should prepare");
        assert_eq!(amount, 0, "removed amount was not max amount");

        // test slot cleanup
        let mut inventory = PlayerInventory::new(1, 1);
        inventory.modify(ModifyOperation {
            item: ITEM,
            slot: 0,
            amount: 0,
        });

        assert_eq!(
            inventory.slots[0].0,
            Item::Empty,
            "inventory should not contain item after none left",
        );
    }
}
