use std::{cmp::Ordering, collections::HashMap};

use crate::components::world::items::Item;

use super::{Inventory, Operation};

pub struct SimpleInventory {
    slots: HashMap<Item, usize>,
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
    fn slots(&self) -> &HashMap<Item, usize> {
        &self.slots
    }

    fn max_slots(&self) -> usize {
        self.limit
    }

    fn modify(
        &mut self,
        operation: &Operation,
        amount: usize,
    ) -> Option<(Item, Box<dyn FnOnce() + '_>)> {
        if let Operation::Add(item) = operation {
            if !self.slots.contains_key(item) && self.slots.len() >= self.limit {
                return None;
            }
        }

        let (item, slot_amount) = match operation {
            Operation::Add(item) => {
                let slot_amount = *self.slots.get(item).unwrap_or(&0);
                slot_amount.checked_add(amount)?;
                (*item, slot_amount)
            }
            Operation::Remove(Some(item)) => {
                let slot_amount = *self.slots.get(item)?;
                if slot_amount < amount {
                    return None;
                }

                (*item, slot_amount)
            }
            Operation::Remove(None) => {
                let mut slots_vec: Vec<_> = self.slots.iter().collect();
                if let Some(preferred) = self.preferred {
                    slots_vec.sort_unstable_by(|a, b| {
                        match (*a.0 == preferred, *b.0 == preferred) {
                            (true, false) => Ordering::Less,
                            (false, true) => Ordering::Greater,
                            _ => Ordering::Equal,
                        }
                    });
                }

                slots_vec
                    .into_iter()
                    .find(|(_, slot_amount)| **slot_amount >= amount)
                    .map(|(i, a)| (*i, *a))?
            }
        };

        Some((
            item,
            if let Operation::Add(_) = operation {
                Box::new(move || *self.slots.entry(item).or_default() = slot_amount + amount)
            } else {
                Box::new(move || {
                    let slot = self.slots.get_mut(&item).unwrap();
                    *slot = slot_amount - amount;
                    if *slot < 1 {
                        self.slots.remove(&item);
                    }
                })
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_limit() {
        assert!(
            SimpleInventory::new(None, 0)
                .modify(&Operation::Add(Item::RawSilver), 1)
                .is_none(),
            "item added even though slot limit exceeded"
        );
    }

    #[test]
    fn test_add() {
        let mut inventory = SimpleInventory::new(None, 1);
        inventory
            .modify(&Operation::Add(Item::RawSilver), 1)
            .expect("inventory add should succeed")
            .1();

        assert_eq!(
            *inventory
                .slots
                .get(&Item::RawSilver)
                .expect("item should exist in slots"),
            1,
            "item did not get added"
        )
    }

    #[test]
    fn test_add_overflow() {
        let mut inventory = SimpleInventory::new(None, 1);

        inventory
            .modify(&Operation::Add(Item::RawSilver), usize::MAX)
            .expect("inventory add should succeed")
            .1();

        assert!(
            inventory
                .modify(&Operation::Add(Item::RawSilver), 1)
                .is_none(),
            "should not be able to add beyond usize::MAX"
        );
    }

    #[test]
    fn test_remove() {
        let mut inventory = SimpleInventory::new(None, 1);

        inventory
            .modify(&Operation::Add(Item::RawSilver), 2)
            .expect("inventory add should succeed")
            .1();
        inventory
            .modify(&Operation::Remove(Some(Item::RawSilver)), 1)
            .expect("inventory remove should succeed")
            .1();

        assert_eq!(
            *inventory
                .slots
                .get(&Item::RawSilver)
                .expect("item should exist in slots"),
            1,
            "item did not get removed"
        );

        inventory
            .modify(&Operation::Remove(Some(Item::RawSilver)), 1)
            .expect("inventory remove should succeed")
            .1();

        assert!(
            !inventory.slots.contains_key(&Item::RawSilver),
            "inventory contains item after none left",
        );

        assert!(
            inventory
                .modify(&Operation::Remove(Some(Item::RawSilver)), 1)
                .is_none(),
            "item removed even though no items left"
        );
    }

    #[test]
    fn test_remove_preferred() {
        let mut inventory = SimpleInventory::new(Some(Item::RawSilver), 2);

        inventory
            .modify(&Operation::Add(Item::RawGold), 1)
            .expect("inventory add should succeed")
            .1();
        inventory
            .modify(&Operation::Add(Item::RawSilver), 1)
            .expect("inventory add should succeed")
            .1();

        let (item, commit) = inventory
            .modify(&Operation::Remove(None), 1)
            .expect("inventory remove should succeed");
        assert_eq!(
            item,
            Item::RawSilver,
            "removed item was not the preferred item"
        );
        commit();

        let item = inventory
            .modify(&Operation::Remove(None), 1)
            .expect("inventory remove should succeed")
            .0;
        assert_eq!(item, Item::RawGold, "item did not get removed");
    }
}
