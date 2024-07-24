use alloc::vec::Vec;

use crate::{
    el::ElId,
    event::{Event, EventResponse, Propagate},
    layout::{Layout, LayoutNode, Limits, Position, Viewport},
    render::Renderer,
    size::{Length, Size},
    state::{State, StateNode, StateTag},
    ui::UiCtx,
};

// pub struct Overlay {
//     bounds: Bounds,
//     children: Vec<Overlay>,
// }

pub trait Widget<Message, R, E: Event, S>
where
    R: Renderer,
{
    fn id(&self) -> Option<ElId>;
    fn tree_ids(&self) -> Vec<ElId>;
    fn size(&self, viewport: &Viewport) -> Size<Length>;
    fn position(&self) -> Position {
        Position::Relative
    }
    fn layout(
        &self,
        ctx: &mut UiCtx<Message>,
        state: &mut StateNode,
        styler: &S,
        limits: &Limits,
        viewport: &Viewport,
    ) -> LayoutNode;

    fn draw(
        &self,
        ctx: &mut UiCtx<Message>,
        state: &mut StateNode,
        renderer: &mut R,
        styler: &S,
        layout: Layout,
        viewport: &Viewport,
    );

    fn on_event(
        &mut self,
        ctx: &mut UiCtx<Message>,
        event: E,
        state: &mut StateNode,
        layout: Layout,
    ) -> EventResponse<E> {
        let _ = ctx;
        let _ = event;
        let _ = state;
        let _ = layout;

        Propagate::Ignored.into()
    }

    fn state_tag(&self) -> StateTag {
        StateTag::stateless()
    }
    fn state(&self) -> State {
        State::None
    }
    fn state_children(&self) -> Vec<StateNode> {
        vec![]
    }
}
