use std::{
    io::{stdout, Write},
    panic,
};

use crossterm::{cursor, event, execute, queue, terminal};
use eyre::Result;
use futures::{
    channel::mpsc::{self, SendError},
    future::BoxFuture,
    stream::{iter, select},
    SinkExt, StreamExt,
};

use crate::{AppMessage, Model};

pub enum RuntimeMessage<T> {
    Empty,
    Exit,
    Batch(Vec<RuntimeMessage<T>>),
    Task(BoxFuture<'static, RuntimeMessage<T>>),
    App(AppMessage<T>),
}

impl<T: 'static> RuntimeMessage<T> {
    // Add a method to map the inner type
    pub fn map<U>(self, f: impl FnOnce(T) -> U + Send + 'static + Clone) -> RuntimeMessage<U> {
        match self {
            RuntimeMessage::Exit => RuntimeMessage::Exit,
            RuntimeMessage::Empty => RuntimeMessage::Empty,
            RuntimeMessage::Batch(msgs) => {
                RuntimeMessage::Batch(msgs.into_iter().map(|m| m.map(f.clone())).collect())
            }
            RuntimeMessage::App(msg) => RuntimeMessage::App(match msg {
                AppMessage::Init => AppMessage::Init,
                AppMessage::Event(event) => AppMessage::Event(event),
                AppMessage::App(msg) => AppMessage::App(f(msg)),
            }),
            RuntimeMessage::Task(task) => {
                RuntimeMessage::Task(Box::pin(async move { task.await.map(f) }))
            }
        }
    }
}

async fn event_loop<T: Send + 'static>(mut model: impl Model<T>) -> Result<()> {
    let (mut msg_tx, msgs) = mpsc::unbounded();
    let msgs = msgs.map(|m| Ok(m));
    let events =
        event::EventStream::new().map(|e| e.map(|e| RuntimeMessage::App(AppMessage::Event(e))));
    let mut combined = select(msgs, events);

    msg_tx.send(RuntimeMessage::App(AppMessage::Init)).await?;

    let (c, r) = terminal::size()?;
    msg_tx
        .send(RuntimeMessage::App(AppMessage::Event(
            event::Event::Resize(c, r),
        )))
        .await?;

    let mut writer = stdout();
    while let Some(message) = combined.next().await {
        let msg = message?;
        match msg {
            RuntimeMessage::Exit => break,
            RuntimeMessage::Empty => (),
            RuntimeMessage::Batch(msgs) => {
                let mut msg_tx = &msg_tx;
                iter(msgs.into_iter())
                    .fold(Ok(()), |acc: Result<_, SendError>, x| async move {
                        if let Err(_) = acc {
                            acc
                        } else {
                            msg_tx.send(x).await.map(|_| ())
                        }
                    })
                    .await?;
            }
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

pub fn cleanup() -> Result<()> {
    execute!(
        stdout(),
        cursor::Show,
        terminal::EnableLineWrap,
        event::DisableMouseCapture,
        terminal::LeaveAlternateScreen,
    )?;
    terminal::disable_raw_mode()?;

    Ok(())
}

pub async fn init<T: Send + 'static>(model: impl Model<T>) -> Result<()> {
    terminal::enable_raw_mode()?;

    execute!(
        stdout(),
        cursor::Hide,
        terminal::EnterAlternateScreen,
        terminal::DisableLineWrap,
        event::EnableMouseCapture
    )?;

    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        let _ = cleanup();
        original_hook(info);
    }));

    let event_loop = event_loop(model).await;
    cleanup()?;
    event_loop
}
