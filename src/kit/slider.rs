use embedded_graphics::{geometry::Point, primitives::Rectangle};

use crate::{
    align::Axis,
    block::{Block, Border},
    color::UiColor,
    el::{El, ElId},
    event::{Capture, CommonEvent, Event, Propagate},
    icons::IconKind,
    layout::Layout,
    render::Renderer,
    size::{Length, Size},
    state::{State, StateNode, StateTag},
    style::component_style,
    ui::UiCtx,
    widget::Widget,
};

#[derive(Clone, Copy)]
struct SliderState {
    is_active: bool,
    is_pressed: bool,
}

impl Default for SliderState {
    fn default() -> Self {
        Self { is_active: false, is_pressed: false }
    }
}

#[derive(Clone, Copy)]
pub enum SliderStatus {
    Normal,
    Focused,
    Pressed,
    Active,
}

component_style! {
    pub SliderStyle: SliderStyler(SliderStatus) {
        background: background,
        border: border,
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
        match state.get::<SliderState>() {
            SliderState { is_active: true, .. } => return SliderStatus::Active,
            SliderState { is_pressed: true, .. } => return SliderStatus::Pressed,
            SliderState { is_active: false, is_pressed: false } => {},
        }

        if UiCtx::is_focused::<R, E, S>(&ctx, self) {
            return SliderStatus::Focused;
        }

        SliderStatus::Normal
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

    fn size(&self) -> Size<Length> {
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
    ) -> crate::event::EventResponse<E> {
        let focused = ctx.is_focused::<R, E, S>(self);
        let current_state = *state.get::<SliderState>();

        if let Some(offset) = event.as_slider_shift() {
            if current_state.is_active {
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
                CommonEvent::FocusClickDown if focused => {
                    state.get_mut::<SliderState>().is_pressed = true;
                    return Capture::Captured.into();
                },
                CommonEvent::FocusClickUp if focused => {
                    state.get_mut::<SliderState>().is_pressed = false;

                    if current_state.is_pressed {
                        state.get_mut::<SliderState>().is_active =
                            !state.get::<SliderState>().is_active;
                        return Capture::Captured.into();
                    }
                },
                CommonEvent::FocusClickDown
                | CommonEvent::FocusClickUp
                | CommonEvent::FocusMove(_) => {
                    // Should we reset state on any event? Or only on common
                    state.reset::<SliderState>();
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
    ) -> crate::layout::LayoutNode {
        Layout::sized(limits, self.size, |limits| {
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
    ) {
        let style = styler.style(&self.class, self.status::<E>(ctx, state));

        let state = state.get::<SliderState>();

        let bounds = layout.bounds();

        if bounds.size.width == 0 || bounds.size.height == 0 {
            return;
        }

        renderer.block(&Block {
            border: style.border,
            rect: bounds.into(),
            background: style.background,
        });

        // TODO: WTF. Just finally add compound type `AxisData` for such calculations, looks buggy asf
        let (main_axis_pos, anti_axis_pos) = self.axis.canon(bounds.position.x, bounds.position.y);
        let (main_length, anti_length) = self.axis.canon(bounds.size.width, bounds.size.height);

        let guide_anti_axis_pos = anti_axis_pos + anti_length as i32 / 2;

        let guide_start_pos = self.axis.canon(main_axis_pos, guide_anti_axis_pos);
        let guide_start = Point::new(guide_start_pos.0, guide_start_pos.1);

        let guide_end_pos =
            self.axis.canon(main_axis_pos + main_length as i32, guide_anti_axis_pos);
        let guide_end = Point::new(guide_end_pos.0, guide_end_pos.1);

        // TODO: Style for guide
        renderer.line(guide_start, guide_end, R::Color::default_foreground(), 1);

        // let knob_size = Size::new_equal(bounds.size.width.min(bounds.size.height));
        let knob_size = Size::new_equal(5);
        // let (knob_center_main, knob_center_anti) = self.axis.canon(
        //     main_axis_pos + knob_size.width as i32 / 2,
        //     anti_axis_pos + knob_size.height as i32 / 2,
        // );

        let knob_shift_offset = self.value as u32 * main_length / u8::MAX as u32;
        let (knob_main_axis_pos, knob_anti_axis_pos) =
            self.axis.canon(main_axis_pos + knob_shift_offset as i32, guide_anti_axis_pos);

        let knob_background = if state.is_active {
            R::Color::default_foreground()
        } else {
            R::Color::default_background()
        };

        let knob = Block {
            border: Border { color: R::Color::default_foreground(), width: 1, radius: 0.into() },
            rect: Rectangle::with_center(
                Point::new(knob_main_axis_pos, knob_anti_axis_pos),
                knob_size.into(),
            ),
            background: knob_background,
        };

        renderer.block(&knob);
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
