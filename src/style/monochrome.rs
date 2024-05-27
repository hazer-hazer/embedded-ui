use embedded_graphics::pixelcolor::BinaryColor;

use crate::kit::{
    button::{ButtonStyle, ButtonStyler},
    checkbox::{CheckboxStyle, CheckboxStyler},
    knob::{KnobStyle, KnobStyler},
    select::{SelectStyle, SelectStyler},
    slider::{SliderStatus, SliderStyle, SliderStyler},
};

use super::Styler;

#[derive(Default)]
pub struct Monochrome;

#[derive(Clone, Copy)]
pub enum BinaryClass {
    Raw,
    Inverted,
}

impl Styler<BinaryColor> for Monochrome {}

impl ButtonStyler<BinaryColor> for Monochrome {
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
            crate::kit::button::ButtonStatus::Normal => ButtonStyle::new()
                .background(BinaryColor::Off)
                .border_color(BinaryColor::Off)
                .border_width(1)
                .border_radius(0),
            crate::kit::button::ButtonStatus::Focused => ButtonStyle::new()
                .background(BinaryColor::Off)
                .border_color(BinaryColor::On)
                .border_width(1)
                .border_radius(3),
            crate::kit::button::ButtonStatus::Pressed => ButtonStyle::new()
                .background(BinaryColor::Off)
                .border_color(BinaryColor::On)
                .border_width(2)
                .border_radius(5),
        }
    }
}

impl SelectStyler<BinaryColor> for Monochrome {
    type Class<'a> = BinaryClass;

    fn default<'a>() -> Self::Class<'a> {
        BinaryClass::Raw
    }

    fn style(
        &self,
        _class: &Self::Class<'_>,
        status: crate::kit::select::SelectStatus,
    ) -> crate::kit::select::SelectStyle<BinaryColor> {
        match status {
            crate::kit::select::SelectStatus::Normal => SelectStyle::new()
                .background(BinaryColor::Off)
                .border_color(BinaryColor::Off)
                .border_width(1)
                .border_radius(0),
            crate::kit::select::SelectStatus::Pressed
            | crate::kit::select::SelectStatus::Focused => SelectStyle::new()
                .background(BinaryColor::Off)
                .border_color(BinaryColor::On)
                .border_width(1)
                .border_radius(0),
            crate::kit::select::SelectStatus::Active => SelectStyle::new()
                .background(BinaryColor::Off)
                .border_color(BinaryColor::On)
                .border_width(1)
                .border_radius(5),
        }
    }
}

impl SliderStyler<BinaryColor> for Monochrome {
    type Class<'a> = BinaryClass;

    fn default<'a>() -> Self::Class<'a> {
        BinaryClass::Raw
    }

    fn style(&self, _class: &Self::Class<'_>, status: SliderStatus) -> SliderStyle<BinaryColor> {
        match status {
            SliderStatus::Normal => SliderStyle::new()
                .background(BinaryColor::Off)
                .border_color(BinaryColor::Off)
                .border_width(1)
                .border_radius(0),
            SliderStatus::Focused => SliderStyle::new()
                .background(BinaryColor::Off)
                .border_color(BinaryColor::On)
                .border_width(1)
                .border_radius(5),
            SliderStatus::Active => SliderStyle::new()
                .background(BinaryColor::Off)
                .border_color(BinaryColor::Off)
                .border_width(1)
                .border_radius(0),
            SliderStatus::Pressed => SliderStyle::new()
                .background(BinaryColor::Off)
                .border_color(BinaryColor::On)
                .border_width(2)
                .border_radius(5),
        }
    }
}

impl CheckboxStyler<BinaryColor> for Monochrome {
    type Class<'a> = BinaryClass;

    fn default<'a>() -> Self::Class<'a> {
        BinaryClass::Raw
    }

    fn style(
        &self,
        _class: &Self::Class<'_>,
        status: crate::kit::checkbox::CheckboxStatus,
    ) -> CheckboxStyle<BinaryColor> {
        match status {
            crate::kit::checkbox::CheckboxStatus::Normal => CheckboxStyle::new()
                .background(BinaryColor::Off)
                .border_color(BinaryColor::On)
                .border_width(1)
                .border_radius(0),
            crate::kit::checkbox::CheckboxStatus::Pressed => CheckboxStyle::new()
                .background(BinaryColor::Off)
                .border_color(BinaryColor::On)
                .border_width(2)
                .border_radius(5),
            crate::kit::checkbox::CheckboxStatus::Focused => CheckboxStyle::new()
                .background(BinaryColor::Off)
                .border_color(BinaryColor::On)
                .border_width(1)
                .border_radius(3),
            crate::kit::checkbox::CheckboxStatus::Checked => CheckboxStyle::new()
                .background(BinaryColor::Off)
                .border_color(BinaryColor::On)
                .border_width(1)
                .border_radius(0),
        }
    }
}

impl KnobStyler<BinaryColor> for Monochrome {
    type Class<'a> = BinaryClass;

    fn default<'a>() -> Self::Class<'a> {
        BinaryClass::Raw
    }

    fn style(
        &self,
        _class: &Self::Class<'_>,
        status: crate::kit::knob::KnobStatus,
    ) -> crate::kit::knob::KnobStyle<BinaryColor> {
        match status {
            crate::kit::knob::KnobStatus::Normal => KnobStyle::new()
                .center_color(BinaryColor::On)
                .color(BinaryColor::On)
                .track_color(BinaryColor::Off)
                .track_width(3),
            crate::kit::knob::KnobStatus::Focused => KnobStyle::new()
                .center_color(BinaryColor::On)
                .color(BinaryColor::On)
                .track_color(BinaryColor::Off)
                .track_width(4),
            crate::kit::knob::KnobStatus::Pressed => KnobStyle::new()
                .center_color(BinaryColor::On)
                .color(BinaryColor::On)
                .track_color(BinaryColor::Off)
                .track_width(3),
            crate::kit::knob::KnobStatus::Active => KnobStyle::new()
                .center_color(BinaryColor::On)
                .color(BinaryColor::On)
                .track_color(BinaryColor::Off)
                .track_width(3),
        }
    }
}
