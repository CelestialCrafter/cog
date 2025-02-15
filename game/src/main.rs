use std::{cell::RefCell, rc::Rc};

use cog_core::{
    generic_passthrough, init, restore,
    runtime::{event_loop, RuntimeMessage},
    AppMessage, Model,
};
use crossterm::event;
use eyre::Result;
use inventory::{InventoryMessage, InventoryModel};
use ratatui::Frame;
use store::Store;
use world::{WorldMessage, WorldModel};

pub mod inventory;
pub mod store;
pub mod world;

enum MainMessage {
    World(WorldMessage),
    Inventory(InventoryMessage),
}

struct MainModel {
    world_model: WorldModel,
    inventory_model: InventoryModel,
}

impl MainModel {
    pub fn new(store: Rc<RefCell<Store>>) -> Self {
        Self {
            world_model: WorldModel::new(store.clone()),
            inventory_model: InventoryModel::new(store),
        }
    }
}

impl Model<MainMessage> for MainModel {
    fn view(&mut self, frame: &mut Frame) {
        self.world_model.view(frame);
        self.inventory_model.view(frame);
    }

    fn update(&mut self, message: AppMessage<MainMessage>) -> RuntimeMessage<MainMessage> {
        let main_msg = match message {
            AppMessage::Event(event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Esc | event::KeyCode::Char('q'),
                ..
            })) => RuntimeMessage::Exit,
            _ => RuntimeMessage::Empty,
        };

        let msgs = generic_passthrough!(
            message,
            (MainMessage::Inventory, self.inventory_model),
            (MainMessage::World, self.world_model)
        );

        RuntimeMessage::Batch(vec![main_msg, msgs])
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let store = Rc::new(RefCell::new(Store::new(44)));
    event_loop(MainModel::new(store), init()?).await?;
    restore()
}
