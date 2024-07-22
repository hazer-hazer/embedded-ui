use alloc::vec::Vec;

use crate::{
    el::ElId,
    event::{Event, EventResponse, Propagate},
    layout::{Layout, LayoutNode, Limits, Position, Viewport},
    render::Renderer,
    size::{Length, Size},
    state::{State, StateNode, StateTag},
    style::Styler,
    ui::UiCtx,
};

// pub struct Overlay {
//     bounds: Bounds,
//     children: Vec<Overlay>,
// }

pub struct LayoutCtx<'a, Message, S> {
    pub ctx: &'a mut UiCtx<Message>,
    pub state: &'a mut StateNode,
    pub styler: &'a S,
    pub limits: &'a Limits,
    pub viewport: &'a Viewport,
}

// impl<'a, Message, S> LayoutCtx<'a, Message, S> {
//     pub fn children(&'a mut self) -> impl Iterator<Item = LayoutCtx<'a,
// Message, S>> + 'a {         self.state.children.iter_mut().map(|state_child|
// LayoutCtx {             ctx: self.ctx,
//             state: state_child,
//             styler: self.styler,
//             limits: self.limits,
//             viewport: self.viewport,
//         })
//     }
// }

pub struct DrawCtx<'a, Message, R, S>
where
    R: Renderer,
{
    pub ctx: &'a mut UiCtx<Message>,
    pub state: &'a mut StateNode,
    pub renderer: &'a mut R,
    pub styler: &'a S,
    pub layout: Layout<'a>,
    pub viewport: &'a Viewport,
}

impl<'a, Message, R, S> DrawCtx<'a, Message, R, S>
where
    R: Renderer,
{
    // pub fn for_child(&'a mut self, state: &'a mut StateNode, layout: Layout<'a>) -> Self {
    //     Self {
    //         ctx: self.ctx,
    //         state,
    //         renderer: self.renderer,
    //         styler: self.styler,
    //         layout,
    //         viewport: self.viewport,
    //     }
    // }

    // pub fn draw_children<E: Event>(self, children: &mut [impl Widget<Message, R, E, S>]) {
    //     for ((child, child_state), child_layout) in
    //         children.iter().zip(self.state.children.iter_mut()).zip(self.layout.children())
    //     {
    //         child.draw(&mut self.for_child(child_state, child_layout));
    //     }
    // }
}

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
    fn layout(&self, ctx: &mut LayoutCtx<'_, Message, S>) -> LayoutNode;

    fn draw(&self, ctx: &mut DrawCtx<'_, Message, R, S>);

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
