use embedded_graphics::geometry::{AnchorPoint, Point};

use crate::{
    block::{Block, BoxModel},
    el::{El, ElId},
    event::{Capture, CommonEvent, Event, EventResponse, Propagate},
    layout::{Layout, Limits, Viewport},
    palette::PaletteColor,
    render::Renderer,
    size::{Length, Size, SizeExt},
    state::{State, StateNode, StateTag},
    style::{component_style, Styler},
    theme::Theme,
    widget::Widget,
};

#[derive(Clone, Copy)]
struct ScrollableState {
    // offset: Size,
    offset: u32,
    pressed: bool,
    active: bool,
}

impl Default for ScrollableState {
    fn default() -> Self {
        Self { offset: 0, pressed: false, active: false }
    }
}

pub struct ScrollableStatus {
    // scrolling: bool,
    pressed: bool,
    active: bool,
    focused: bool,
}

component_style! {
    pub ScrollableStyle: ScrollableStyler(ScrollableStatus) default {default} {
        background: background,
        color: color,
        // TODO: Should be outline
        border: border,
    }
}

pub fn default<C: PaletteColor>(theme: &Theme<C>, status: ScrollableStatus) -> ScrollableStyle<C> {
    let palette = theme.palette();
    let base = ScrollableStyle::new(&palette)
        .background(palette.background)
        .color(palette.primary)
        .border_color(palette.background);

    match status {
        ScrollableStatus { pressed: true, .. } => {
            base.border_color(palette.selection_background).border_width(2).border_radius(4)
        },
        ScrollableStatus { active: true, pressed: false, .. } => {
            base.border_color(palette.selection_background).border_width(1).border_radius(8)
        },
        ScrollableStatus { active: false, pressed: false, focused: true } => {
            base.border_color(palette.selection_background).border_width(1).border_radius(2)
        },
        ScrollableStatus { .. } => base.border_width(1).border_radius(0),
    }
}

#[derive(Clone, Copy)]
pub enum ScrollDir {
    None,
    Horizontal,
    Vertical,
    // Both,
}

impl ScrollDir {
    pub fn limits(&self, limits: &Limits) -> Limits {
        let (new_width, new_height) = match self {
            ScrollDir::None => (None, None),
            ScrollDir::Horizontal => (Some(u32::MAX), None),
            ScrollDir::Vertical => (None, Some(u32::MAX)),
            // ScrollDir::Both => (Some(u32::MAX), Some(u32::MAX)),
        };

        let new_max = limits.max();
        let new_max =
            if let Some(new_width) = new_width { new_max.new_width(new_width) } else { new_max };
        let new_max = if let Some(new_height) = new_height {
            new_max.new_height(new_height)
        } else {
            new_max
        };

        limits.with_max(new_max)
    }
}

pub struct Scrollable<'a, Message, R, E, S>
where
    R: Renderer,
    E: Event,
    S: ScrollableStyler<R::Color>,
{
    id: ElId,
    content: El<'a, Message, R, E, S>,
    size: Size<Length>,
    dir: ScrollDir,
    class: S::Class<'a>,
}

impl<'a, Message, R, E, S> Scrollable<'a, Message, R, E, S>
where
    R: Renderer,
    E: Event,
    S: ScrollableStyler<R::Color>,
{
    pub fn new(content: impl Into<El<'a, Message, R, E, S>>) -> Self {
        let content: El<'a, Message, R, E, S> = content.into();

        /*
        TODO: Size depends on viewport and we don't know if content size is fluid on viewport change. So this arbitrary Viewport may cause some bugs, btw I don't know how to make it better without removing Widget::size dependency on Viewport.
        */
        let viewport = Viewport { size: Size::new(1080, 720) };
        // TODO: Better check that only scroll axis non-fill
        assert!(!content.size(&viewport).is_fill());

        Self {
            id: ElId::unique(),
            content,
            size: Size::fill(),
            dir: ScrollDir::Vertical,
            class: S::default(),
        }
    }
}

