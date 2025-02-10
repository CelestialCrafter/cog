use std::{
    io::{stdout, Write},
    time::Duration,
};

use cog_core::{
    runtime::{init, RuntimeMessage},
    AppMessage,
};
use crossterm::{
    cursor,
    event::{Event, KeyCode, KeyEvent, KeyModifiers},
    queue,
    style::Print,
};
use eyre::Result;

#[derive(Default)]
struct MainModel {
    last_event: Option<Event>,
    counter: u64,
    initialized: bool,
}

#[derive(Debug)]
enum Message {
    Increment(u64),
}

impl cog_core::Model<Message> for MainModel {
    fn update(&mut self, message: AppMessage<Message>) -> RuntimeMessage<Message> {
        match message {
            AppMessage::Event(event) => {
                if let Event::Key(KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers,
                    ..
                }) = event
                {
                    if modifiers.contains(KeyModifiers::CONTROL) {
                        return RuntimeMessage::Exit;
                    }
                }

                self.last_event = Some(event);
                return RuntimeMessage::Task(Box::pin(async {
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    RuntimeMessage::App(AppMessage::App(Message::Increment(1)))
                }));
            }
            AppMessage::App(Message::Increment(amount)) => self.counter += amount,
            AppMessage::Init => self.initialized = true,
        };

        RuntimeMessage::Empty
    }

    fn view(&self, mut writer: impl Write) -> Result<()> {
        queue!(
            writer,
            Print(format!("terminal event: {:?}", self.last_event)),
            cursor::MoveToNextLine(1),
            Print(format!("delayed count: {}", self.counter)),
            cursor::MoveToNextLine(1),
            Print(format!("initialized: {}", self.initialized))
        )?;

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    init(stdout(), MainModel::default()).await
}
