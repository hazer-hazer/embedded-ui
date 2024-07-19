use embedded_graphics::pixelcolor::{BinaryColor, Rgb555, Rgb565, Rgb666, Rgb888, RgbColor};

use crate::color::{Rgba, UiColor};

#[derive(Clone, Copy)]
pub struct Palette<C: UiColor> {
    pub background: C,
    pub foreground: C,
    pub selection_background: C,
    pub selection_foreground: C,
    pub primary: C,
}

pub trait PaletteColor: UiColor {
    const LIGHT: Palette<Self>;
    const DARK: Palette<Self>;
    const AYU_LIGHT: Palette<Self>;
}

impl PaletteColor for Rgba {
    const LIGHT: Palette<Self> = Palette {
        background: Self::new_hex_rgb(0xffffff),
        foreground: Self::new_hex_rgb(0x000000),
        selection_background: Self::new_hex_rgb(0x227DFC),
        selection_foreground: Self::new_hex_rgb(0x000000),
        primary: Self::new_hex_rgb(0xE086D3),
    };

    const DARK: Palette<Self> = Palette {
        background: Self::new_hex_rgb(0x000000),
        foreground: Self::new_hex_rgb(0xffffff),
        selection_background: Self::new_hex_rgb(0x227DFC),
        selection_foreground: Self::new_hex_rgb(0xffffff),
        primary: Self::new_hex_rgb(0xE086D3),
    };

    const AYU_LIGHT: Palette<Self> = Palette {
        background: Self::new_hex_rgb(0xFCFCFC),
        foreground: Self::new_hex_rgb(0x5C6166),
        selection_background: Self::new_hex_rgb(0xD6E4F6),
        selection_foreground: Self::new_hex_rgb(0x5C6166),
        primary: Self::new_hex_rgb(0xF2AE49),
    };
}

impl PaletteColor for BinaryColor {
    const LIGHT: Palette<Self> = Palette {
        background: Self::On,
        foreground: Self::Off,
        selection_background: Self::Off,
        selection_foreground: Self::On,
        primary: Self::Off,
    };

    const DARK: Palette<Self> = Palette {
        background: Self::Off,
        foreground: Self::Off,
        selection_background: Self::On,
        selection_foreground: Self::Off,
        primary: Self::On,
    };

    const AYU_LIGHT: Palette<Self> = Self::LIGHT;
}

macro_rules! impl_from_rgba {
    ($($color_ty:ty : $conv_method:ident),*) => {
        $(
            impl PaletteColor for $color_ty {
                impl_from_rgba!(@theme LIGHT: $conv_method);
                impl_from_rgba!(@theme DARK: $conv_method);
                impl_from_rgba!(@theme AYU_LIGHT: $conv_method);
            }
        )*
    };

    (@theme $theme: ident: $conv_method: ident) => {
        const $theme: Palette<Self> = impl_from_rgba!(@palette $theme: $conv_method {
            background,
            foreground,
            selection_background,
            selection_foreground,
            primary,
        });
    };

    (@palette $theme: ident: $conv_method: ident {$($color_name: ident),* $(,)?}) => {
        Palette {
            $($color_name: impl_from_rgba!(@color $theme: $color_name: $conv_method)),*
        }
    };

    (@color $theme: ident: $name: ident: $conv_method: ident) => {
        <Rgba as PaletteColor>::$theme.$name.$conv_method()
    };
}

impl_from_rgba!(Rgb555: into_rgb555, Rgb565: into_rgb565, Rgb666: into_rgb666, Rgb888: into_rgb888);
