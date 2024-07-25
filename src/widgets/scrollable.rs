use embedded_graphics::{
    geometry::{AnchorPoint, Point},
    primitives::{Line, Primitive, PrimitiveStyle},
};

use crate::{
    axis::{Axial, Axis},
    block::{Block, BoxModel},
    el::{El, ElId},
    event::{Capture, CommonEvent, Event, Propagate},
    layout::{Layout, Limits, Viewport},
    padding::Padding,
    palette::PaletteColor,
    render::Renderer,
    size::{Length, Size},
    state::{State, StateNode, StateTag},
    style::component_style,
    theme::Theme,
    widget::Widget,
};

struct Scrollbar<'a> {
    child_layout: Layout<'a>,
    max_offset: u32,
    track: Line,
    thumb: Line,
    overflow: bool,
}

impl<'a> Scrollbar<'a> {
    fn new(axis: Axis, layout: Layout<'a>, offset: u32) -> Self {
        let bounds = layout.bounds();
        let child_layout = layout.first_child();

        let container_len = bounds.size.main_for(axis);
        let child_len = child_layout.bounds().size.main_for(axis);

        let track_len = container_len;
        let thumb_len = (track_len as f32 * (container_len as f32 / child_len as f32)) as u32;

        // Clamp offset to content length (width/height) minus scrollable viewport
        let max_offset = child_len.saturating_sub(container_len);
        let offset = offset.clamp(0, max_offset);

        let thumb_offset = ((track_len as f32 / child_len as f32) * offset as f32) as u32;

        let child_layout = child_layout.translated(axis.canon(-(offset as i32), 0));

        let thumb_start_anchor = match axis {
            Axis::X => AnchorPoint::BottomLeft,
            Axis::Y => AnchorPoint::TopRight,
        };
        let thumb_start =
            bounds.anchor_point(thumb_start_anchor) + Point::new(0, thumb_offset as i32);

        Self {
            child_layout,
            track: Line::new(
                bounds.anchor_point(thumb_start_anchor),
                bounds.anchor_point(AnchorPoint::BottomRight),
            ),
            thumb: Line::new(thumb_start, thumb_start + axis.canon::<Point>(thumb_len as i32, 0)),
            max_offset,
            overflow: child_len > container_len,
        }
    }
}

#[derive(Clone, Copy)]
struct ScrollableState {
    // TODO: Solve how to interface with 2D scrollable with encoder and add 2D
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
        outline: outline,
        track_color: color,
        thumb_color: color,
        scrollbar_width: width,
    }
}

pub fn default<C: PaletteColor>(theme: &Theme<C>, status: ScrollableStatus) -> ScrollableStyle<C> {
    let palette = theme.palette();
    let base = ScrollableStyle::new(&palette)
        .background(palette.background)
        .outline_color(palette.background)
        .outline_width(0)
        .thumb_color(palette.foreground)
        .track_color(palette.background)
        .scrollbar_width(3);

    let base = if status.focused { base.thumb_color(palette.primary) } else { base };

    match status {
        ScrollableStatus { pressed: true, active: _, focused: _ } => {
            base.outline_color(palette.selection_background).outline_width(2).outline_radius(4)
        },
        ScrollableStatus { active: true, pressed: _, focused: _ } => {
            base.outline_color(palette.selection_background).outline_width(1).outline_radius(4)
        },
        ScrollableStatus { focused: true, pressed: _, active: _ } => {
            base.outline_color(palette.selection_background).outline_width(1).outline_radius(2)
        },
        ScrollableStatus { .. } => base,
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
    axis: Axis,
    content: El<'a, Message, R, E, S>,
    size: Size<Length>,
    dir: ScrollDir,
    padding: Padding,
    always_show: bool,
    class: S::Class<'a>,
}

impl<'a, Message, R, E, S> Scrollable<'a, Message, R, E, S>
where
    R: Renderer,
    E: Event,
    S: ScrollableStyler<R::Color>,
{
    pub fn new(axis: Axis, content: impl Into<El<'a, Message, R, E, S>>) -> Self {
        let content: El<'a, Message, R, E, S> = content.into();

        /*
        TODO: Size depends on viewport and we don't know if content size is fluid on viewport change. So this arbitrary Viewport may cause some bugs, btw I don't know how to make it better without removing Widget::size dependency on Viewport.
        */
        let viewport = Viewport { size: Size::new(1080, 720) };
        // TODO: Better check that only scroll axis non-fill
        assert!(!content.size(&viewport).is_fill());

        Self {
            id: ElId::unique(),
            axis,
            content,
            size: Size::fill(),
            dir: ScrollDir::Vertical,
            padding: 1.into(),
            always_show: false,
            class: S::default(),
        }
    }

    pub fn always_show(mut self) -> Self {
        self.always_show = true;
        self
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

        let scrollbar = Scrollbar::new(self.axis, layout, current_state.offset);

        let event = if focused && current_state.active {
            // Child (content) receives events only if scrollable is focused
            let child_propagate = self.content.on_event(
                ctx,
                event.clone(),
                &mut state.children[0],
                scrollbar.child_layout,
            )?;

            let event = if let Propagate::Ignored = child_propagate {
                event
            } else {
                return child_propagate.into();
            };

            if let Some(offset) = event.as_scroll_offset() {
                if focused && current_state.active {
                    // TODO: on_scroll event message publish
                    state.get_mut::<ScrollableState>().offset = (current_state.offset as i32
                        + offset)
                        .clamp(0, scrollbar.max_offset as i32)
                        as u32;
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
                CommonEvent::FocusButtonDown if focused => {
                    state.get_mut::<ScrollableState>().pressed = true;
                    return Capture::Captured.into();
                },
                CommonEvent::FocusButtonUp if focused => {
                    let was_pressed = current_state.pressed;

                    let state_mut = state.get_mut::<ScrollableState>();

                    state_mut.pressed = false;

                    if was_pressed {
                        state_mut.active = !current_state.active;
                        return Capture::Captured.into();
                    }
                },
                CommonEvent::FocusButtonDown
                | CommonEvent::FocusButtonUp
                | CommonEvent::FocusMove(_)
                | CommonEvent::Exit => {
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
            BoxModel::new().padding(self.padding),
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

        let scrollbar = Scrollbar::new(self.axis, layout, state.offset);

        renderer.block(Block::new_background(bounds, style.background));

        renderer.clipped(bounds, |renderer| {
            self.content.draw(
                ctx,
                &mut state_tree.children[0],
                renderer,
                styler,
                scrollbar.child_layout,
                viewport,
            );
        });

        if scrollbar.overflow || self.always_show {
            renderer.line(scrollbar.track.into_styled(PrimitiveStyle::with_stroke(
                style.track_color,
                style.scrollbar_width,
            )));

            renderer.line(scrollbar.thumb.into_styled(PrimitiveStyle::with_stroke(
                style.thumb_color,
                style.scrollbar_width,
            )));
        }

        renderer.block(style.outline.into_outline(bounds));
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