impl<'a, Message, R, E, S> Widget<Message, R, E, S> for Scrollable<'a, Message, R, E, S>
where
    R: Renderer,
    E: Event,
    S: ScrollableStyler<R::Color>,
{
    fn id(&self) -> Option<ElId> {
        Some(self.id)
    }

    fn tree_ids(&self) -> alloc::vec::Vec<ElId> {
        let mut ids = vec![self.id];
        ids.extend(self.content.tree_ids());
        ids
    }

    fn size(&self, _viewport: &crate::layout::Viewport) -> Size<Length> {
        self.size
    }

    fn state_tag(&self) -> crate::state::StateTag {
        StateTag::of::<ScrollableState>()
    }

    fn state(&self) -> State {
        State::new(ScrollableState::default())
    }

    fn state_children(&self) -> alloc::vec::Vec<crate::state::StateNode> {
        vec![StateNode::new(&self.content)]
    }

    fn on_event(
        &mut self,
        ctx: &mut crate::ui::UiCtx<Message>,
        event: E,
        state: &mut StateNode,
        layout: Layout,
    ) -> crate::event::EventResponse<E> {
        let focused = ctx.is_focused(self);
        let current_state = *state.get::<ScrollableState>();

        let event = if focused && current_state.active {
            // Child (content) receives events only if scrollable is focused
            let child_propagate = self.content.on_event(
                ctx,
                event.clone(),
                &mut state.children[0],
                layout.first_child(),
            )?;

            let event = if let Propagate::Ignored = child_propagate {
                event
            } else {
                return child_propagate.into();
            };

            if let Some(offset) = event.as_scroll_offset() {
                if focused && current_state.active {
                    // TODO: on_scroll event message publish
                    state.get_mut::<ScrollableState>().offset =
                        (current_state.offset as i32 + offset).clamp(0, i32::MAX) as u32;
                    return Capture::Captured.into();
                }
            }

            event
        } else {
            event
        };

        // TODO: Generalized logic for focus+click components
        if let Some(common) = event.as_common() {
            match common {
                CommonEvent::FocusMove(_) if focused => {
                    return Propagate::BubbleUp(self.id, event).into()
                },
                CommonEvent::FocusClickDown if focused => {
                    state.get_mut::<ScrollableState>().pressed = true;
                    return Capture::Captured.into();
                },
                CommonEvent::FocusClickUp if focused => {
                    let was_pressed = current_state.pressed;

                    let state_mut = state.get_mut::<ScrollableState>();

                    state_mut.pressed = false;

                    if was_pressed {
                        state_mut.active = !current_state.active;
                        return Capture::Captured.into();
                    }
                },
                CommonEvent::FocusClickDown
                | CommonEvent::FocusClickUp
                | CommonEvent::FocusMove(_) => {
                    state.state.reset::<ScrollableState>();
                },
            }
        }

        Propagate::Ignored.into()
    }

    fn layout(
        &self,
        ctx: &mut crate::ui::UiCtx<Message>,
        state: &mut crate::state::StateNode,
        styler: &S,
        limits: &crate::layout::Limits,
        viewport: &crate::layout::Viewport,
    ) -> crate::layout::LayoutNode {
        Layout::container(
            limits,
            self.size,
            crate::layout::Position::Relative,
            viewport,
            BoxModel::new(),
            crate::align::Align::Start,
            crate::align::Align::Start,
            |limits| {
                let child_limits = self.dir.limits(limits);
                self.content.layout(ctx, &mut state.children[0], styler, &child_limits, viewport)
            },
        )
    }

    fn draw(
        &self,
        ctx: &mut crate::ui::UiCtx<Message>,
        state_tree: &mut crate::state::StateNode,
        renderer: &mut R,
        styler: &S,
        layout: crate::layout::Layout,
        viewport: &crate::layout::Viewport,
    ) {
        let bounds = layout.bounds();
        let state = state_tree.get::<ScrollableState>();
        let style = styler.style(
            &self.class,
            ScrollableStatus {
                pressed: state.pressed,
                active: state.active,
                focused: ctx.is_focused(self),
            },
        );

        let child_layout = layout.first_child().translated(Point::new(0, -(state.offset as i32)));
        let child_bounds = child_layout.bounds();

        let scrollbar_track_len = bounds.size.height;
        let scrollbar_thumb_len = (scrollbar_track_len as f32
            * (bounds.size.height as f32 / child_bounds.size.height as f32))
            as u32;

        let scrollbar_thumb_offset = ((scrollbar_track_len as f32
            / child_bounds.size.height as f32)
            * state.offset as f32) as u32;

        let thumb_start = bounds.anchor_point(AnchorPoint::TopRight)
            + Point::new(0, scrollbar_thumb_offset as i32);

        renderer.block(Block { border: style.border, rect: bounds, background: style.background });

        renderer.clipped(bounds, |renderer| {
            self.content.draw(
                ctx,
                &mut state_tree.children[0],
                renderer,
                styler,
                child_layout,
                viewport,
            );
        });

        renderer.line(
            bounds.anchor_point(AnchorPoint::TopRight),
            bounds.anchor_point(AnchorPoint::BottomRight),
            style.background,
            2,
        );

        renderer.line(
            thumb_start,
            thumb_start + Point::new(0, scrollbar_thumb_len as i32),
            style.color,
            2,
        )
    }
}

impl<'a, Message, R, E, S> From<Scrollable<'a, Message, R, E, S>> for El<'a, Message, R, E, S>
where
    Message: 'a,
    R: Renderer + 'a,
    E: Event + 'a,
    S: ScrollableStyler<R::Color> + 'a,
{
    fn from(value: Scrollable<'a, Message, R, E, S>) -> Self {
        El::new(value)
    }
}
