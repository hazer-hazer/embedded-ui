use alloc::vec::Vec;

use crate::{
    el::{El, ElId},
    event::Event,
    layout::Layout,
    render::Renderer,
    size::{Length, Size},
    widget::Widget,
};

pub struct Select<'a, Message, R, E, S>
where
    R: Renderer,
    E: Event,
{
    id: ElId,
    size: Size<Length>,
    options: Vec<El<'a, Message, R, E, S>>,
    chosen: usize,
}

impl<'a, Message, R, E, S> Select<'a, Message, R, E, S>
where
    R: Renderer,
    E: Event,
{
    pub fn new(options: Vec<El<'a, Message, R, E, S>>) -> Self {
        Self { id: ElId::unique(), size: Size::fill(), options, chosen: 0 }
    }

    pub fn initial(mut self, index: impl Into<usize>) -> Self {
        self.chosen = index.into();
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.size.width = width.into();
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.size.height = height.into();
        self
    }
}

impl<'a, Message, R, E, S> Widget<Message, R, E, S> for Select<'a, Message, R, E, S>
where
    R: Renderer,
    E: Event,
{
    fn id(&self) -> Option<crate::el::ElId> {
        Some(self.id)
    }

    fn tree_ids(&self) -> Vec<crate::el::ElId> {
        self.options.iter().map(|option| option.tree_ids()).flatten().collect()
    }

    fn size(&self) -> Size<Length> {
        self.size
    }

    fn layout(
        &self,
        ctx: &mut crate::ui::UiCtx<Message>,
        state: &mut crate::state::StateNode,
        styler: &S,
        limits: &crate::layout::Limits,
    ) -> crate::layout::LayoutNode {
        Layout::container(limits, self.size.width, self.size.height, |limits| {
            self.options[self.chosen].layout(ctx, state, styler, limits)
        })
    }

    fn draw(
        &self,
        ctx: &mut crate::ui::UiCtx<Message>,
        state: &mut crate::state::StateNode,
        renderer: &mut R,
        styler: &S,
        layout: crate::layout::Layout,
    ) {
        
    }
}
