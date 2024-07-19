use core::fmt::Display;

use embedded_graphics::{
    pixelcolor::{raw::RawU32, BinaryColor, Rgb555, Rgb565, Rgb666, Rgb888, RgbColor},
    prelude::RawData,
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

    fn from_rgb(r: u8, g: u8, b: u8) -> Self;
    fn from_hex(hex: u32) -> Self {
        Self::from_rgb(
            (hex & 0xff0000 >> 4) as u8,
            (hex & 0x00ff00 >> 2) as u8,
            (hex & 0x0000ff) as u8,
        )
    }
    fn lightness(&self) -> f32;
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

    fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        let lightness =
            0.2126 * (r as f32 / 255.0) + 0.7152 * (g as f32 / 255.0) + 0.0722 * (b as f32 / 255.0);

        if lightness >= 0.5 {
            Self::On
        } else {
            Self::Off
        }
    }

    fn lightness(&self) -> f32 {
        match self {
            BinaryColor::Off => 0.0,
            BinaryColor::On => 1.0,
        }
    }
}

macro_rules! impl_rgb_colors {
    ($($ty: ty),*) => {
        $(
            impl UiColor for $ty {
                fn default_background() -> Self {
                    <$ty as embedded_graphics_core::pixelcolor::RgbColor>::BLACK
                }

                fn default_foreground() -> Self {
                    <$ty as embedded_graphics_core::pixelcolor::RgbColor>::WHITE
                }

                fn transparent() -> Self {
                    <$ty as embedded_graphics_core::pixelcolor::RgbColor>::BLACK
                }

                fn from_rgb(r: u8, g: u8, b: u8) -> Self {
                    Self::new(r, g, b)
                }

                fn lightness(&self) -> f32 {
                    0.2126 * (self.r() as f32 / Self::MAX_R as f32)
                        + 0.7152 * (self.g() as f32 / Self::MAX_G as f32)
                        + 0.0722 * (self.b() as f32 / Self::MAX_B as f32)
                }
            }
        )*
    };
}

impl_rgb_colors!(Rgb555, Rgb565, Rgb666, Rgb888);

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]

pub struct Rgba(u32);

impl Rgba {
    pub const WHITE: Self = Self::new_hex_rgb(0xFFFFFFFF);
    pub const BLACK: Self = Self::new_hex_rgb(0x00000000);
    pub const TRANSPARENT: Self = Self::new(0, 0, 0, 0);

    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self((r as u32) << 24 | (g as u32) << 16 | (b as u32) << 8 | a as u32)
    }

    pub const fn new_rgb(r: u8, g: u8, b: u8) -> Self {
        Self::new(r, g, b, 0xff)
    }

    pub fn from_f32(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self::new((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8, (a * 255.0) as u8)
    }

    pub const fn new_hex(rgba: u32) -> Self {
        Self(rgba)
    }

    pub const fn new_hex_rgb(rgb: u32) -> Self {
        Self(rgb << 8 | 0xff)
    }

    pub const fn r(self) -> u8 {
        (self.0 >> 24) as u8
    }

    pub const fn g(self) -> u8 {
        (self.0 >> 16) as u8
    }

    pub const fn b(self) -> u8 {
        (self.0 >> 8) as u8
    }

    pub const fn a(self) -> u8 {
        self.0 as u8
    }

    pub fn into_f32(self) -> (f32, f32, f32, f32) {
        (
            self.r() as f32 / 255.0,
            self.g() as f32 / 255.0,
            self.b() as f32 / 255.0,
            self.a() as f32 / 255.0,
        )
    }

    pub fn into_rgb(self, bg: Self) -> Self {
        let (r, g, b, a) = self.into_f32();
        let bg = bg.into_f32();
        let inv_a = 1.0 - a;
        Self::from_f32(inv_a * bg.0 + r * a, inv_a * bg.1 + g * a, inv_a * bg.2 + b * a, 1.0)
    }

    pub const fn into_rgb555(self) -> Rgb555 {
        Rgb555::new(self.r(), self.g(), self.b())
    }

    pub const fn into_rgb565(self) -> Rgb565 {
        Rgb565::new(self.r(), self.g(), self.b())
    }

    pub const fn into_rgb666(self) -> Rgb666 {
        Rgb666::new(self.r(), self.g(), self.b())
    }

    pub const fn into_rgb888(self) -> Rgb888 {
        Rgb888::new(self.r(), self.g(), self.b())
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for Rgba {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "{:X}", self.0)
    }
}

impl Display for Rgba {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "#{:08X}", self.0)
    }
}

impl PixelColor for Rgba {
    type Raw = RawU32;
}

impl From<RawU32> for Rgba {
    fn from(value: RawU32) -> Self {
        Self(value.into_inner())
    }
}

impl UiColor for Rgba {
    fn default_background() -> Self {
        Self::WHITE
    }

    fn default_foreground() -> Self {
        Self::BLACK
    }

    fn transparent() -> Self {
        Self::TRANSPARENT
    }

    fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self::new_rgb(r, g, b)
    }

    fn lightness(&self) -> f32 {
        0.2126 * (self.r() as f32 / 255.0)
            + 0.7152 * (self.g() as f32 / 255.0)
            + 0.0722 * (self.b() as f32 / 255.0)
    }
}
