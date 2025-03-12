use std::fmt;

use ratatui::{
    layout,
    prelude::{Buffer, Rect},
    widgets,
};

use super::world::items::Item;

pub mod simple;

pub type Slot = u64;
pub type Amount = u64;

pub struct Before(pub Amount);
pub struct After(pub Amount);

pub enum VerifyOperation {
    Add(Item, Amount),
    Remove(Option<Item>, Option<Amount>),
}

#[derive(Clone)]
pub struct ModifyOperation {
    pub slot: Slot,
    pub item: Item,
    pub amount: Amount,
}

pub trait Inventory: Send + Sync {
    fn slots(&self) -> Vec<&(Item, u64)>;

    fn verify(&self, operation: VerifyOperation) -> Option<(ModifyOperation, Before, After)>;
    /// warning: the inventory is expected not to change between transaction verification and modification
    fn modify(&mut self, operation: ModifyOperation);

    fn swap(&mut self, other: &mut Box<dyn Inventory>, operation: VerifyOperation) -> Option<()> {
        let (self_op, before, _) = self.verify(operation)?;
        let (other_op, ..) = other.verify(VerifyOperation::Add(self_op.item, before.0))?;

        self.modify(self_op);
        other.modify(other_op);

        Some(())
    }
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
        let slots = self.0.slots();

        let block = widgets::Block::bordered().border_type(widgets::BorderType::Rounded);
        let block_area = block.inner(area);

        widgets::Clear.render(area, buf);
        block.render(area, buf);

        let layout_slots = layout::Layout::default()
            .direction(layout::Direction::Horizontal)
            .constraints(vec![
                layout::Constraint::Ratio(1, slots.len() as u32);
                slots.len()
            ])
            .split(block_area);

        for (i, (item, amount)) in slots.into_iter().enumerate() {
            widgets::Paragraph::new(format!("{}\n{}", item, amount))
                .alignment(ratatui::layout::Alignment::Center)
                .style(item.color())
                .render(layout_slots[i], buf);
        }
    }
}
