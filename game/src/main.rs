use std::{
    cell::RefCell,
    fs::OpenOptions,
    io::{stdout, Write},
    rc::Rc,
};

use cog_core::{
    init, restore,
    runtime::{event_loop, RuntimeMessage},
    AppMessage, Model,
};
use crossterm::style::{Color, Stylize};
use env_logger::{Builder, Target};
use eyre::Result;
use log::Level;
use ratatui::Frame;

use store::{RRStore, Store};

pub mod colors;
pub mod controls;
pub mod store;

#[derive(Debug)]
enum MainMessage {}

struct MainModel {}

impl MainModel {
    pub fn new(store: RRStore) -> Self {
        Self {}
    }
}

impl Model<MainMessage> for MainModel {
    fn view(&mut self, frame: &mut Frame) { }

    fn update(&mut self, message: AppMessage<MainMessage>) -> RuntimeMessage<MainMessage> {
        RuntimeMessage::Empty
    }
}

fn logging() -> Result<()> {
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("cog.log")?;

    Builder::from_default_env()
        .target(Target::Pipe(Box::new(file)))
        .format(|f, record| {
            let level = {
                let level = record.level();
                level.to_string().with(match level {
                    Level::Error => Color::Red,
                    Level::Warn => Color::Yellow,
                    Level::Info => Color::DarkGrey,
                    Level::Debug => Color::Magenta,
                    Level::Trace => Color::Blue,
                })
            };

            let target = record.target().with(colors::PRIMARY.into()).bold();

            writeln!(f, "{} {}: {}", level, target, record.args())
        })
        .try_init()?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
   logging()?;

    let store = Rc::new(RefCell::new(Store::new(44)));
    let term = init(stdout())?;
    event_loop(MainModel::new(store), term).await?;
    restore()
}
