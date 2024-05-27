use crate::{
    el::El,
    event::Event,
    kit::{
        button::{Button, ButtonStyler},
        checkbox::{Checkbox, CheckboxStyler},
        divider::Divider,
        knob::{Knob, KnobStyler},
        select::{Select, SelectStyler},
        slider::{Slider, SliderPosition, SliderStyler},
    },
    render::Renderer,
    text::Text,
};

pub fn button<'a, Message: Clone, R: Renderer, E: Event, S: ButtonStyler<R::Color>>(
    content: impl Into<El<'a, Message, R, E, S>>,
) -> Button<'a, Message, R, E, S> {
    Button::new(content)
}

pub fn text<'a, R: Renderer>(content: impl Into<Text<'a, R>>) -> Text<'a, R> {
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

pub fn checkbox<'a, Message, R: Renderer, S: CheckboxStyler<R::Color>>(
    on_change: impl (Fn(bool) -> Message) + 'a,
) -> Checkbox<'a, Message, R, S> {
    Checkbox::new(on_change)
}

pub fn select<'a, Message: Clone, R: Renderer, E: Event, S: SelectStyler<R::Color>>(
    options: impl IntoIterator<Item = impl Into<El<'a, Message, R, E, S>>>,
) -> Select<'a, Message, R, E, S> {
    Select::new(options.into_iter().map(Into::into).collect())
}

pub fn slider_v<'a, Message: Clone, R: Renderer, S: SliderStyler<R::Color>>(
    on_change: impl (Fn(SliderPosition) -> Message) + 'a,
) -> Slider<'a, Message, R, S> {
    Slider::new(crate::align::Axis::Y, on_change)
}

pub fn slider_h<'a, Message: Clone, R: Renderer, S: SliderStyler<R::Color>>(
    on_change: impl (Fn(SliderPosition) -> Message) + 'a,
) -> Slider<'a, Message, R, S> {
    Slider::new(crate::align::Axis::X, on_change)
}

pub fn knob<'a, Message: Clone, R: Renderer, E: Event, S: KnobStyler<R::Color>>(
    on_change: impl (Fn(u8) -> Message) + 'a,
) -> Knob<'a, Message, R, E, S> {
    Knob::new(on_change)
}
