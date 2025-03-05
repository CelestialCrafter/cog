use std::iter::repeat_n;

use cog_core::{runtime::RuntimeMessage, util::controls::ControlCluster, AppMessage, Model};
use crossterm::event::Event;
use items::{Item, ZoomLevel};
use ndarray::{s, Array2, Dim, NdIndex};
use rand::{
    distr::{Distribution, StandardUniform},
    Rng,
};
use ratatui::{
    prelude::{Buffer, Rect},
    style::Style,
    text::Line,
    widgets::{Paragraph, Widget},
    Frame,
};

use crate::{
    colors,
    components::store::RRStore,
    controls::{BasicCluster, WorldCluster},
};

use super::{
    entity::get_player,
    inventory::{Inventory, Operation},
    store::Store,
};

pub mod items;

pub const SIZE: usize = 5;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Position(pub usize, pub usize);

impl Distribution<Position> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Position {
        Position(rng.random_range(..SIZE), rng.random_range(..SIZE))
    }
}

type Dim2 = Dim<[usize; 2]>;
unsafe impl NdIndex<Dim2> for Position {
    #[inline]
    fn index_checked(&self, dim: &Dim2, strides: &Dim2) -> Option<isize> {
        [self.0, self.1].index_checked(dim, strides)
    }

    #[inline]
    fn index_unchecked(&self, strides: &Dim2) -> isize {
        [self.0, self.1].index_unchecked(strides)
    }
}

impl From<(usize, usize)> for Position {
    fn from(val: (usize, usize)) -> Self {
        Self(val.0, val.1)
    }
}

impl Position {
    pub fn move_by(&self, direction: Direction, multiplier: usize) -> Option<Self> {
        let bounds = |a: isize, b: usize| {
            let v = match a {
                0.. => b.checked_add(a as usize * multiplier)?,
                ..0 => b.checked_sub(a.abs() as usize * multiplier)?,
            };

            if v >= SIZE {
                None
            } else {
                Some(v)
            }
        };

        let direction: (isize, isize) = direction.into();
        Some(Position(
            bounds(direction.0, self.0)?,
            bounds(direction.1, self.1)?,
        ))
    }
}

#[derive(Clone, Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Distribution<Direction> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.random_range(..4_u8) {
            0 => Direction::North,
            1 => Direction::South,
            2 => Direction::East,
            3 => Direction::West,
            _ => unreachable!(),
        }
    }
}

impl Into<(isize, isize)> for Direction {
    fn into(self) -> (isize, isize) {
        match self {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::East => (0, -1),
            Direction::West => (0, 1),
        }
    }
}

impl Direction {
    pub fn flip(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

pub struct World {
    pub grid: Array2<Item>,
    pub cursor: Position,
}

impl World {
    pub fn new() -> Self {
        Self {
            grid: Array2::default((SIZE, SIZE)),
            cursor: Position(0, 0),
        }
    }

    pub fn size(&self) -> usize {
        self.grid.shape()[0]
    }

    pub fn place(&mut self, item: Item, position: Position) {
        *self.grid.get_mut(position).expect("cell out of bounds") = item;
    }

    pub fn destroy(&mut self, position: Position) {
        *self.grid.get_mut(position).expect("cell out of bounds") = Item::Empty;
    }
}

pub struct WorldWidget<'a>(&'a World, ZoomLevel);

impl<'a> WorldWidget<'a> {
    fn new(world: &'a World, zoom: ZoomLevel) -> Self {
        Self(world, zoom)
    }
}

impl<'a> Widget for WorldWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let Position(cur_row, cur_col) = self.0.cursor;

        let zoom_n = self.1 as usize;
        let width = area.width as usize / 2 / zoom_n;
        let height = area.height as usize / zoom_n;

        let ((cursor_row, cursor_col), viewport) = {
            let bounds = |a: usize, b: usize| {
                let s = a.saturating_sub(b / 2).min(SIZE.saturating_sub(b));
                let e = (a + b).min(SIZE);

                (s, e)
            };

            let (rs, re) = bounds(cur_row, height);
            let (cs, ce) = bounds(cur_col, width);

            (
                (cur_row - rs, cur_col - cs),
                self.0.grid.slice(s![rs..re, cs..ce]),
            )
        };

        let lines: Vec<_> = viewport
            .rows()
            .into_iter()
            .enumerate()
            .map(|(r, row)| {
                let mut lines: Vec<_> = repeat_n(Line::default(), zoom_n).collect();

                for (c, cell) in row.into_iter().enumerate() {
                    let mut text = cell.render(self.1);
                    if r == cursor_row && c == cursor_col {
                        text = text.patch_style(Style::new().bg(colors::ACCENT));
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
pub enum WorldMessage {}

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

    fn handle_select(store: &mut Store) {
        let cursor = store.world.cursor;
        let cursor_item = store.world.grid[cursor];

        let (_, inventory) = get_player::<&mut Box<dyn Inventory>>(&mut store.entities);

        match cursor_item {
            Item::Empty => {
                if let Some((_, op)) = inventory.verify(&Operation::Remove(None, Some(1))) {
                    inventory.modify(&op);
                    if let Some(entity) = op.0.entity() {
                        let _ = store.entities.insert_one(*entity, cursor);
                    }
                    store.world.place(op.0, cursor)
                }
            }
            _ => {
                if let Some((_, op)) = inventory.verify(&Operation::Add(cursor_item, 1)) {
                    inventory.modify(&op);
                    if let Some(entity) = op.0.entity() {
                        let _ = store.entities.remove_one::<Position>(*entity);
                    }
                    store.world.destroy(cursor)
                }
            }
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
                let w = &mut store.world;
                let mut new_position = None;
                match BasicCluster::contains(&event) {
                    Some(BasicCluster::Left) => {
                        let _ = new_position.insert(w.cursor.move_by(Direction::East, 1));
                    }
                    Some(BasicCluster::Right) => {
                        let _ = new_position.insert(w.cursor.move_by(Direction::West, 1));
                    }
                    Some(BasicCluster::Up) => {
                        let _ = new_position.insert(w.cursor.move_by(Direction::North, 1));
                    }
                    Some(BasicCluster::Down) => {
                        let _ = new_position.insert(w.cursor.move_by(Direction::South, 1));
                    }
                    Some(BasicCluster::Select) => Self::handle_select(&mut store),
                    None => (),
                }

                if let Some(np) = new_position.flatten() {
                    store.world.cursor = np;

                    let (_, position) = get_player::<&mut Position>(&mut store.entities);
                    *position = np;
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
            _ => (),
        };

        RuntimeMessage::Empty
    }
}
