use core::marker::PhantomData;

use alloc::vec::Vec;

use crate::{
    align::Alignment,
    axis::Axis,
    el::{El, ElId},
    event::{Event, EventResponse, Propagate},
    layout::{Layout, Viewport},
    padding::Padding,
    render::Renderer,
    size::{Length, Size},
    state::StateNode,
    ui::UiCtx,
    widget::Widget,
};

pub trait LinearDirection {
    const AXIS: Axis;
}

pub struct DirectionColumn;
impl LinearDirection for DirectionColumn {
    const AXIS: Axis = Axis::Y;
}

pub struct DirectionRow;
impl LinearDirection for DirectionRow {
    const AXIS: Axis = Axis::X;
}

pub type Column<'a, Message, R, E, S> = Linear<'a, Message, R, E, S, DirectionColumn>;
pub type Row<'a, Message, R, E, S> = Linear<'a, Message, R, E, S, DirectionRow>;

pub struct Linear<'a, Message, R: Renderer, E: Event, S, D: LinearDirection> {
    spacing: u32,
    size: Size<Length>,
    padding: Padding,
    gap: u32,
    align: Alignment,
    children: Vec<El<'a, Message, R, E, S>>,

    dir: PhantomData<D>,
}

impl<'a, Message, R: Renderer, E: Event, S, D: LinearDirection> Linear<'a, Message, R, E, S, D> {
    pub fn new(children: impl IntoIterator<Item = El<'a, Message, R, E, S>>) -> Self {
        Self {
            spacing: 0,
            size: Size::fill(),
            padding: Padding::default(),
            gap: 0,
            align: Alignment::Start,
            children: children.into_iter().collect(),
            dir: PhantomData,
        }
    }

    pub fn spacing(mut self, spacing: u32) -> Self {
        self.spacing = spacing;
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

    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    pub fn gap(mut self, gap: u32) -> Self {
        self.gap = gap;
        self
    }

    pub fn align(mut self, align: Alignment) -> Self {
        self.align = align;
        self
    }

    pub fn add(mut self, child: impl Into<El<'a, Message, R, E, S>>) -> Self {
        self.children.push(child.into());
        self
    }

    // fn focus_child(&self, child_index: usize, focus_offset: i32) -> FocusResult {
    //     let new_focus_index = child_index as i32 + focus_offset;

    //     if new_focus_index < 0 {
    //         return FocusResult::Outside(new_focus_index);
    //     }

    //     let new_focused_child =
    //         self.children.iter().filter_map(|child|
    // child.id()).nth(new_focus_index as usize);

    //     if let Some(new_focused_child) = new_focused_child {
    //         FocusResult::Child(new_focused_child)
    //     } else {
    //         FocusResult::Outside(new_focus_index)
    //     }
    // }
}

impl<'a, Message, R: Renderer, E: Event, S, D: LinearDirection> Widget<Message, R, E, S>
    for Linear<'a, Message, R, E, S, D>
{
    fn id(&self) -> Option<crate::el::ElId> {
        None
    }

    fn tree_ids(&self) -> Vec<ElId> {
        self.children.iter().map(|child| child.tree_ids()).flatten().collect()
    }

    fn size(&self) -> crate::size::Size<Length> {
        self.size
    }

    fn state_children(&self) -> Vec<StateNode> {
        self.children.iter().map(|child| StateNode::new(child)).collect()
    }

    fn on_event(
        &mut self,
        ctx: &mut UiCtx<Message>,
        event: E,
        state: &mut crate::state::StateNode,
    ) -> EventResponse<E> {
        for (child, child_state) in self.children.iter_mut().zip(state.children.iter_mut()) {
            match child.on_event(ctx, event.clone(), child_state)? {
                Propagate::Ignored => {},
                bubbled @ Propagate::BubbleUp(..) => return bubbled.into(),
            }
        }

        Propagate::Ignored.into()
    }

    fn layout(
        &self,
        ctx: &mut UiCtx<Message>,
        state: &mut StateNode,
        styler: &S,
        limits: &crate::layout::Limits,
        viewport: &Viewport,
    ) -> crate::layout::LayoutNode {
        Layout::flex(
            ctx,
            state,
            styler,
            D::AXIS,
            limits,
            self.size,
            crate::layout::Position::Relative,
            viewport,
            self.padding,
            self.gap,
            self.align,
            &self.children,
        )
    }

    fn draw(
        &self,
        ctx: &mut UiCtx<Message>,
        state: &mut StateNode,
        renderer: &mut R,
        styler: &S,
        layout: crate::layout::Layout,
        viewport: &Viewport,
    ) {
        // TODO: Draw only children inside viewport?
        for ((child, child_state), child_layout) in
            self.children.iter().zip(state.children.iter_mut()).zip(layout.children())
        {
            child.draw(ctx, child_state, renderer, styler, child_layout, viewport);
        }
    }
}

impl<'a, Message, R, E, S, D> From<Linear<'a, Message, R, E, S, D>> for El<'a, Message, R, E, S>
where
    Message: 'a,
    R: Renderer + 'a,
    E: Event + 'a,
    S: 'a,
    D: LinearDirection + 'a,
{
    fn from(value: Linear<'a, Message, R, E, S, D>) -> Self {
        Self::new(value)
    }
}
