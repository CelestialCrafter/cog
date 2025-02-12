use std::{cell::RefCell, io::Write, rc::Rc};

use cells::CellId;
use cog_core::{runtime::RuntimeMessage, AppMessage, Model};
use crossterm::{
    cursor::MoveToNextLine,
    event::{Event, KeyCode, KeyEvent},
    queue,
    style::{Color, PrintStyledContent, Stylize},
};
use eyre::Result;
use ndarray::s;

use crate::store::Store;

pub const SIZE: usize = 128;

pub mod cells {
    #[derive(Debug)]
    pub enum CellId {
        Empty,
        Pod,
        Belt,
    }

    pub struct Cell {
        pub id: CellId,
        pub name: &'static str,
    }

    impl Default for Cell {
        fn default() -> Self {
            EMPTY
        }
    }

    pub const EMPTY: Cell = Cell {
        name: "Empty",
        id: CellId::Empty,
    };

    pub const POD: Cell = Cell {
        name: "Pod",
        id: CellId::Pod,
    };

    pub const BELT: Cell = Cell {
        name: "Belt",
        id: CellId::Belt,
    };
}

pub struct WorldModel {
    store: Rc<RefCell<Store>>,
    rows: u16,
    cols: u16,
}

impl WorldModel {
    pub fn new(store: Rc<RefCell<Store>>) -> Self {
        Self {
            store,
            rows: 0,
            cols: 0,
        }
    }
}

pub enum WorldMessage {}

impl Model<WorldMessage> for WorldModel {
    fn view(&self, mut writer: impl Write) -> Result<()> {
        let store = self.store.borrow();
        let [pos_col, pos_row] = store.position;
        let rows = self.rows as usize;
        let cols = self.cols as usize;

        let rs = pos_row
            .saturating_sub(rows / 2)
            .min(SIZE.saturating_sub(rows));
        let re = (rs + rows).min(SIZE);

        let cs = pos_col
            .saturating_sub(cols / 2)
            .min(SIZE.saturating_sub(cols));
        let ce = (cs + cols).min(SIZE);

        let viewport = store.grid.slice(s![rs..re, cs..ce]);

        for (r, row) in viewport.rows().into_iter().enumerate() {
            for (c, cell) in row.iter().enumerate() {
                let styled = if (rs + r) == pos_row && (cs + c) == pos_col {
                    "  ".on_yellow()
                } else {
                    match cell.id {
                        CellId::Empty => "  ".stylize(),
                        CellId::Pod => "  ".on(Color::Grey),
                        CellId::Belt => "  ".on(Color::DarkGrey),
                    }
                };
                queue!(writer, PrintStyledContent(styled))?;
            }
            queue!(writer, MoveToNextLine(1))?;
        }

        Ok(())
    }

    fn update(&mut self, message: AppMessage<WorldMessage>) -> RuntimeMessage<WorldMessage> {
        match message {
            AppMessage::Event(Event::Key(KeyEvent { code, .. })) => {
                let mut store = self.store.borrow_mut();
                let mut c = store.position[0] as isize;
                let mut r = store.position[1] as isize;

                match code {
                    KeyCode::Left => c -= 1,
                    KeyCode::Right => c += 1,
                    KeyCode::Up => r -= 1,
                    KeyCode::Down => r += 1,
                    _ => (),
                }

                let c = c.min(SIZE as isize - 1).max(0) as usize;
                let r = r.min(SIZE as isize - 1).max(0) as usize;

                store.position = [c, r];
            }
            AppMessage::Event(Event::Resize(c, r)) => {
                self.rows = r;
                self.cols = c / 2;
            }
            _ => (),
        };
        RuntimeMessage::Empty
    }
}
