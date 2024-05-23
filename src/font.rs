use embedded_graphics::mono_font::{ascii::FONT_4X6, MonoFont};

use crate::size::Size;

#[derive(Clone, Copy)]
pub enum Font {
    Mono(MonoFont<'static>),
}

impl Default for Font {
    fn default() -> Self {
        Self::Mono(FONT_4X6)
    }
}

impl Font {
    pub fn char_size(&self) -> Size {
        match self {
            Font::Mono(mono) => mono.character_size.into(),
        }
    }

    // TODO: Add text wrap strategy, also consider next line
    pub fn measure_text_size(&self, text: &str) -> Size {
        match self {
            Font::Mono(mono) => {
                let char_size = mono.character_size;
                let char_space = mono.character_spacing;

                // TODO: Optimize with single loop over chars
                let max_line = text.split("\n").map(|s| s.len()).max().unwrap_or(0) as u32;
                let lines_count = text.split("\n").count() as u32;

                // Dividing something linear N times, gives us N + 1 parts
                Size::new(
                    max_line * char_size.width + (max_line - 1) * char_space,
                    lines_count * char_size.height,
                )
            },
        }
    }
}
