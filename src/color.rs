use embedded_graphics::{
    iterator::raw::RawDataSlice,
    pixelcolor::{raw::BigEndian, BinaryColor},
};
use embedded_graphics_core::pixelcolor::PixelColor;

pub trait UiColor: PixelColor + From<<Self as PixelColor>::Raw> + Default
// where
//     RawDataSlice<'static, <Self as PixelColor>::Raw, BigEndian>:
//         IntoIterator<Item = <Self as PixelColor>::Raw>,
{
    fn default_background() -> Self;
    fn default_foreground() -> Self;
    fn transparent() -> Self;
}

impl UiColor for BinaryColor {
    fn default_background() -> Self {
        Self::Off
    }

    fn default_foreground() -> Self {
        Self::On
    }

    fn transparent() -> Self {
        Self::Off
    }
}
