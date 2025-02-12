use std::{cell::RefCell, io::Write, rc::Rc};

use cog_core::{
    runtime::RuntimeMessage,
    util::text::{
        border::Border,
        commands::PrintLines,
        join::{self},
        size::height,
    },
    AppMessage, Model,
};
use crossterm::{
    cursor::MoveTo,
    event::{Event, KeyCode, KeyEvent},
    queue,
    style::Stylize,
};
use eyre::Result;
use ndarray::Array1;

use crate::{store::Store, world::cells::Cell};

pub const SLOTS: usize = 9;

pub struct Inventory {
    selected: usize,
    slots: Array1<(usize, Cell)>,
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            selected: 0,
            slots: Array1::default(SLOTS),
        }
    }
}

pub struct InventoryModel {
    store: Rc<RefCell<Store>>,
    rows: u16,
    cols: u16,
}

impl InventoryModel {
    pub fn new(store: Rc<RefCell<Store>>) -> Self {
        Self {
            store,
            rows: 0,
            cols: 0,
        }
    }
}

pub enum InventoryMessage {}

impl Model<InventoryMessage> for InventoryModel {
    fn view(&self, mut writer: impl Write) -> Result<()> {
        let store = self.store.borrow();
        let output = Border::rounded().render(
            join::horizontal(
                join::CENTER,
                &store
                    .inventory
                    .slots
                    .view()
                    .into_iter()
                    .enumerate()
                    .map(|(i, (amount, item))| {
                        let mut inner = item.name.to_string();
                        inner = join::vertical(join::CENTER, &vec![inner, amount.to_string()]);
                        if i == store.inventory.selected {
                            inner = inner.blue().to_string();
                        }

                        Border::default()
                            .margin_individual(0, 0, 2, if i < SLOTS - 1 { 0 } else { 2 })
                            .render(inner.as_str())
                    })
                    .collect(),
            )
            .as_str(),
        );

        queue!(
            writer,
            MoveTo(0, self.rows.saturating_sub(height(output.as_str()) as u16)),
            PrintLines(output)
        )?;

        Ok(())
    }

    fn update(
        &mut self,
        message: AppMessage<InventoryMessage>,
    ) -> RuntimeMessage<InventoryMessage> {
        let mut store = self.store.borrow_mut();

        match message {
            AppMessage::Event(Event::Key(KeyEvent {
                code: KeyCode::Char(char),
                ..
            })) => match char {
                '1' => store.inventory.selected = 0,
                '2' => store.inventory.selected = 1,
                '3' => store.inventory.selected = 2,
                '4' => store.inventory.selected = 3,
                '5' => store.inventory.selected = 4,
                '6' => store.inventory.selected = 5,
                '7' => store.inventory.selected = 6,
                '8' => store.inventory.selected = 7,
                '9' => store.inventory.selected = 8,
                _ => (),
            },
            AppMessage::Event(Event::Resize(c, r)) => {
                self.rows = r;
                self.cols = c / 2;
            }
            _ => (),
        };

        RuntimeMessage::Empty
    }
}
