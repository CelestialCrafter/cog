use std::io::Write;

use crossterm::event::Event;
use eyre::Result;
use runtime::RuntimeMessage;

pub mod runtime;
pub mod ui;

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

#[macro_export]
macro_rules! generic_passthrough {
    ($msg:expr, $(($msg_path:path, $model:expr)), *) => {
        match $msg {
            AppMessage::Init => RuntimeMessage::Batch(vec![$($model.update(AppMessage::Init).map($msg_path)),*]),
            AppMessage::Event(event) => RuntimeMessage::Batch(vec![$($model.update(AppMessage::Event(event.clone())).map($msg_path)),*]),
            $(AppMessage::App($msg_path(message)) => $model.update(AppMessage::App(message)).map($msg_path)),*
        }
    };
}
