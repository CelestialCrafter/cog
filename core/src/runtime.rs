use std::io::Write;

use crossterm::{cursor, event, execute, queue, terminal};
use eyre::Result;
use futures::{channel::mpsc, future::BoxFuture, stream::select, SinkExt, StreamExt};

use crate::{AppMessage, Model};

pub enum RuntimeMessage<T> {
    Empty,
    Exit,
    Task(BoxFuture<'static, RuntimeMessage<T>>),
    App(AppMessage<T>),
}

async fn event_loop<T: Send + 'static>(
    mut writer: impl Write,
    mut model: impl Model<T>,
) -> Result<()> {
    let (mut msg_tx, msgs) = mpsc::unbounded();
    let msgs = msgs.map(|m| Ok(m));
    let events =
        event::EventStream::new().map(|e| e.map(|e| RuntimeMessage::App(AppMessage::Event(e))));
    let mut combined = select(msgs, events);

    msg_tx.send(RuntimeMessage::App(AppMessage::Init)).await?;

    while let Some(message) = combined.next().await {
        let msg = message?;
        match msg {
            RuntimeMessage::Exit => break,
            RuntimeMessage::Empty => (),
            RuntimeMessage::App(msg) => {
                let out_msg = model.update(msg);
                msg_tx.send(out_msg).await?;
                queue!(
                    writer,
                    terminal::BeginSynchronizedUpdate,
                    terminal::Clear(terminal::ClearType::All),
                    cursor::MoveTo(0, 0),
                )?;
                model.view(&mut writer)?;
                queue!(writer, terminal::EndSynchronizedUpdate)?;
                writer.flush()?;
            }
            RuntimeMessage::Task(task) => {
                let mut msg_tx = msg_tx.clone();
                tokio::task::spawn(async move { msg_tx.send(task.await).await });
            }
        };
    }
    Ok(())
}

pub async fn init<T: Send + 'static>(mut writer: impl Write, model: impl Model<T>) -> Result<()> {
    terminal::enable_raw_mode()?;
    execute!(
        writer,
        cursor::Hide,
        terminal::EnterAlternateScreen,
        terminal::DisableLineWrap,
        event::EnableMouseCapture
    )?;

    let result = event_loop(&mut writer, model).await;

    execute!(
        writer,
        cursor::Show,
        terminal::EnableLineWrap,
        event::DisableMouseCapture,
        terminal::LeaveAlternateScreen,
    )?;
    terminal::disable_raw_mode()?;
    result
}
