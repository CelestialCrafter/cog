use std::{io::Write, rc::Rc};

use cog_core::{runtime::RuntimeMessage, AppMessage, Model};
use crossterm::{
    cursor::MoveToNextLine,
    queue,
    style::{Color, PrintStyledContent, Stylize},
};
use eyre::Result;

use crate::store::Store;

pub const WORLD_SIZE: usize = 128;

#[derive(Default)]
pub enum Cell {
    #[default]
    Empty,
    Full,
}

pub struct WorldModel {
    store: Rc<Store>,
}

impl WorldModel {
    pub fn new(store: Rc<Store>) -> Self {
        Self { store }
    }
}

pub enum WorldMessage {}

impl Model<WorldMessage> for WorldModel {
    fn view(&self, mut writer: impl Write) -> Result<()> {
        for row in self.store.grid.rows() {
            for cell in row {
                queue!(
                    writer,
                    PrintStyledContent(match cell {
                        Cell::Full => "  ".on(Color::Red),
                        Cell::Empty => "  ".stylize(),
                    })
                )?;
            }

            queue!(writer, MoveToNextLine(1))?;
        }
        Ok(())
    }

    fn update(&mut self, _message: AppMessage<WorldMessage>) -> RuntimeMessage<WorldMessage> {
        RuntimeMessage::Empty
    }
}
