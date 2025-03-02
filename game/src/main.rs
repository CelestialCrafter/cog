use std::{
    cell::RefCell,
    fs::OpenOptions,
    io::{stdout, Write},
    rc::Rc,
};

use cog_core::{
    init, passthru, restore,
    runtime::{event_loop, RuntimeMessage},
    AppMessage, Model,
};
use components::{
    entity::{EntityRegistry, PLAYER_ID},
    inventory::InventoryWidget,
    store::{RRStore, Store},
    world::{WorldMessage, WorldModel},
};
use crossterm::{
    event::{Event, KeyCode, KeyEvent},
    style::{Color, Stylize},
};
use env_logger::{Builder, Target};
use eyre::Result;
use log::Level;
use ratatui::{layout::Rect, widgets::Widget, Frame};

pub mod colors;
pub mod components;
pub mod controls;
pub mod util;

#[derive(Debug)]
enum MainMessage {
    World(WorldMessage),
}

struct MainModel {
    world_model: WorldModel,
    store: RRStore,
}

impl MainModel {
    pub fn new(store: RRStore) -> Self {
        Self {
            world_model: WorldModel::new(store.clone()),
            store,
        }
    }
}

impl Model<MainMessage> for MainModel {
    fn view(&mut self, frame: &mut Frame) {
        self.world_model.view(frame);

        let store = self.store.borrow();
        let inventory = store.entities.data.get(&PLAYER_ID).unwrap().inventory();

        let height = 4;
        let area = frame.area();
        InventoryWidget::new(inventory).render(
            Rect::new(
                0,
                area.height - height,
                inventory.max_slots() as u16 * (height as f32 * 2.5) as u16,
                height,
            ).clamp(area),
            frame.buffer_mut(),
        );
    }

    fn update(&mut self, message: AppMessage<MainMessage>) -> RuntimeMessage<MainMessage> {
        if let AppMessage::Event(Event::Key(KeyEvent {
            code: KeyCode::Char('q'),
            ..
        })) = message
        {
            return RuntimeMessage::Exit;
        }

        passthru!(message, (MainMessage::World, self.world_model))
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
