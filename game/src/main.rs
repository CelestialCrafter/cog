use std::{
    cell::RefCell,
    fs::OpenOptions,
    io::{stdout, Write},
    rc::Rc,
    time::Duration,
};

use cog_core::{
    init, passthru, restore,
    runtime::{event_loop, RuntimeMessage},
    util::app_message,
    AppMessage, Model,
};
use components::{
    entity::{get_player, tick},
    inventory::{Inventory, InventoryWidget},
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
    Tick,
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

        let mut store = self.store.borrow_mut();
        let (_, inventory) =
            get_player::<&Box<dyn Inventory>>(&mut store.entities).expect("player should exist");

        let height = 4;
        let area = frame.area();
        InventoryWidget::new(inventory.as_ref()).render(
            Rect::new(
                0,
                area.height - height,
                inventory.slots().len() as u16 * (height as f32 * 2.5) as u16,
                height,
            )
            .clamp(area),
            frame.buffer_mut(),
        );
    }

    fn update(&mut self, message: AppMessage<MainMessage>) -> RuntimeMessage<MainMessage> {
        match message {
            AppMessage::Event(Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            })) => return RuntimeMessage::Exit,
            AppMessage::Init => app_message(MainMessage::Tick),
            AppMessage::App(MainMessage::Tick) => {
                tick(&mut self.store.borrow_mut());
                RuntimeMessage::Task(Box::pin(async {
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    app_message(MainMessage::Tick)
                }))
            }
            _ => passthru!(message, (MainMessage::World, self.world_model)),
        }
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
