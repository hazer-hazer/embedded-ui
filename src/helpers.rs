use core::{borrow::Borrow, fmt::Display};

use crate::{
    axis::Axis,
    el::El,
    event::Event,
    kit::{
        bar::{Bar, BarStyler},
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

use alloc::string::ToString;
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

pub fn select_h<'a, Message: Clone, R: Renderer, S, O, L>(
    options: L,
) -> Select<'a, Message, R, S, O, L>
where
    S: SelectStyler<R::Color> + IconStyler<R::Color> + 'a,
    O: ToString,
    L: Borrow<[O]>,
{
    Select::horizontal(options)
}

pub fn select_v<'a, Message: Clone, R: Renderer, S, O, L>(
    options: L,
) -> Select<'a, Message, R, S, O, L>
where
    S: SelectStyler<R::Color> + IconStyler<R::Color> + 'a,
    O: ToString,
    L: Borrow<[O]>,
{
    Select::vertical(options)
}

// pub fn select_h_keyed<'a, Message: Clone, R: Renderer, E: Event, S, V>(
//     options: impl IntoIterator<Item = impl Into<SelectOption<'a, Message, R,
// E, S, V>>>, ) -> Select<'a, Message, R, E, S, V>
// where
//     S: SelectStyler<R::Color> + IconStyler<R::Color> + 'a,
// {
//     Select::horizontal(options.into_iter())
// }

// pub fn select_v_keyed<'a, Message: Clone, R: Renderer, E: Event, S, V>(
//     options: impl IntoIterator<Item = impl Into<SelectOption<'a, Message, R,
// E, S, V>>>, ) -> Select<'a, Message, R, E, S, V>
// where
//     S: SelectStyler<R::Color> + IconStyler<R::Color> + 'a,
// {
//     Select::vertical(options.into_iter())
// }

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

pub fn bar_h<'a, R: Renderer, S: BarStyler<R::Color>>() -> Bar<'a, R, S> {
    Bar::horizontal()
}

pub fn bar_v<'a, R: Renderer, S: BarStyler<R::Color>>() -> Bar<'a, R, S> {
    Bar::vertical()
}
