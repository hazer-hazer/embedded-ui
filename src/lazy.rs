use crate::{el::El, event::Event, render::Renderer, widget::Widget};

pub struct Lazy<'a, F, Message, R: Renderer, E: Event, S>
where
    F: FnMut() -> El<'a, Message, R, E, S>,
{
    gen: F,
    el: El<'a, Message, R, E, S>,
}

impl<'a, F, Message, R: Renderer, E: Event, S> Widget<Message, R, E, S>
    for Lazy<'a, F, Message, R, E, S>
where
    F: FnMut() -> El<'a, Message, R, E, S>,
{
    fn id(&self) -> Option<crate::el::ElId> {
        self.el.id()
    }

    fn tree_ids(&self) -> Vec<crate::el::ElId> {
        self.el.tree_ids()
    }

    fn state_tag(&self) -> crate::state::StateTag {
        self.el.state_tag()
    }

    fn state(&self) -> crate::state::State {
        self.el.state()
    }

    fn state_children(&self) -> Vec<crate::state::StateNode> {
        self.el.state_children()
    }

    fn on_event(
        &mut self,
        ctx: &mut crate::ui::UiCtx<Message>,
        event: E,
        state: &mut crate::state::StateNode,
    ) -> crate::event::EventResponse<E> {
        self.el.on_event(ctx, event, state)
    }

    fn size(&self) -> crate::size::Size<crate::size::Length> {
        self.el.size()
    }

    fn layout(
        &self,
        ctx: &mut crate::ui::UiCtx<Message>,
        state: &mut crate::state::StateNode,
        styler: &S,
        limits: &crate::layout::Limits,
    ) -> crate::layout::LayoutNode {
        self.el.layout(ctx, state, styler, limits)
    }

    fn draw(
        &self,
        ctx: &mut crate::ui::UiCtx<Message>,
        state: &mut crate::state::StateNode,
        renderer: &mut R,
        styler: &S,
        layout: crate::layout::Layout,
    ) {
        self.el.draw(ctx, state, renderer, styler, layout)
    }
}
