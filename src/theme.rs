use embedded_graphics::pixelcolor::RgbColor;

use crate::{
    color::UiColor,
    palette::{Palette, PaletteColor},
    style::Styler,
};

#[derive(Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Theme<C: UiColor> {
    /// Normal Light theme looking good on all displays
    #[default]
    Light,

    /// Normal Dark theme looking good on all displays
    Dark,

    AyuLight,

    /// Custom user-defined theme
    Custom(CustomTheme<C>),
}

impl<C: UiColor> Theme<C>
where
    C: PaletteColor,
{
    #[inline]
    pub fn palette(&self) -> Palette<C> {
        match self {
            Theme::Light => PaletteColor::LIGHT,
            Theme::Dark => PaletteColor::DARK,
            Theme::AyuLight => PaletteColor::AYU_LIGHT,
            Theme::Custom(custom) => custom.palette,
        }
    }
}

impl<C: PaletteColor + 'static> Styler<C> for Theme<C> {
    fn background(&self) -> C {
        self.palette().background
    }
}

pub struct CustomTheme<C: UiColor> {
    palette: Palette<C>,
}
