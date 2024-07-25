use crate::{
    block::{Block, BoxModel},
    color::UiColor,
};

pub struct WidgetDebug<C: UiColor> {
    pub block: Option<Block<C>>,
}

impl<C: UiColor> WidgetDebug<C> {
    pub fn new() -> Self {
        Self { block: None }
    }

    pub fn block(mut self, block: Block<C>) -> Self {
        self.block = Some(block);
        self
    }
}
