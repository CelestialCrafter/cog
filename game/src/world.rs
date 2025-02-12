use std::{io::Write, rc::Rc};

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

pub const WORLD_SIZE: usize = 128;

#[derive(Default, Debug)]
pub enum Cell {
    #[default]
    Empty,
    Full,
}

pub struct WorldModel {
    store: Rc<Store>,
    rows: u16,
    cols: u16,
}

impl WorldModel {
    pub fn new(store: Rc<Store>) -> Self {
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
        let [pos_col, pos_row] = self.store.position;
        let rows = self.rows as usize;
        let cols = self.cols as usize;

        let rs = pos_row
            .saturating_sub(rows / 2)
            .min(WORLD_SIZE.saturating_sub(rows));
        let cs = pos_col
            .saturating_sub(cols / 2)
            .min(WORLD_SIZE.saturating_sub(cols));

        let viewport = self.store.grid.slice(s![rs..rs + rows, cs..cs + cols]);

        for (r, row) in viewport.rows().into_iter().enumerate() {
            for (c, cell) in row.iter().enumerate() {
                let styled = if (rs + r) == pos_row && (cs + c) == pos_col {
                    "  ".on_yellow()
                } else {
                    match cell {
                        Cell::Full => "  ".on(Color::Red),
                        Cell::Empty => "  ".stylize(),
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
                let mut c = self.store.position[0] as isize;
                let mut r = self.store.position[1] as isize;

                match code {
                    KeyCode::Left => c -= 1,
                    KeyCode::Right => c += 1,
                    KeyCode::Up => r -= 1,
                    KeyCode::Down => r += 1,
                    _ => (),
                }

                let c = c.min(WORLD_SIZE as isize - 1).max(0) as usize;
                let r = r.min(WORLD_SIZE as isize - 1).max(0) as usize;

                let store = Rc::get_mut(&mut self.store).unwrap();
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
