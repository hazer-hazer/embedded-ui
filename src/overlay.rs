use crate::{
    el::El,
    event::{Event, Propagate},
    layout::LayoutNode,
    render::Renderer,
    size::Size,
    state::{StateNode, StateTag},
    widget::Widget,
};

pub struct Overlay<'a, Message, R, E, S>
where
    R: Renderer,
    E: Event,
{
    overlay: Option<&'a El<'a, Message, R, E, S>>,
    children: Vec<Overlay<'a, Message, R, E, S>>,
}

impl<'a, Message, R, E, S> Overlay<'a, Message, R, E, S>
where
    R: Renderer,
    E: Event,
{
    pub fn none() -> Self {
        Self { overlay: None, children: vec![] }
    }

    pub fn new(
        overlay: Option<&'a El<'a, Message, R, E, S>>,
        children: &'a [El<'a, Message, R, E, S>],
    ) -> Self {
        Self { overlay, children: children.iter().map(|el| el.overlay()).collect() }
    }

    pub fn from_children(children: &'a [El<'a, Message, R, E, S>]) -> Self {
        Self::new(None, children)
    }

    pub fn with_children(
        overlay: &'a El<'a, Message, R, E, S>,
        children: &'a [El<'a, Message, R, E, S>],
    ) -> Self {
        Self::new(Some(overlay), children)
    }
}

impl<'a, Message, R, E, S> Widget<Message, R, E, S> for Overlay<'a, Message, R, E, S>
where
    R: Renderer,
    E: Event,
{
    fn id(&self) -> Option<crate::el::ElId> {
        self.overlay.as_ref().and_then(|el| el.id())
    }

    fn tree_ids(&self) -> Vec<crate::el::ElId> {
        let mut ids = vec![];
        if let Some(id) = self.id() {
            ids.push(id);
        }

        ids.extend(self.children.iter().map(|child| child.tree_ids()).flatten());

        ids
    }

    fn size(&self) -> crate::size::Size<crate::size::Length> {
        // TODO: ????
        if let Some(overlay) = self.overlay.as_ref() {
            overlay.size()
        } else {
            Size::shrink()
        }
    }

    fn on_event(
        &mut self,
        ctx: &mut crate::ui::UiCtx<Message>,
        event: E,
        state: &mut crate::state::StateNode,
    ) -> crate::event::EventResponse<E> {
        self.overlay.as_mut().map_or(Propagate::Ignored.into(), |overlay| {
            overlay.on_event(ctx, event.clone(), state)
        })?;

        for (child, state) in self.children.iter_mut().zip(state.children.iter_mut()) {
            child.on_event(ctx, event.clone(), state)?;
        }

        Propagate::Ignored.into()
    }

    fn state_tag(&self) -> crate::state::StateTag {
        self.overlay.as_ref().map_or(StateTag::stateless(), |overlay| overlay.state_tag())
    }

    fn state(&self) -> crate::state::State {
        self.overlay.as_ref().map_or(crate::state::State::None, |overlay| overlay.state())
    }

    fn state_children(&self) -> Vec<crate::state::StateNode> {
        self.children
            .iter()
            .map(|child| StateNode::new(child as &dyn Widget<Message, R, E, S>))
            .collect()
    }

    fn layout(
        &self,
        ctx: &mut crate::ui::UiCtx<Message>,
        state: &mut crate::state::StateNode,
        styler: &S,
        limits: &crate::layout::Limits,
    ) -> crate::layout::LayoutNode {
        LayoutNode::with_children(
            limits.max(),
            self.children
                .iter()
                .map(|child| child.layout(ctx, state, styler, limits))
                .collect::<Vec<_>>(),
        )
    }

    fn draw(
        &self,
        ctx: &mut crate::ui::UiCtx<Message>,
        state: &mut crate::state::StateNode,
        renderer: &mut R,
        styler: &S,
        layout: crate::layout::Layout,
    ) {
        if let Some(overlay) = self.overlay.as_ref() {
            overlay.draw(ctx, state, renderer, styler, layout.clone());
        }

        for (child, layout) in self.children.iter().zip(layout.children()) {
            child.draw(ctx, state, renderer, styler, layout);
        }
    }

    fn overlay(&self) -> Overlay<'_, Message, R, E, S> {
        // TODO: Nested overlays
        Overlay::none()
    }
}
