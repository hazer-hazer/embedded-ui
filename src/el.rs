use core::{borrow::Borrow, sync::atomic::AtomicUsize};

use alloc::boxed::Box;

use crate::{
    event::Event,
    layout::{Layout, Viewport},
    render::Renderer,
    size::{Length, Size},
    state::{self, StateNode},
    ui::UiCtx,
    widget::Widget,
};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub enum ElId {
    Unique(usize),
    Custom(&'static str),
}

impl ElId {
    pub fn new(name: &'static str) -> Self {
        Self::Custom(name)
    }

    pub fn unique() -> Self {
        Self::Unique(NEXT_ID.fetch_add(1, core::sync::atomic::Ordering::Relaxed))
    }
}

impl From<&'static str> for ElId {
    fn from(value: &'static str) -> Self {
        Self::new(value)
    }
}

#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub struct El<'a, Message, R: Renderer, E: Event, S> {
    widget: Box<dyn Widget<Message, R, E, S> + 'a>,
}

impl<'a, Message, R: Renderer, E: Event, S> Widget<Message, R, E, S> for El<'a, Message, R, E, S> {
    fn id(&self) -> Option<ElId> {
        self.widget.id()
    }

    fn tree_ids(&self) -> alloc::vec::Vec<ElId> {
        self.widget.tree_ids()
    }

    fn size(&self, viewport: &Viewport) -> Size<Length> {
        self.widget.size(viewport)
    }

    fn layout(
        &self,
        ctx: &mut UiCtx<Message>,
        state_tree: &mut StateNode,
        styler: &S,
        limits: &crate::layout::Limits,
        viewport: &Viewport,
    ) -> crate::layout::LayoutNode {
        self.widget.layout(ctx, state_tree, styler, limits, viewport)
    }

    fn draw(
        &self,
        ctx: &mut UiCtx<Message>,
        state_tree: &mut StateNode,
        renderer: &mut R,
        styler: &S,
        layout: Layout,
        viewport: &Viewport,
    ) {
        self.widget.draw(ctx, state_tree, renderer, styler, layout, viewport)
    }

    fn on_event(
        &mut self,
        ctx: &mut UiCtx<Message>,
        event: E,
        state: &mut StateNode,
        layout: Layout,
    ) -> crate::event::EventResponse<E> {
        self.widget.on_event(ctx, event, state, layout)
    }

    fn state_tag(&self) -> crate::state::StateTag {
        self.widget.state_tag()
    }

    fn state(&self) -> state::State {
        self.widget.state()
    }

    fn state_children(&self) -> alloc::vec::Vec<StateNode> {
        self.widget.state_children()
    }
}

impl<'a, Message, R: Renderer, E: Event, S> El<'a, Message, R, E, S> {
    pub fn new(widget: impl Widget<Message, R, E, S> + 'a) -> Self {
        Self { widget: Box::new(widget) }
    }

    pub fn widget(&self) -> &dyn Widget<Message, R, E, S> {
        self.widget.as_ref()
    }
}

impl<'a, Message, R: Renderer, E: Event, S> Borrow<dyn Widget<Message, R, E, S> + 'a>
    for El<'a, Message, R, E, S>
{
    fn borrow(&self) -> &(dyn Widget<Message, R, E, S> + 'a) {
        self.widget.borrow()
    }
}

impl<'a, Message, R: Renderer, E: Event, S> Borrow<dyn Widget<Message, R, E, S> + 'a>
    for &El<'a, Message, R, E, S>
{
    fn borrow(&self) -> &(dyn Widget<Message, R, E, S> + 'a) {
        self.widget.borrow()
    }
}
