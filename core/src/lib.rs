use std::io::Write;

use crossterm::event::Event;
use eyre::Result;
use runtime::RuntimeMessage;

pub mod runtime;
pub mod util;

pub trait Model<T> {
    fn update(&mut self, message: AppMessage<T>) -> RuntimeMessage<T>;
    fn view(&self, writer: impl Write) -> Result<()>;
}

#[derive(Debug, Clone)]
pub enum AppMessage<T> {
    Event(Event),
    App(T),
    Init,
}
