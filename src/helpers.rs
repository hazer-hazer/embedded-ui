use core::fmt::Display;

use crate::{
    el::El,
    event::Event,
    kit::{
        button::{Button, ButtonStyler},
        checkbox::{Checkbox, CheckboxStyler},
        divider::Divider,
        icon::IconStyler,
        knob::{Knob, KnobStyler, KnobValue},
        select::{Select, SelectStyler},
        slider::{Slider, SliderPosition, SliderStyler},
        text::{Text, TextStyler},
    },
    render::Renderer,
    value::Value,
};

pub fn button<'a, Message: Clone, R: Renderer, E: Event, S: ButtonStyler<R::Color>>(
    content: impl Into<El<'a, Message, R, E, S>>,
) -> Button<'a, Message, R, E, S> {
    Button::new(content)
}

pub fn text<'a, T: Display, R: Renderer, S: TextStyler<R::Color>>(
    content: impl Into<Text<'a, T, R, S>>,
) -> Text<'a, T, R, S> {
    content.into()
}

pub fn h_div<R: Renderer>() -> Divider<R> {
    Divider::horizontal()
}

pub fn v_div<R: Renderer>() -> Divider<R> {
    Divider::vertical()
}

#[macro_export]
macro_rules! col {
    ($($el: expr),* $(,)?) => [
        $crate::kit::linear::Column::new([$($crate::el::El::from($el)),*])
    ];
}

pub use col;

#[macro_export]
macro_rules! row {
    ($($el: expr),* $(,)?) => [
        $crate::kit::linear::Row::new([$($crate::el::El::from($el)),*])
    ];
}

pub use row;

pub fn checkbox<'a, Message, R, S>(
    on_change: impl (Fn(bool) -> Message) + 'a,
) -> Checkbox<'a, Message, R, S>
where
    R: Renderer + 'a,
    S: CheckboxStyler<R::Color> + IconStyler<R::Color> + 'a,
{
    Checkbox::new(on_change)
}

pub fn select<'a, Message: Clone, R: Renderer, E: Event, S>(
    options: impl IntoIterator<Item = impl Into<El<'a, Message, R, E, S>>>,
) -> Select<'a, Message, R, E, S, usize>
where
    S: SelectStyler<R::Color> + IconStyler<R::Color> + 'a,
{
    Select::new(options.into_iter().map(Into::into).enumerate())
}

pub fn select_keyed<'a, Message: Clone, R: Renderer, E: Event, S, V>(
    options: impl IntoIterator<Item = (V, impl Into<El<'a, Message, R, E, S>>)>,
) -> Select<'a, Message, R, E, S, V>
where
    S: SelectStyler<R::Color> + IconStyler<R::Color> + 'a,
{
    Select::new(options.into_iter().map(|(value, el)| (value, el.into())))
}

pub fn slider_v<'a, Message: Clone, R: Renderer, S: SliderStyler<R::Color>>(
    on_change: impl (Fn(SliderPosition) -> Message) + 'a,
) -> Slider<'a, Message, R, S> {
    Slider::new(crate::axis::Axis::Y, on_change)
}

pub fn slider_h<'a, Message: Clone, R: Renderer, S: SliderStyler<R::Color>>(
    on_change: impl (Fn(SliderPosition) -> Message) + 'a,
) -> Slider<'a, Message, R, S> {
    Slider::new(crate::axis::Axis::X, on_change)
}

// pub fn knob<'a, Message: Clone, R: Renderer, E: Event, S:
// KnobStyler<R::Color>>(     on_change: impl (Fn(u8) -> Message) + 'a,
// ) -> Knob<'a, Message, R, E, S> { Knob::new(on_change)
// }

pub fn knob<'a, Message: Clone, R: Renderer, E: Event, S: KnobStyler<R::Color>>(
    value: impl Into<Value<KnobValue>>,
) -> Knob<'a, Message, R, E, S> {
    Knob::new(value.into())
}
