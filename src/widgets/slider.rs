use alloc::{boxed::Box, vec::Vec};
use embedded_graphics::primitives::{Line, Primitive, PrimitiveStyleBuilder, Rectangle};

use crate::{
    axis::{Axial, Axis},
    block::{Block, Border},
    color::UiColor,
    el::{El, ElId},
    event::{Capture, CommonEvent, Event, Propagate},
    layout::{Layout, Viewport},
    palette::PaletteColor,
    render::Renderer,
    size::{Length, Size},
    state::{State, StateNode, StateTag},
    style::component_style,
    theme::Theme,
    ui::UiCtx,
    widget::Widget,
};

#[derive(Clone, Copy)]
struct SliderState {
    active: bool,
    pressed: bool,
}

impl Default for SliderState {
    fn default() -> Self {
        Self { active: false, pressed: false }
    }
}

#[derive(Clone, Copy)]
pub struct SliderStatus {
    active: bool,
    pressed: bool,
    focused: bool,
}

component_style! {
    pub SliderStyle: SliderStyler(SliderStatus) default {primary} {
        // background: background,
        outline: outline,
    }
}

pub fn primary<C: PaletteColor>(theme: &Theme<C>, status: SliderStatus) -> SliderStyle<C> {
    let palette = theme.palette();
    let base = SliderStyle::new(&palette).outline_color(palette.background);

    match status {
        SliderStatus { active: true, pressed: _, focused: _ } => {
            base.outline_width(1).outline_radius(0)
        },
        SliderStatus { pressed: true, active: _, focused: _ } => {
            base.outline_color(palette.primary).outline_width(2).outline_radius(5)
        },
        SliderStatus { focused: true, active: _, pressed: _ } => {
            base.outline_color(palette.primary).outline_width(1).outline_radius(5)
        },
        SliderStatus { .. } => base.outline_width(1).outline_radius(0),
    }
}

pub type SliderPosition = u8;

pub struct Slider<'a, Message, R, S>
where
    R: Renderer,
    S: SliderStyler<R::Color>,
{
    axis: Axis,
    id: ElId,
    size: Size<Length>,
    value: u8,
    step: u8,
    // knob_icon: IconKind,
    on_change: Box<dyn Fn(SliderPosition) -> Message + 'a>,
    class: S::Class<'a>,
}

impl<'a, Message, R, S> Slider<'a, Message, R, S>
where
    R: Renderer,
    S: SliderStyler<R::Color>,
{
    pub fn new<F>(axis: Axis, on_change: F) -> Self
    where
        F: 'a + Fn(SliderPosition) -> Message,
    {
        Self {
            axis,
            id: ElId::unique(),
            size: Size::fill(),
            value: 0,
            step: 1,
            on_change: Box::new(on_change),
            class: S::default(),
        }
    }

    pub fn vertical<F>(on_change: F) -> Self
    where
        F: 'a + Fn(SliderPosition) -> Message,
    {
        Self::new(Axis::Y, on_change)
    }

    pub fn horizontal<F>(on_change: F) -> Self
    where
        F: 'a + Fn(SliderPosition) -> Message,
    {
        Self::new(Axis::X, on_change)
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.size.width = width.into();
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.size.height = height.into();
        self
    }

    pub fn step(mut self, step: u8) -> Self {
        self.step = step;
        self
    }

    // Helpers //
    fn status<E: Event>(&self, ctx: &UiCtx<Message>, state: &StateNode) -> SliderStatus {
        let &SliderState { active, pressed } = state.get::<SliderState>();
        let focused = UiCtx::is_focused::<R, E, S>(&ctx, self);

        SliderStatus { active, pressed, focused }
    }
}

