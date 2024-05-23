use crate::{
    icons::{IconData, InternalIcon, InternalIconSet, IntoIcon},
    kit::icon::Icon,
    render::Renderer,
    size::Size,
};

use super::{MonoIcon, MonoIconSet};

#[derive(Clone, Copy)]
pub struct Icons5x7;

impl Icons5x7 {
    pub const ARROW_LEFT: IconData =
        &[0b00001000, 0b00010000, 0b00100000, 0b01000000, 0b00100000, 0b00010000, 0b00001000];

    pub const ARROW_RIGHT: IconData =
        &[0b01000000, 0b00100000, 0b00010000, 0b00001000, 0b00010000, 0b00100000, 0b01000000];

    pub fn arrow_left<R: Renderer>(&self) -> Icon<'_, R> {
        MonoIcon::<{ Self::SIZE.width }, { Self::SIZE.height }> { data: Self::ARROW_LEFT }
            .into_icon()
    }

    pub fn arrow_right<R: Renderer>(&self) -> Icon<'_, R> {
        MonoIcon::<{ Self::SIZE.width }, { Self::SIZE.height }> { data: Self::ARROW_RIGHT }
            .into_icon()
    }
}

impl MonoIconSet for Icons5x7 {
    const SIZE: Size = Size::new(5, 7);
}

impl<R: Renderer> InternalIconSet<R> for Icons5x7 {
    fn internal(icon: InternalIcon) -> impl crate::icons::IntoIcon<R> {
        MonoIcon::<{ Self::SIZE.width }, { Self::SIZE.height }> {
            data: match icon {
                InternalIcon::ArrowLeft => Self::ARROW_LEFT,
                InternalIcon::ArrowRight => Self::ARROW_RIGHT,
            },
        }
    }
}
