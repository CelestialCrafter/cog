use std::{
    io::{stdout, Write},
    rc::Rc,
};

use cog_core::{runtime::RuntimeMessage, AppMessage, Model};
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
        match message {
            AppMessage::Event(event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('c'),
                modifiers,
                ..
            })) => {
                if modifiers.contains(event::KeyModifiers::CONTROL) {
                    RuntimeMessage::Exit
                } else {
                    RuntimeMessage::Empty
                }
            }
            AppMessage::Init => self
                .world_model
                .update(AppMessage::Init)
                .map(MainMessage::World),
            _ => RuntimeMessage::Empty,
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let store = Rc::new(Store::new(44));
    cog_core::runtime::init::<MainMessage>(stdout(), MainModel::new(store)).await
}
