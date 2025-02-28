use cog_core::{control_cluster, util::controls::ControlSet};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

fn no_mods(code: KeyCode) -> KeyEvent {
    KeyEvent::new(code, KeyModifiers::NONE)
}

pub enum Basic {
    Left,
    Right,
    Up,
    Down,
    Select,
}

pub enum Actions {
    Back,
    Exit,
}

pub enum World {
    Interact,
    ZoomIn,
    ZoomOut,
}

control_cluster!(
    Basic,
    (
        Left,
        ControlSet::new(&vec![
            no_mods(KeyCode::Left),
            no_mods(KeyCode::Char('h')),
            no_mods(KeyCode::Char('a')),
        ])
    ),
    (
        Right,
        ControlSet::new(&vec![
            no_mods(KeyCode::Right),
            no_mods(KeyCode::Char('l')),
            no_mods(KeyCode::Char('d')),
        ])
    ),
    (
        Up,
        ControlSet::new(&vec![
            no_mods(KeyCode::Up),
            no_mods(KeyCode::Char('k')),
            no_mods(KeyCode::Char('w')),
        ])
    ),
    (
        Down,
        ControlSet::new(&vec![
            no_mods(KeyCode::Down),
            no_mods(KeyCode::Char('j')),
            no_mods(KeyCode::Char('s')),
        ])
    ),
    (Select, ControlSet::new(&vec![no_mods(KeyCode::Enter)]))
);

control_cluster!(
    Actions,
    (
        Back,
        ControlSet::new(&vec![no_mods(KeyCode::Char('q')), no_mods(KeyCode::Esc)])
    ),
    (
        Exit,
        ControlSet::new(&vec![KeyEvent::new(
            KeyCode::Char('c'),
            KeyModifiers::CONTROL
        )])
    )
);

control_cluster!(
    World,
    // = is + without shift
    (
        Interact,
        ControlSet::new(&vec![no_mods(KeyCode::Char(' '))])
    ),
    (ZoomIn, ControlSet::new(&vec![no_mods(KeyCode::Char('='))])),
    (ZoomOut, ControlSet::new(&vec![no_mods(KeyCode::Char('-'))]))
);
