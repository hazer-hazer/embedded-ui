use core::{borrow::Borrow, fmt::Display};

use crate::{
    color::UiColor,
    el::El,
    event::Event,
    render::Renderer,
    widgets::{
        bar::{Bar, BarStyler},
        button::{Button, ButtonStyler},
        checkbox::{Checkbox, CheckboxStyler},
        container::{Container, ContainerStyler},
        divider::Divider,
        icon::IconStyler,
        knob::{Knob, KnobStyler, KnobValue},
        select::{Select, SelectStyler},
        slider::{Slider, SliderPosition, SliderStyler},
        text::{Text, TextStyler},
    },
};

pub fn button<'a, Message: Clone, C: UiColor, E: Event, S: ButtonStyler<C>>(
    content: impl Into<El<'a, Message, C, E, S>>,
) -> Button<'a, Message, C, E, S> {
    Button::new(content)
}

pub fn container<'a, Message: Clone, C: UiColor, E: Event, S: ContainerStyler<C>>(
    content: impl Into<El<'a, Message, C, E, S>>,
) -> Container<'a, Message, C, E, S> {
    Container::new(content)
}

pub fn text<'a, T: Display, C: UiColor, S: TextStyler<C>>(
    content: impl Into<Text<'a, T, C, S>>,
) -> Text<'a, T, C, S> {
    content.into()
}

pub fn h_div<C: UiColor>() -> Divider<C> {
    Divider::horizontal()
}

pub fn v_div<C: UiColor>() -> Divider<C> {
    Divider::vertical()
}

#[macro_export]
macro_rules! col {
    ($($el: expr),* $(,)?) => [
        $crate::widgets::linear::Column::new([$($crate::el::El::from($el)),*])
    ];
}

use alloc::string::ToString;
pub use col;

#[macro_export]
macro_rules! row {
    ($($el: expr),* $(,)?) => [
        $crate::widgets::linear::Row::new([$($crate::el::El::from($el)),*])
    ];
}

pub use row;

pub fn checkbox<'a, Message, C, S>(
    on_change: impl (Fn(bool) -> Message) + 'a,
) -> Checkbox<'a, Message, C, S>
where
    C: UiColor,
    S: CheckboxStyler<C> + IconStyler<C> + 'a,
{
    Checkbox::new(on_change)
}

pub fn select_h<'a, Message: Clone, C: UiColor, S, O, L>(
    options: L,
) -> Select<'a, Message, C, S, O, L>
where
    S: SelectStyler<C> + IconStyler<C> + 'a,
    O: ToString,
    L: Borrow<[O]>,
{
    Select::horizontal(options)
}

pub fn select_v<'a, Message: Clone, C: UiColor, S, O, L>(
    options: L,
) -> Select<'a, Message, C, S, O, L>
where
    S: SelectStyler<C> + IconStyler<C> + 'a,
    O: ToString,
    L: Borrow<[O]>,
{
    Select::vertical(options)
}

// pub fn select_h_keyed<'a, Message: Clone, C: UiColor, E: Event, S, V>(
//     options: impl IntoIterator<Item = impl Into<SelectOption<'a, Message, R,
// E, S, V>>>, ) -> Select<'a, Message, C, E, S, V>
// where
//     S: SelectStyler<C> + IconStyler<C> + 'a,
// {
//     Select::horizontal(options.into_iter())
// }

// pub fn select_v_keyed<'a, Message: Clone, C: UiColor, E: Event, S, V>(
//     options: impl IntoIterator<Item = impl Into<SelectOption<'a, Message, R,
// E, S, V>>>, ) -> Select<'a, Message, C, E, S, V>
// where
//     S: SelectStyler<C> + IconStyler<C> + 'a,
// {
//     Select::vertical(options.into_iter())
// }

pub fn slider_v<'a, Message: Clone, C: UiColor, S: SliderStyler<C>>(
    on_change: impl (Fn(SliderPosition) -> Message) + 'a,
) -> Slider<'a, Message, C, S> {
    Slider::new(crate::axis::Axis::Y, on_change)
}

pub fn slider_h<'a, Message: Clone, C: UiColor, S: SliderStyler<C>>(
    on_change: impl (Fn(SliderPosition) -> Message) + 'a,
) -> Slider<'a, Message, C, S> {
    Slider::new(crate::axis::Axis::X, on_change)
}

// pub fn knob<'a, Message: Clone, C: UiColor, E: Event, S:
// KnobStyler<C>>(     on_change: impl (Fn(u8) -> Message) + 'a,
// ) -> Knob<'a, Message, C, E, S> { Knob::new(on_change)
// }

pub fn knob<'a, Message: Clone, C: UiColor, E: Event, S: KnobStyler<C>>(
    on_change: impl (Fn(KnobValue) -> Message) + 'a,
) -> Knob<'a, Message, C, E, S> {
    Knob::new(on_change)
}

pub fn bar_h<'a, C: UiColor, S: BarStyler<C>>() -> Bar<'a, C, S> {
    Bar::horizontal()
}

pub fn bar_v<'a, C: UiColor, S: BarStyler<C>>() -> Bar<'a, C, S> {
    Bar::vertical()
}
