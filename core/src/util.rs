use ratatui::layout::{Constraint, Flex, Layout, Rect};

use crate::{runtime::RuntimeMessage, AppMessage};

pub mod controls;

#[macro_export]
macro_rules! passthru {
    ($msg:expr, $(($path:path, $model:expr)), *) => {
        match $msg {
            AppMessage::Init => RuntimeMessage::Batch(vec![$($model.update(AppMessage::Init).map($path)),*]),
            AppMessage::Event(event) => RuntimeMessage::Batch(vec![$($model.update(AppMessage::Event(event.clone())).map($path)),*]),
            $(AppMessage::App($path(message)) => $model.update(AppMessage::App(message)).map($path)),*,
            _ => RuntimeMessage::Empty
        }
    };
}

pub fn app_message<T>(msg: T) -> RuntimeMessage<T> {
    RuntimeMessage::App(AppMessage::App(msg))
}

#[derive(Default)]
pub struct Anchor {
    fv: Flex,
    fh: Flex,
    pv: u16,
    ph: u16,
}

impl Anchor {
    pub fn flex(mut self, v: Flex, h: Flex) -> Self {
        self.fv = v;
        self.fh = h;
        self
    }

    pub fn flex_uniform(mut self, f: Flex) -> Self {
        self.fv = f;
        self.fh = f;
        self
    }

    pub fn percentage(mut self, v: u16, h: u16) -> Self {
        self.pv = v;
        self.ph = h;
        self
    }

    pub fn percentage_uniform(mut self, p: u16) -> Self {
        self.pv = p;
        self.ph = p;
        self
    }

    pub fn compute(&self, area: Rect) -> Rect {
        let vertical = Layout::vertical([Constraint::Percentage(self.pv)]).flex(self.fv);
        let horizontal = Layout::horizontal([Constraint::Percentage(self.ph)]).flex(self.fh);

        let [area] = vertical.areas(area);
        let [area] = horizontal.areas(area);

        area
    }
}
