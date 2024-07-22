use core::default;

#[derive(Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub enum Align {
    Start,
    Center,
    End,
}

#[derive(Clone, Copy, Default)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub enum HorizontalAlign {
    #[default]
    Left,
    Center,
    Right,
}

impl Into<embedded_graphics::text::Alignment> for HorizontalAlign {
    fn into(self) -> embedded_graphics::text::Alignment {
        match self {
            Self::Left => embedded_graphics::text::Alignment::Left,
            Self::Center => embedded_graphics::text::Alignment::Center,
            Self::Right => embedded_graphics::text::Alignment::Right,
        }
    }
}

impl Into<embedded_text::alignment::HorizontalAlignment> for HorizontalAlign {
    fn into(self) -> embedded_text::alignment::HorizontalAlignment {
        match self {
            Self::Left => embedded_text::alignment::HorizontalAlignment::Left,
            Self::Center => embedded_text::alignment::HorizontalAlignment::Center,
            Self::Right => embedded_text::alignment::HorizontalAlignment::Right,
        }
    }
}

#[derive(Clone, Copy, Default)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub enum VerticalAlign {
    #[default]
    Top,
    Center,
    Bottom,
}

impl Into<embedded_text::alignment::VerticalAlignment> for VerticalAlign {
    fn into(self) -> embedded_text::alignment::VerticalAlignment {
        match self {
            VerticalAlign::Top => embedded_text::alignment::VerticalAlignment::Top,
            VerticalAlign::Center => embedded_text::alignment::VerticalAlignment::Middle,
            VerticalAlign::Bottom => embedded_text::alignment::VerticalAlignment::Bottom,
        }
    }
}
