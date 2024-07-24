use embedded_graphics::mono_font::{
        ascii::{FONT_10X20, FONT_4X6, FONT_5X7, FONT_6X10, FONT_7X13, FONT_8X13, FONT_9X15},
        MonoFont,
    };

use crate::{layout::Viewport, size::Size};

const MIN_FONT_SIZE: u32 = 4;

#[derive(Clone, Copy)]
pub enum FontStyle {
    Normal,
    Italic,
    Bold,
    BoldItalic,
}

#[derive(Clone, Copy)]
pub enum FontSize {
    Relative(f32),
    Fixed(u32),
}

impl From<u32> for FontSize {
    fn from(value: u32) -> Self {
        Self::Fixed(value)
    }
}

impl From<f32> for FontSize {
    fn from(value: f32) -> Self {
        Self::Relative(value)
    }
}

impl FontSize {
    pub fn base_for_viewport(viewport: &Viewport) -> u32 {
        match viewport.size.width.min(viewport.size.height) {
            0..=64 => 4,
            65..=128 => 5,
            129..=180 => 6,
            181..=240 => 7,
            241..=320 => 8,
            321..=380 => 9,
            381.. => 16,
        }
    }

    pub fn to_real(&self, viewport: &Viewport) -> u32 {
        match self {
            FontSize::Relative(scale) => {
                ((Self::base_for_viewport(viewport) as f32 * scale) as u32).max(MIN_FONT_SIZE)
            },
            &FontSize::Fixed(fixed) => fixed,
        }
    }
}

#[derive(Clone, Copy)]
pub enum FontFamily {
    // Mono(&'static MonoFont<'static>),
    Mono,
}

impl FontFamily {
    pub fn to_real(&self, size: u32) -> RealFontFamily {
        match self {
            FontFamily::Mono => RealFontFamily::Mono(match size {
                0..=4 => &FONT_4X6,
                5 => &FONT_5X7,
                6 => &FONT_6X10,
                7 => &FONT_7X13,
                8 => &FONT_8X13,
                9 => &FONT_9X15,
                10.. => &FONT_10X20,
            }),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Font {
    pub family: FontFamily,
    pub size: FontSize,
    pub style: FontStyle,
}

impl Font {
    pub fn to_real(&self, viewport: &Viewport) -> RealFont {
        let size = self.size.to_real(viewport);
        let family = self.family.to_real(size);
        RealFont { family }
    }
}

struct FontMetrics {
    char_size: Size,
    char_space: u32,
}

#[derive(Clone, Copy)]
pub enum RealFontFamily {
    Mono(&'static MonoFont<'static>),
}

impl RealFontFamily {
    fn metrics(&self) -> FontMetrics {
        match self {
            RealFontFamily::Mono(mono) => FontMetrics {
                char_size: mono.character_size.into(),
                char_space: mono.character_spacing,
            },
        }
    }
}

/// The calculated font properties
pub struct RealFont {
    family: RealFontFamily,
}

impl RealFont {
    // TODO: Add text wrap strategy, also consider next line
    pub fn measure_text_size(&self, text: &str) -> Size {
        let FontMetrics { char_size, char_space } = self.family.metrics();

        // TODO: Optimize with single loop over chars
        let (lines_count, max_length) =
            text.split("\n").fold((0u32, 0u32), |(lines_count, max_length), s| {
                (lines_count + 1, (s.len() as u32).max(max_length))
            });

        // Dividing something linear N times, gives us N + 1 parts
        Size::new(
            max_length * char_size.width + (max_length.saturating_sub(1)) * char_space,
            lines_count * char_size.height,
        )
    }

    // pub fn measure_text_size(&self, text: &str) -> Size {
    //     match self.family {
    //         RealFontFamily::Mono(mono) => TextBoxStyleBuilder::new()
    //             .alignment(embedded_text::alignment::HorizontalAlignment::Center)
    //             .height_mode(embedded_text::style::HeightMode::FitToText)
    //             .line_height(embedded_graphics::text::LineHeight::Percent(100))
    //             .build().measure_text_height(MonoTextStyle::new(mono,
    //                 BinaryColor::Off)             .measure_string(text,
    // Point::zero(),                 embedded_graphics::text::Baseline::Top),
    // text, max_width),     }
    // }

    pub fn font(&self) -> &'static MonoFont<'static> {
        match self.family {
            RealFontFamily::Mono(mono) => mono,
        }
    }
}
