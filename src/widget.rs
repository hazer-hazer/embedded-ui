use core::any::Any;

use alloc::vec::Vec;

use crate::{
    el::{El, ElId},
    event::{Event, EventResponse, Propagate},
    layout::{Layout, LayoutNode, Limits},
    render::Renderer,
    size::{Length, Size},
    state::{State, StateNode, StateTag},
    style::Styler,
    ui::UiCtx,
};

pub trait Widget<Message, R, E: Event, S>
where
    R: Renderer,
{
    fn id(&self) -> Option<ElId>;
    fn tree_ids(&self) -> Vec<ElId>;
    fn size(&self) -> Size<Length>;
    fn layout(
        &self,
        ctx: &mut UiCtx<Message>,
        state: &mut StateNode,
        styler: &S,
        limits: &Limits,
    ) -> LayoutNode;
    fn draw(
        &self,
        ctx: &mut UiCtx<Message>,
        state: &mut StateNode,
        renderer: &mut R,
        styler: &S,
        layout: Layout,
    );

    fn on_event(
        &mut self,
        ctx: &mut UiCtx<Message>,
        event: E,
        state: &mut StateNode,
    ) -> EventResponse<E> {
        let _ = ctx;
        let _ = event;
        let _ = state;
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
