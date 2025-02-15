use std::{io::stdout, panic};

use crossterm::{event, execute, terminal};
use eyre::Result;
use ratatui::{prelude::CrosstermBackend, DefaultTerminal, Frame, Terminal};

use runtime::RuntimeMessage;

pub mod runtime;
pub mod util;

pub trait Model<T> {
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
            eprintln!("could not restore terminal: {}", err);
        }

        hook(info);
    }));
}

pub fn init() -> Result<DefaultTerminal> {
    panic_hook();
    terminal::enable_raw_mode()?;
    execute!(
        stdout(),
        terminal::EnterAlternateScreen,
        event::EnableMouseCapture
    )?;

    Ok(Terminal::new(CrosstermBackend::new(stdout()))?)
}
