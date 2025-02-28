use crossterm::event::KeyEvent;
use std::collections::HashSet;

fn strip(event: &KeyEvent) -> KeyEvent {
    KeyEvent::new(event.code, event.modifiers)
}

#[derive(Default)]
pub struct ControlSet {
    set: HashSet<KeyEvent>,
}

impl ControlSet {
    pub fn new(keys: &Vec<KeyEvent>) -> Self {
        Self {
            set: keys.iter().map(|e| strip(e)).collect(),
        }
    }

    pub fn contains(&self, key: &KeyEvent) -> bool {
        self.set.contains(&strip(key))
    }
}

pub trait ControlCluster {
    fn contains(event: &KeyEvent) -> Option<Self>
    where
        Self: Sized;
}

#[macro_export]
macro_rules! control_cluster {
    ($cluster:ident, $(($path:ident, $set:expr)), *) => {
        impl cog_core::util::controls::ControlCluster for $cluster {
            fn contains(event: &KeyEvent) -> Option<Self> {
                $(if $set.contains(event) {
                    return Some($cluster::$path);
                })*

                None
            }
        }
    };
}
