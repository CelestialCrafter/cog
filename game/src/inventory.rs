use std::{cell::RefCell, rc::Rc};

use cog_core::{runtime::RuntimeMessage, AppMessage, Model};
use crossterm::event::{Event, KeyCode, KeyEvent};
use ndarray::Array1;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

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
    fn view(&mut self, frame: &mut Frame) {
        let store = self.store.borrow();

        let block = Block::bordered().border_type(BorderType::Rounded);
        let block_area = {
            let frame_area = frame.area();
            let height = 4;
            Rect::new(0, frame_area.height - height, SLOTS as u16 * 8, height).clamp(frame_area)
        };

        frame.render_widget(Clear, block_area);
        frame.render_widget(&block, block_area);

        let slots = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Ratio(1, SLOTS as u32); SLOTS])
            .split(block.inner(block_area));

        for (i, (amount, item)) in store.inventory.slots.view().into_iter().enumerate() {
            let mut style = Style::new();
            if i == store.inventory.selected {
                style = style.fg(Color::Blue);
            }

            frame.render_widget(
                Paragraph::new(format!("{}\n{}", item.name, amount))
                    .alignment(ratatui::layout::Alignment::Center)
                    .style(style),
                slots[i],
            );
        }
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
