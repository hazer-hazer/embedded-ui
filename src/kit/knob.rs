use embedded_graphics::{
    geometry::Angle,
    primitives::{Arc, Circle, PrimitiveStyle},
};

use crate::{
    color::UiColor,
    el::{El, ElId},
    event::{Capture, CommonEvent, Event, Propagate},
    layout::{Layout, LayoutNode},
    padding::Padding,
    render::Renderer,
    size::{Length, Size},
    state::{State, StateTag},
    style::component_style,
    ui::UiCtx,
    value::Value,
    widget::Widget,
};

#[derive(Clone, Copy)]
struct KnobState {
    is_active: bool,
    is_pressed: bool,
}

impl Default for KnobState {
    fn default() -> Self {
        Self { is_active: false, is_pressed: false }
    }
}

#[derive(Clone, Copy)]
pub enum KnobStatus {
    Normal,
    Focused,
    Pressed,
    Active,
}

// TODO:
// - Color of value (filled track)
// - Color of track (not filled track)
// - Center color instead of background
component_style! {
    pub KnobStyle: KnobStyler(KnobStatus) {
        // background: background,
        center_color: color,
        color: color,
        track_color: color,
        track_width: width,
    }
}

pub type KnobValue = u8;

pub struct Knob<'a, Message, R, E, S>
where
    R: Renderer,
    E: Event,
    S: KnobStyler<R::Color>,
{
    id: ElId,
    diameter: Length,
    value: Value<KnobValue>,
    step: KnobValue,
    min: KnobValue,
    max: KnobValue,
    inner: Option<El<'a, Message, R, E, S>>,
    // TODO: Can be moved to style as it doesn't affect layout
    start: Angle,
    on_change: Option<Box<dyn Fn(KnobValue) -> Message + 'a>>,
    class: S::Class<'a>,
}

impl<'a, Message, R, E, S> Knob<'a, Message, R, E, S>
where
    R: Renderer,
    E: Event,
    S: KnobStyler<R::Color>,
{
    // pub fn new<F>(on_change: F) -> Self
    // where
    //     F: 'a + Fn(KnobValue) -> Message,
    // {
    //     Self {
    //         id: ElId::unique(),
    //         diameter: Length::Fill,
    //         value: ,
    //         step: 1,
    //         min: 0,
    //         max: KnobValue::MAX,
    //         inner: None,
    //         start: Angle::from_degrees(-90.0),
    //         on_change: Box::new(on_change),
    //         class: S::default(),
    //     }
    // }

    pub fn new(value: Value<KnobValue>) -> Self {
        Self {
            id: ElId::unique(),
            diameter: Length::Fill,
            value,
            step: 1,
            min: 0,
            max: KnobValue::MAX,
            inner: None,
            start: Angle::from_degrees(-90.0),
            on_change: None,
            class: S::default(),
        }
    }

    pub fn value(mut self, value: Value<KnobValue>) -> Self {
        self.value = value;
        self
    }

    pub fn min(mut self, min: KnobValue) -> Self {
        self.min = min;
        self
    }

    pub fn max(mut self, max: KnobValue) -> Self {
        self.max = max;
        self
    }

    pub fn step(mut self, step: KnobValue) -> Self {
        self.step = step;
        self
    }

    pub fn diameter(mut self, diameter: impl Into<Length>) -> Self {
        self.diameter = diameter.into();
        self
    }

    pub fn start(mut self, start: impl Into<Angle>) -> Self {
        self.start = start.into();
        self
    }

    pub fn inner(mut self, inner: impl Into<El<'a, Message, R, E, S>>) -> Self {
        self.inner = Some(inner.into());
        self
    }

    // Helpers //
    fn status(&self, ctx: &UiCtx<Message>, state: &KnobState) -> KnobStatus {
        let is_focused = ctx.is_focused(self);
        match (is_focused, state) {
            (_, KnobState { is_active: true, .. }) => KnobStatus::Active,
            (_, KnobState { is_pressed: true, .. }) => KnobStatus::Pressed,
            (true, KnobState { is_active: false, is_pressed: false }) => KnobStatus::Focused,
            (false, KnobState { is_active: false, is_pressed: false }) => KnobStatus::Normal,
        }
    }
}

