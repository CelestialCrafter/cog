use std::{io::Write, rc::Rc};

use cog_core::{
    runtime::{self, RuntimeMessage},
    AppMessage, Model,
};
use crossterm::event;
use eyre::Result;
use store::Store;
use world::{WorldMessage, WorldModel};

pub mod store;
pub mod world;

enum MainMessage {
    World(WorldMessage),
}

impl From<WorldMessage> for MainMessage {
    fn from(message: WorldMessage) -> Self {
        MainMessage::World(message)
    }
}

struct MainModel {
    world_model: WorldModel,
}

impl MainModel {
    pub fn new(store: Rc<Store>) -> Self {
        Self {
            world_model: WorldModel::new(store),
        }
    }
}

impl Model<MainMessage> for MainModel {
    fn view(&self, writer: impl Write) -> Result<()> {
        self.world_model.view(writer)
    }

    fn update(&mut self, message: AppMessage<MainMessage>) -> RuntimeMessage<MainMessage> {
        let main_msg = match message {
            AppMessage::Event(event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('c'),
                modifiers,
                ..
            })) => {
                if modifiers.contains(event::KeyModifiers::CONTROL) {
                    Some(RuntimeMessage::Exit)
                } else {
                    None
                }
            }
            _ => None,
        }
        .unwrap_or(RuntimeMessage::Empty);

        let world_msg = self
            .world_model
            .update(match message {
                AppMessage::Init => AppMessage::Init,
                AppMessage::Event(event) => AppMessage::Event(event),
                AppMessage::App(MainMessage::World(message)) => AppMessage::App(message),
            })
            .map(MainMessage::World);

        RuntimeMessage::Batch(vec![main_msg, world_msg])
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let store = Rc::new(Store::new(44));
    runtime::init::<MainMessage>(MainModel::new(store)).await
}
