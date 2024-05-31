use core::fmt::Display;

use embedded_graphics::{mono_font::MonoTextStyle, primitives::Rectangle};
use embedded_text::style::TextBoxStyle;

use crate::{color::UiColor, value::Value};

#[derive(Clone)]
pub struct TextBox<'a, T, C: UiColor>
where
    T: Display + Clone,
{
    pub text: Value<T>,
    pub bounds: Rectangle,
    pub character_style: MonoTextStyle<'a, C>,
    pub style: TextBoxStyle,
    pub vertical_offset: i32,
}

// impl<T, C: UiColor> Into<embedded_text::TextBox<'static, MonoTextStyle<'static, C>>>
//     for TextBox<T, C>
// where
//     T: Display + 'static,
// {
//     fn into(self) -> embedded_text::TextBox<'static, MonoTextStyle<'static, C>> {
//         embedded_text::TextBox::with_textbox_style(
//             &self.text.get().to_string(),
//             self.bounds,
//             self.character_style,
//             self.style,
//         )
//     }
// }
