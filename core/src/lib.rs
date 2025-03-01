use std::{
    fmt::Debug,
    io::{stdout, Write},
    panic,
};

use crossterm::{event, execute, terminal};
use eyre::Result;
use log::{error, info};
use ratatui::{prelude::CrosstermBackend, Frame, Terminal};

use runtime::RuntimeMessage;

pub mod runtime;
pub mod util;

pub trait Model<T: Send + 'static> {
    fn update(&mut self, message: AppMessage<T>) -> RuntimeMessage<T>;
    fn view(&mut self, frame: &mut Frame);
}

#[derive(Debug, Clone)]
pub enum AppMessage<T> {
    Event(event::Event),
    App(T),
    Init,
}

pub fn restore() -> Result<()> {
    info!("restoring terminal");

    terminal::disable_raw_mode()?;
    execute!(
        stdout(),
        terminal::LeaveAlternateScreen,
        event::DisableMouseCapture
    )?;

    Ok(())
}

fn panic_hook() {
    let hook = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        if let Err(err) = restore() {
            error!("could not restore terminal: {}", err);
        }

        hook(info);
    }));
}

pub fn init<T: Write>(mut writer: T) -> Result<Terminal<CrosstermBackend<T>>> {
    info!("initializing cog");

    panic_hook();
    terminal::enable_raw_mode()?;
    execute!(
        writer,
        terminal::EnterAlternateScreen,
        event::EnableMouseCapture
    )?;

    Ok(Terminal::new(CrosstermBackend::new(writer))?)
}
