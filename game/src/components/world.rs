use std::iter::repeat_n;

use cog_core::{runtime::RuntimeMessage, util::controls::ControlCluster, AppMessage, Model};
use crossterm::event::Event;
use items::{Item, ZoomLevel};
use ndarray::{s, Array2};
use ratatui::{
    prelude::{Buffer, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Paragraph, Widget},
    Frame,
};

use crate::{
    controls::{BasicCluster, WorldCluster},
    store::RRStore,
};

pub mod items;

pub type Position = (usize, usize);

pub struct World {
    pub grid: Array2<Item>,
    pub cursor: Position,
}

impl World {
    pub fn new(size: usize) -> Self {
        Self {
            grid: Array2::default((size, size)),
            cursor: (0, 0),
        }
    }

    pub fn size(&self) -> usize {
        self.grid.shape()[0]
    }

    pub fn travel(&mut self, r: isize, c: isize) {
        let bounds = |a: isize, b: usize| {
            match a {
                0.. => b.saturating_add(a as usize),
                ..0 => b.saturating_sub(a.abs() as usize),
            }
            .min(self.size() - 1)
        };

        self.cursor = (bounds(r, self.cursor.0), bounds(c, self.cursor.1));
    }

    pub fn place(&mut self, item: Item, position: Position) {
        *self.grid.get_mut(position).expect("cell out of bounds") = item;
    }

    pub fn destroy(&mut self, position: Position) {
        *self.grid.get_mut(position).expect("cell out of bounds") = Item::Empty;
    }
}

pub struct WorldWidget<'a> {
    world: &'a World,
    zoom: ZoomLevel,
}

impl<'a> WorldWidget<'a> {
    fn new(world: &'a World, zoom: ZoomLevel) -> Self {
        Self { world, zoom }
    }
}

impl<'a> Widget for WorldWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let (cur_row, cur_col) = self.world.cursor;

        let zoom_n = self.zoom as usize;
        let size = self.world.size();
        let width = area.width as usize / 2 / zoom_n;
        let height = area.height as usize / zoom_n;

        let ((cursor_row, cursor_col), viewport) = {
            let bounds = |a: usize, b: usize| {
                let s = a.saturating_sub(b / 2).min(size.saturating_sub(b));
                let e = (a + b).min(size);

                (s, e)
            };

            let (rs, re) = bounds(cur_row, height);
            let (cs, ce) = bounds(cur_col, width);

            (
                (cur_row - rs, cur_col - cs),
                self.world.grid.slice(s![rs..re, cs..ce]),
            )
        };

        let lines: Vec<_> = viewport
            .rows()
            .into_iter()
            .enumerate()
            .map(|(r, row)| {
                let mut lines: Vec<_> = repeat_n(Line::default(), zoom_n).collect();

                for (c, cell) in row.into_iter().enumerate() {
                    let mut text = cell.render(self.zoom);
                    if r == cursor_row && c == cursor_col {
                        text = text.patch_style(Style::new().bg(Color::LightYellow));
                    }

                    for (i, line) in text.lines.into_iter().enumerate() {
                        for span in line.spans.into_iter().map(|span| span.style(text.style)) {
                            lines[i].push_span(span);
                        }
                    }
                }

                lines
            })
            .flatten()
            .collect();

        Paragraph::new(lines).centered().render(area, buf);
    }
}

#[derive(Debug)]
pub enum WorldMessage {
    Place(Item, Position),
    Destroy(Position),
}

pub struct WorldModel {
    store: RRStore,
    zoom: ZoomLevel,
}

impl WorldModel {
    pub fn new(store: RRStore) -> Self {
        Self {
            store,
            zoom: ZoomLevel::Close,
        }
    }
}

impl Model<WorldMessage> for WorldModel {
    fn view(&mut self, frame: &mut Frame) {
        WorldWidget::new(&self.store.borrow().world, self.zoom)
            .render(frame.area(), frame.buffer_mut());
    }

    fn update(&mut self, message: AppMessage<WorldMessage>) -> RuntimeMessage<WorldMessage> {
        let mut store = self.store.borrow_mut();
        match message {
            AppMessage::Event(Event::Key(event)) => {
                match BasicCluster::contains(&event) {
                    Some(BasicCluster::Left) => store.world.travel(0, -1),
                    Some(BasicCluster::Right) => store.world.travel(0, 1),
                    Some(BasicCluster::Up) => store.world.travel(-1, 0),
                    Some(BasicCluster::Down) => store.world.travel(1, 0),
                    _ => (),
                }

                match WorldCluster::contains(&event) {
                    Some(WorldCluster::ZoomIn) => {
                        if let ZoomLevel::Far = self.zoom {
                            self.zoom = ZoomLevel::Close;
                        }
                    }
                    Some(WorldCluster::ZoomOut) => {
                        if let ZoomLevel::Close = self.zoom {
                            self.zoom = ZoomLevel::Far;
                        }
                    }
                    _ => (),
                }
            }
            AppMessage::App(msg) => match msg {
                WorldMessage::Place(item, pos) => store.world.place(item, pos),
                WorldMessage::Destroy(pos) => store.world.destroy(pos),
            },
            _ => (),
        };

        RuntimeMessage::Empty
    }
}
