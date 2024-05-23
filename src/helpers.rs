use crate::{
    el::El,
    event::Event,
    kit::{
        button::{Button, ButtonStyler},
        checkbox::Checkbox,
        divider::Divider,
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
        $crate::linear::Column::new([$($crate::el::El::from($el)),*])
    ];
}

pub use col;

#[macro_export]
macro_rules! row {
    ($($el: expr),* $(,)?) => [
        $crate::linear::Row::new([$($crate::el::El::from($el)),*])
    ];
}

pub use row;

pub fn checkbox<R: Renderer>() -> Checkbox<R> {
    Checkbox::new()
}