impl<'a, Message, R, E, S> Widget<Message, R, E, S> for Slider<'a, Message, R, S>
where
    R: Renderer,
    E: Event,
    S: SliderStyler<R::Color>,
{
    fn id(&self) -> Option<ElId> {
        Some(self.id)
    }

    fn tree_ids(&self) -> Vec<ElId> {
        vec![self.id]
    }

    fn size(&self, _viewport: &Viewport) -> Size<Length> {
        self.size
    }

    fn state_tag(&self) -> crate::state::StateTag {
        StateTag::of::<SliderState>()
    }

    fn state(&self) -> State {
        State::new(SliderState::default())
    }

    fn state_children(&self) -> Vec<crate::state::StateNode> {
        vec![]
    }

    fn on_event(
        &mut self,
        ctx: &mut crate::ui::UiCtx<Message>,
        event: E,
        state: &mut crate::state::StateNode,
        _layout: Layout,
    ) -> crate::event::EventResponse<E> {
        let focused = ctx.is_focused::<R, E, S>(self);
        let current_state = *state.get::<SliderState>();

        if let Some(offset) = event.as_slider_shift() {
            if current_state.active {
                let prev_value = self.value;

                self.value = (self.value as i32)
                    .saturating_add(offset * self.step as i32)
                    .clamp(0, u8::MAX as i32) as u8;

                if prev_value != self.value {
                    ctx.publish((self.on_change)(self.value));
                }

                return Capture::Captured.into();
            }
        }

        // TODO: Generalize this focus logic for button, select and slider, etc.
        if let Some(common) = event.as_common() {
            match common {
                CommonEvent::FocusMove(_) if focused => {
                    return Propagate::BubbleUp(self.id, event).into()
                },
                CommonEvent::FocusButtonDown if focused => {
                    state.get_mut::<SliderState>().pressed = true;
                    return Capture::Captured.into();
                },
                CommonEvent::FocusButtonUp if focused => {
                    state.get_mut::<SliderState>().pressed = false;

                    if current_state.pressed {
                        state.get_mut::<SliderState>().active = !state.get::<SliderState>().active;
                        return Capture::Captured.into();
                    }
                },
                CommonEvent::FocusButtonDown
                | CommonEvent::FocusButtonUp
                | CommonEvent::FocusMove(_)
                | CommonEvent::Exit => {
                    // Should we reset state on any event? Or only on common
                    state.reset::<SliderState>();
                },
            }
        }

        Propagate::Ignored.into()
    }

    fn layout(
        &self,
        _ctx: &mut crate::ui::UiCtx<Message>,
        _state: &mut crate::state::StateNode,
        _styler: &S,
        limits: &crate::layout::Limits,
        viewport: &Viewport,
    ) -> crate::layout::LayoutNode {
        Layout::sized(limits, self.size, crate::layout::Position::Relative, viewport, |limits| {
            limits.resolve_size(self.size.width, self.size.height, Size::zero())
        })
    }

    fn draw(
        &self,
        ctx: &mut crate::ui::UiCtx<Message>,
        state: &mut crate::state::StateNode,
        renderer: &mut R,
        styler: &S,
        layout: crate::layout::Layout,
        _viewport: &Viewport,
    ) {
        let style = styler.style(&self.class, self.status::<E>(ctx, state));

        let state = state.get::<SliderState>();

        let bounds = layout.bounds();

        if bounds.size.width == 0 || bounds.size.height == 0 {
            return;
        }

        let position = bounds.top_left.into_axial(self.axis);
        let length = bounds.size.into_axial(self.axis);

        let guide_cross_axis_pos = position.cross() + (length.cross() / 2) as i32;

        let guide_start = self.axis.canon(position.main(), guide_cross_axis_pos);

        let guide_end =
            self.axis.canon(position.main() + length.main() as i32, guide_cross_axis_pos);

        // TODO: Style for guide
        renderer.line(
            Line::new(guide_start, guide_end).into_styled(
                PrimitiveStyleBuilder::new()
                    .stroke_color(R::Color::default_foreground())
                    .stroke_width(1)
                    .build(),
            ),
        );

        // let knob_size = Size::new_equal(bounds.size.width.min(bounds.size.height));
        let knob_size = Size::new_equal(5);
        // let (knob_center_main, knob_center_cross) = self.axis.canon(
        //     main_axis_pos + knob_size.width as i32 / 2,
        //     cross_axis_pos + knob_size.height as i32 / 2,
        // );

        let knob_shift_offset = self.value as u32 * length.main() / u8::MAX as u32;
        let knob_position =
            self.axis.canon(position.main() + knob_shift_offset as i32, guide_cross_axis_pos);

        let knob_background = if state.active {
            R::Color::default_foreground()
        } else {
            R::Color::default_background()
        };

        let knob = Block {
            border: Border { color: R::Color::default_foreground(), width: 1, radius: 0.into() },
            rect: Rectangle::with_center(knob_position, knob_size.into()),
            background: Some(knob_background),
        };

        renderer.block(knob);

        renderer.block(style.outline.into_outline(bounds));
    }
}

impl<'a, Message, R, E, S> From<Slider<'a, Message, R, S>> for El<'a, Message, R, E, S>
where
    Message: Clone + 'a,
    R: Renderer + 'a,
    E: Event + 'a,
    S: SliderStyler<R::Color> + 'a,
{
    fn from(value: Slider<'a, Message, R, S>) -> Self {
        El::new(value)
    }
}
