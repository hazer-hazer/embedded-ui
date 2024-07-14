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

macro_rules! schemes {
    (
        $($name:ident {
            $($color_name:ident: $color:expr),+ $(,)?
        })+
        {$($color_ty:ty: $conv_method:ident),+}
    ) => {
        impl PaletteColor for Rgba {
            $(
                const $name: Palette<Self> = Palette {
                    $($color_name: schemes!(@color $color)),+
                };
            )+
        }

        $(
            impl PaletteColor for $color_ty {
                $(
                    const $name: Palette<Self> = Palette {
                        $($color_name: <Rgba as PaletteColor>::$name.$conv_method()),*
                    };
                )+
            }
        )+
    };

    (@color $color: literal) => {
        Self::new_hex($color)
    };

    (@color ($r: expr, $g: expr, $b: expr, $a: expr)) => {
        Self::new($r, $g, $b, $a)
    };

    (@color ($r: expr, $g: expr, $b: expr)) => {
        Self::new_rgb($r, $g, $b)
    };
}

pub trait PaletteColor: UiColor {
    const LIGHT: Palette<Self>;
    const DARK: Palette<Self>;
    const AYU_LIGHT: Palette<Self>;
}

schemes! {
    LIGHT {
        background: 0xffffffFF,
        foreground: 0x000000FF,
        selection_background: 0x227DFCFF,
        selection_foreground: 0x000000FF,
        primary: 0xE086D3FF,
    }
    DARK {
        background: 0x000000FF,
        foreground: 0xffffffFF,
        selection_background: 0x227DFCFF,
        selection_foreground: 0xffffffFF,
        primary: 0xE086D3FF,
    }
    AYU_LIGHT {
        background: 0xFCFCFCFF,
        foreground: 0x5C6166FF,
        selection_background: 0x035BD6FF,
        selection_foreground: 0x5C6166FF,
        primary: 0xF2AE49FF,
    }
    {
        Rgb555: into_rgb555,
        Rgb565: into_rgb565,
        Rgb666: into_rgb666,
        Rgb888: into_rgb888
    }
}

// impl PaletteColor for Rgba {
//     const LIGHT: Palette<Self> = Palette {
//         background: Self::new_hex_rgb(0xffffff),
//         foreground: Self::new_hex_rgb(0x000000),
//         selection_background: Self::new_hex_rgb(0x227DFC),
//         selection_foreground: Self::new_hex_rgb(0x000000),
//         primary: Self::new_hex_rgb(0xE086D3),
//     };

//     const DARK: Palette<Self> = Palette {
//         background: Self::new_hex_rgb(0x000000),
//         foreground: Self::new_hex_rgb(0xffffff),
//         selection_background: Self::new_hex_rgb(0x227DFC),
//         selection_foreground: Self::new_hex_rgb(0xffffff),
//         primary: Self::new_hex_rgb(0xE086D3),
//     };

//     const AYU_LIGHT: Palette<Self> = Palette {
//         background: Self::new_hex_rgb(0xFCFCFC),
//         foreground: Self::new_hex_rgb(0x5C6166),
//         selection_background: Self::new_hex_rgb(0x035BD6),
//         selection_foreground: Self::new_hex_rgb(0x5C6166),
//         primary: Self::new_hex_rgb(0xF2AE49),
//     };
// }

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
