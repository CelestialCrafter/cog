use std::{cmp::Ordering, collections::HashMap};

use crate::components::world::items::Item;

use super::{After, Before, Inventory, Operation};

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

    fn verify(&self, operation: &Operation) -> Option<(Before, After)> {
        if let Operation::Add(item, _) = operation {
            if !self.slots.contains_key(item) && self.slots.len() >= self.limit {
                return None;
            }
        }

        let (item, before_amount, after_amount) = match operation {
            Operation::Add(item, amount) => {
                let sa = self.slots.get(item).unwrap_or(&0);
                (item, sa, sa.checked_add(*amount)?)
            }
            Operation::Remove(Some(item), amount) => {
                let sa = self.slots.get(item)?;
                let var_name = match amount {
                    Some(amount) => (sa, sa.checked_sub(*amount)?),
                    None => (sa, 0),
                };
                (item, var_name.0, var_name.1)
            }
            Operation::Remove(None, amount) => {
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

                match amount {
                    Some(amount) => {
                        let (i, sa) = slots_vec.into_iter().find(|(_, sa)| *sa >= amount)?;
                        (i, sa, sa.checked_sub(*amount)?)
                    }
                    None => {
                        let (i, a) = slots_vec.first()?;
                        (*i, *a, 0)
                    }
                }
            }
        };

        Some((Before(*item, *before_amount), After(*item, after_amount)))
    }

    fn modify(&mut self, operation: &After) {
        let slot = self.slots.entry(operation.0).or_default();
        *slot = operation.1;
        if *slot < 1 {
            self.slots.remove(&operation.0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_limit() {
        assert!(
            SimpleInventory::new(None, 0)
                .verify(&Operation::Add(Item::RawSilver, 1))
                .is_none(),
            "add operation should not verify with slot limit exceeded"
        );
    }

    #[test]
    fn test_add() {
        assert!(
            SimpleInventory::new(None, 1)
                .verify(&Operation::Add(Item::RawSilver, 1))
                .is_some(),
            "add operation should verify"
        );

        let mut inventory = SimpleInventory::new(Some(Item::RawSilver), 1);
        inventory.modify(&After(Item::RawSilver, usize::MAX));
        assert!(
            inventory
                .verify(&Operation::Add(Item::RawSilver, 1))
                .is_none(),
            "should not be able to add beyond usize::MAX"
        );
    }

    #[test]
    fn test_remove() {
        assert!(
            SimpleInventory::new(None, 1)
                .verify(&Operation::Remove(Some(Item::RawSilver), Some(1)))
                .is_none(),
            "remove operation should not verify with no items"
        );

        let mut inventory = SimpleInventory::new(Some(Item::RawSilver), 1);
        inventory.modify(&After(Item::RawSilver, 5));
        if let Some((_, After(item, amount))) = inventory.verify(&Operation::Remove(None, None)) {
            assert_eq!(
                item,
                Item::RawSilver,
                "removed item should be the preferred item"
            );
            assert_eq!(amount, 0, "removed amount was not max amount");
        } else {
            panic!("remove operation should verify");
        };

        inventory.modify(&After(Item::RawSilver, 0));
        assert!(
            !inventory.slots.contains_key(&Item::RawSilver),
            "inventory should not contain item after none left",
        );
    }
}