impl<'a, Message, R, E, S> Widget<Message, R, E, S> for Knob<'a, Message, R, E, S>
where
    R: Renderer,
    E: Event,
    S: KnobStyler<R::Color>,
{
    fn id(&self) -> Option<ElId> {
        Some(self.id)
    }

    fn tree_ids(&self) -> Vec<ElId> {
        vec![self.id]
    }

    fn size(&self) -> crate::size::Size<crate::size::Length> {
        Size::new_equal(self.diameter)
    }

    fn state_tag(&self) -> crate::state::StateTag {
        StateTag::of::<KnobState>()
    }

    fn state(&self) -> crate::state::State {
        State::new(KnobState::default())
    }

    fn state_children(&self) -> Vec<crate::state::StateNode> {
        vec![]
    }

    fn on_event(
        &mut self,
        ctx: &mut UiCtx<Message>,
        event: E,
        state: &mut crate::state::StateNode,
    ) -> crate::event::EventResponse<E> {
        let focused = ctx.is_focused::<R, E, S>(self);
        let current_state = *state.get::<KnobState>();

        if let Some(offset) = event.as_knob_rotation() {
            if current_state.is_active {
                let prev_value = *self.value.get();

                *self.value.get_mut() = (prev_value as i32)
                    .saturating_add(offset * self.step as i32)
                    .clamp(self.min as i32, self.max as i32)
                    as u8;

                if let Some(on_change) = self.on_change.as_ref() {
                    if prev_value != *self.value.get() {
                        ctx.publish((on_change)(*self.value.get()));
                    }
                }

                return Capture::Captured.into();
            }
        }

        if let Some(common) = event.as_common() {
            match common {
                CommonEvent::FocusMove(_) if focused => {
                    return Propagate::BubbleUp(self.id, event).into()
                },
                CommonEvent::FocusClickDown if focused => {
                    state.get_mut::<KnobState>().is_pressed = true;
                    return Capture::Captured.into();
                },
                CommonEvent::FocusClickUp if focused => {
                    state.get_mut::<KnobState>().is_pressed = false;

                    if current_state.is_pressed {
                        state.get_mut::<KnobState>().is_active =
                            !state.get::<KnobState>().is_active;

                        return Capture::Captured.into();
                    }
                },
                CommonEvent::FocusClickDown
                | CommonEvent::FocusClickUp
                | CommonEvent::FocusMove(_) => {
                    // Should we reset state on any event? Or only on common
                    state.reset::<KnobState>();
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
        let size = Size::new_equal(self.diameter);
        Layout::container(
            limits,
            size,
            Padding::zero(),
            Padding::zero(),
            crate::align::Alignment::Center,
            crate::align::Alignment::Center,
            |limits| {
                if let Some(inner) = self.inner.as_ref() {
                    inner.layout(ctx, &mut state.children[0], styler, limits)
                } else {
                    LayoutNode::new(Size::zero())
                }
            },
        )
        // Layout::sized(limits, size, |limits| {
        //     limits.resolve_size(size.width, size.height, Size::zero())
        // })
    }

    fn draw(
        &self,
        ctx: &mut crate::ui::UiCtx<Message>,
        state_tree: &mut crate::state::StateNode,
        renderer: &mut R,
        styler: &S,
        layout: crate::layout::Layout,
    ) {
        let state = state_tree.get::<KnobState>();
        let status = self.status(ctx, state);
        let style = styler.style(&self.class, status);
        let bounds = layout.bounds();

        let outer_diameter = bounds.size.max_square();
        let track_diameter = outer_diameter - style.track_width - style.track_width / 2;

        let center = bounds.center();

        // TODO: Fix stroke drawing, offset by half of the stroke so it goes on outer bound of arc

        // Center circle
        renderer.circle(
            Circle::with_center(center, outer_diameter - style.track_width - style.track_width / 2),
            &PrimitiveStyle::with_fill(style.center_color),
        );

        if let Some(inner) = self.inner.as_ref() {
            inner.draw(
                ctx,
                &mut state_tree.children[0],
                renderer,
                styler,
                layout.children().next().unwrap(),
            );
        }

        // Whole track
        renderer.arc(
            Arc::with_center(center, track_diameter, self.start, Angle::from_degrees(360.0)),
            &PrimitiveStyle::with_stroke(style.track_color, style.track_width),
        );

        // TODO: Draw min/max serifs

        let value_degree = 360.0 * (*self.value.get() as f32 / u8::MAX as f32);

        renderer.arc(
            Arc::with_center(center, track_diameter, self.start, Angle::from_degrees(value_degree)),
            &PrimitiveStyle::with_stroke(style.color, style.track_width),
        );
    }
}

impl<'a, Message, R, E, S> From<Knob<'a, Message, R, E, S>> for El<'a, Message, R, E, S>
where
    Message: Clone + 'a,
    R: Renderer + 'a,
    E: Event + 'a,
    S: KnobStyler<R::Color> + 'a,
{
    fn from(value: Knob<'a, Message, R, E, S>) -> Self {
        El::new(value)
    }
}
