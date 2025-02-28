use std::fmt::Debug;

use crossterm::{event, terminal};
use eyre::Result;
use futures::{
    channel::mpsc::{self, SendError},
    future::BoxFuture,
    stream::{iter, select},
    SinkExt, StreamExt,
};
use log::{error, trace};
use ratatui::DefaultTerminal;

use crate::{AppMessage, Model};

pub enum RuntimeMessage<T> {
    Empty,
    Exit,
    Batch(Vec<RuntimeMessage<T>>),
    Task(BoxFuture<'static, RuntimeMessage<T>>),
    App(AppMessage<T>),
}

impl<T: 'static> RuntimeMessage<T> {
    pub fn map<U>(self, f: impl FnOnce(T) -> U + Send + 'static + Clone) -> RuntimeMessage<U> {
        match self {
            RuntimeMessage::Exit => RuntimeMessage::Exit,
            RuntimeMessage::Empty => RuntimeMessage::Empty,
            RuntimeMessage::Batch(msgs) => {
                RuntimeMessage::Batch(msgs.into_iter().map(|m| m.map(f.clone())).collect())
            }
            RuntimeMessage::Task(task) => {
                RuntimeMessage::Task(Box::pin(async move { task.await.map(f) }))
            }
            RuntimeMessage::App(msg) => RuntimeMessage::App(match msg {
                AppMessage::Init => AppMessage::Init,
                AppMessage::Event(event) => AppMessage::Event(event),
                AppMessage::App(msg) => AppMessage::App(f(msg)),
            }),
        }
    }
}

pub async fn event_loop<T: Debug + Send + 'static>(
    mut model: impl Model<T>,
    mut terminal: DefaultTerminal,
) -> Result<()> {
    let (mut msg_tx, msgs) = mpsc::unbounded();
    let msgs = msgs.map(|msg| Ok(msg));
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

    let mut frame: usize = 0;
    while let Some(msg) = combined.next().await {
        let msg = match msg {
            Err(err) => {
                error!("message error: {}", err);
                continue;
            }
            Ok(msg) => msg,
        };

        if let RuntimeMessage::App(AppMessage::Event(event::Event::Mouse(_))) = msg {
            continue;
        }

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
                trace!("frame: {}, application msg: {:?}", frame, msg);
                frame = frame.wrapping_add(1);

                let out_msg = model.update(msg);
                msg_tx.send(out_msg).await?;
                terminal.draw(|frame| model.view(frame))?;
            }
            RuntimeMessage::Task(task) => {
                let mut msg_tx = msg_tx.clone();
                tokio::task::spawn(async move { msg_tx.send(task.await).await });
            }
        };
    }
    Ok(())
}
