pub mod text;

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
