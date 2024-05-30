use crate::{
    el::El, event::Event, layout::LayoutNode, overlay::Overlay, render::Renderer, size::Size,
    widget::Widget,
};

pub struct Popup<'a, Message, R, E, S>
where
    R: Renderer,
    E: Event,
{
    content: El<'a, Message, R, E, S>,
}

impl<'a, Message, R, E, S> Popup<'a, Message, R, E, S>
where
    R: Renderer,
    E: Event,
{
    pub fn new(content: El<'a, Message, R, E, S>) -> Self {
        Self { content }
    }
}

impl<'a, Message, R, E, S> Widget<Message, R, E, S> for Popup<'a, Message, R, E, S>
where
    R: Renderer,
    E: Event,
{
    fn id(&self) -> Option<crate::el::ElId> {
        self.content.id()
    }

    fn tree_ids(&self) -> Vec<crate::el::ElId> {
        self.content.tree_ids()
    }

    fn size(&self) -> crate::size::Size<crate::size::Length> {
        Size::zero().as_fixed_length()
    }

    fn layout(
        &self,
        ctx: &mut crate::ui::UiCtx<Message>,
        state: &mut crate::state::StateNode,
        styler: &S,
        limits: &crate::layout::Limits,
    ) -> crate::layout::LayoutNode {
        LayoutNode::new(Size::zero())
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

    fn overlay(&self) -> crate::overlay::Overlay<'_, Message, R, E, S> {
        Overlay::new(Some(self.content), &[])
    }
}
