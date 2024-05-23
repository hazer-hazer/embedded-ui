use core::{borrow::Borrow, sync::atomic::AtomicU64};

use alloc::boxed::Box;

use crate::{
    event::Event,
    layout::Layout,
    render::Renderer,
    size::{Length, Size},
    state::{self, StateNode},
    style::Styler,
    ui::UiCtx,
    widget::Widget,
};

static NEXT_ID: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ElId {
    Unique(u64),
    Custom(&'static str), // TODO: Custom
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

    fn size(&self) -> Size<Length> {
        self.widget.size()
    }

    fn layout(
        &self,
        ctx: &mut UiCtx<Message>,
        state_tree: &mut StateNode,
        styler: &S,
        limits: &crate::layout::Limits,
    ) -> crate::layout::LayoutNode {
        self.widget.layout(ctx, state_tree, styler, limits)
    }

    fn draw(
        &self,
        ctx: &mut UiCtx<Message>,
        state_tree: &mut StateNode,
        renderer: &mut R,
        styler: &S,
        layout: Layout,
    ) {
        self.widget.draw(ctx, state_tree, renderer, styler, layout)
    }

    fn on_event(
        &mut self,
        ctx: &mut UiCtx<Message>,
        event: E,
        state: &mut StateNode,
    ) -> crate::event::EventResponse<E> {
        self.widget.on_event(ctx, event, state)
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

// impl<'a, Message, R: Renderer, T> From<T> for El<'a, Message, R>
// where
//     T: Widget<Message, R>,
// {

// }
