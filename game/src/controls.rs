use std::{collections::HashSet, sync::LazyLock};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Default)]
pub struct ControlSet {
    set: HashSet<KeyEvent>,
}

fn strip(event: &KeyEvent) -> KeyEvent {
    KeyEvent::new(event.code, event.modifiers)
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

fn no_mods(code: KeyCode) -> KeyEvent {
    KeyEvent::new(code, KeyModifiers::NONE)
}

pub struct Directonal {
    pub left: ControlSet,
    pub right: ControlSet,
    pub up: ControlSet,
    pub down: ControlSet,
}

pub static DIRECTONAL: LazyLock<Directonal> = LazyLock::new(|| Directonal {
    left: ControlSet::new(&vec![
        no_mods(KeyCode::Left),
        no_mods(KeyCode::Char('h')),
        no_mods(KeyCode::Char('a')),
    ]),
    right: ControlSet::new(&vec![
        no_mods(KeyCode::Right),
        no_mods(KeyCode::Char('l')),
        no_mods(KeyCode::Char('d')),
    ]),
    up: ControlSet::new(&vec![
        no_mods(KeyCode::Up),
        no_mods(KeyCode::Char('k')),
        no_mods(KeyCode::Char('w')),
    ]),
    down: ControlSet::new(&vec![
        no_mods(KeyCode::Down),
        no_mods(KeyCode::Char('j')),
        no_mods(KeyCode::Char('s')),
    ]),
});

pub static QUIT: LazyLock<ControlSet> = LazyLock::new(|| {
    ControlSet::new(&vec![
        KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL),
        no_mods(KeyCode::Char('q')),
        no_mods(KeyCode::Esc),
    ])
});
