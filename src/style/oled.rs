use embedded_graphics::pixelcolor::BinaryColor;

use crate::kit::button::{ButtonStyle, ButtonStyler};

use super::Styler;

#[derive(Default)]
pub struct MonochromeOled;

#[derive(Clone, Copy)]
pub enum BinaryClass {
    Raw,
    Inverted,
}

impl ButtonStyler<BinaryColor> for MonochromeOled {
    type Class<'a> = BinaryClass;

    fn default<'a>() -> Self::Class<'a> {
        BinaryClass::Raw
    }

    fn style(
        &self,
        _class: &Self::Class<'_>,
        status: crate::kit::button::ButtonStatus,
    ) -> crate::kit::button::ButtonStyle<BinaryColor> {
        match status {
            crate::kit::button::ButtonStatus::Active => ButtonStyle::new()
                .background(BinaryColor::Off)
                .border_color(BinaryColor::Off)
                .border_width(0)
                .border_radius(0),
            crate::kit::button::ButtonStatus::Focused => ButtonStyle::new()
                .background(BinaryColor::Off)
                .border_color(BinaryColor::On)
                .border_width(1)
                .border_radius(0),
            crate::kit::button::ButtonStatus::Pressed => ButtonStyle::new()
                .background(BinaryColor::Off)
                .border_color(BinaryColor::On)
                .border_width(1)
                .border_radius(0),
        }
    }
}

impl Styler<BinaryColor> for MonochromeOled {}
