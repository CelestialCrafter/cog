use std::{collections::HashMap, iter::repeat_n, fmt::self};

use ratatui::{
    layout,
    prelude::{Buffer, Rect},
    widgets,
};

use super::world::items::Item;

pub mod simple;

pub enum Operation {
    Add(Item, usize),
    Remove(Option<Item>, Option<usize>),
}

pub struct Before(pub Item, pub usize);
pub struct After(pub Item, pub usize);

pub trait Inventory: Send + Sync {
    fn slots(&self) -> &HashMap<Item, usize>;
    fn max_slots(&self) -> usize;

    fn verify(&self, operation: &Operation) -> Option<(Before, After)>;

    /// warning: the inventory is expected not to change between transaction verification and modification
    fn modify(&mut self, operation: &After);
}

impl fmt::Debug for dyn Inventory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.slots())
    }
}

pub struct InventoryWidget<'a>(&'a dyn Inventory);

impl<'a> InventoryWidget<'a> {
    pub fn new(inventory: &'a dyn Inventory) -> Self {
        Self(inventory)
    }
}

impl widgets::Widget for InventoryWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let max_slots = self.0.max_slots();

        let item = Item::default();
        let slots = {
            let original = self.0.slots();
            let len = original.len();
            original
                .into_iter()
                .chain(repeat_n((&item, &0), max_slots - len))
        };

        let block = widgets::Block::bordered().border_type(widgets::BorderType::Rounded);
        let block_area = block.inner(area);

        widgets::Clear.render(area, buf);
        block.render(area, buf);

        let layout_slots = layout::Layout::default()
            .direction(layout::Direction::Horizontal)
            .constraints(vec![
                layout::Constraint::Ratio(1, max_slots as u32);
                max_slots
            ])
            .split(block_area);

        for (i, (item, amount)) in slots.enumerate() {
            widgets::Paragraph::new(format!("{}\n{}", item, amount))
                .alignment(ratatui::layout::Alignment::Center)
                .style(item.color())
                .render(layout_slots[i], buf);
        }
    }
}
