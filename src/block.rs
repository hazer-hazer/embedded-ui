use embedded_graphics::primitives::{CornerRadii, Rectangle};

use crate::size::Size;

use crate::color::UiColor;

#[derive(Clone, Copy, Debug)]
pub enum Radius {
    Size(Size),
    SizeEqual(u32),
    Percentage(Size<f32>),
    PercentageEqual(f32),
}

impl Radius {
    pub fn into_real(self, corner_size: Size) -> embedded_graphics_core::geometry::Size {
        match self {
            Radius::Size(size) => size,
            Radius::SizeEqual(size) => Size::new_equal(size),
            Radius::Percentage(percentage) => corner_size * percentage,
            Radius::PercentageEqual(percentage) => corner_size * percentage,
        }
        .min(corner_size)
        .into()
    }
}

impl From<Size> for Radius {
    fn from(value: Size) -> Self {
        Self::Size(value)
    }
}

impl From<u32> for Radius {
    fn from(value: u32) -> Self {
        Self::SizeEqual(value)
    }
}

impl From<f32> for Radius {
    fn from(value: f32) -> Self {
        Self::PercentageEqual(value)
    }
}

impl From<Size<f32>> for Radius {
    fn from(value: Size<f32>) -> Self {
        Self::Percentage(value)
    }
}

impl From<(f32, f32)> for Radius {
    fn from(value: (f32, f32)) -> Self {
        Self::Percentage(Size::new(value.0, value.1))
    }
}

#[derive(Clone, Copy, Debug)]
pub struct BorderRadius {
    pub top_left: Radius,
    pub top_right: Radius,
    pub bottom_right: Radius,
    pub bottom_left: Radius,
}

impl BorderRadius {
    pub fn new(
        top_left: Radius,
        top_right: Radius,
        bottom_right: Radius,
        bottom_left: Radius,
    ) -> Self {
        Self { top_left, top_right, bottom_right, bottom_left }
    }

    pub fn new_equal(ellipse: Radius) -> Self {
        Self::new(ellipse, ellipse, ellipse, ellipse)
    }

    pub fn into_corner_radii(self, block_size: Size) -> CornerRadii {
        CornerRadii {
            top_left: self.top_left.into_real(block_size),
            top_right: self.top_right.into_real(block_size),
            bottom_right: self.bottom_right.into_real(block_size),
            bottom_left: self.bottom_left.into_real(block_size),
        }
    }
}

// impl Into<CornerRadii> for BorderRadius {
//     fn into(self) -> CornerRadii {
//         CornerRadii {
//             top_left: self.top_left.into(),
//             top_right: self.top_right.into(),
//             bottom_right: self.bottom_right.into(),
//             bottom_left: self.bottom_left.into(),
//         }
//     }
// }

impl From<u32> for BorderRadius {
    fn from(value: u32) -> Self {
        Self::new_equal(Radius::SizeEqual(value))
    }
}

impl From<[u32; 4]> for BorderRadius {
    fn from(value: [u32; 4]) -> Self {
        Self::new(
            Radius::SizeEqual(value[0]),
            Radius::SizeEqual(value[1]),
            Radius::SizeEqual(value[2]),
            Radius::SizeEqual(value[3]),
        )
    }
}

impl Default for BorderRadius {
    fn default() -> Self {
        Self::new_equal(Radius::SizeEqual(0))
    }
}

#[derive(Debug)]
pub struct Border<C: UiColor>
where
    C: Copy,
{
    pub color: C,
    pub width: u32,
    pub radius: BorderRadius,
}

impl<C: UiColor> Clone for Border<C> {
    fn clone(&self) -> Self {
        Self { color: self.color, width: self.width, radius: self.radius }
    }
}

impl<C: UiColor> Copy for Border<C> {}

impl<C: UiColor> Border<C> {
    pub fn new() -> Self {
        Self { color: C::default_foreground(), width: 1, radius: BorderRadius::default() }
    }
}

#[derive(Clone, Copy)]
pub struct Block<C: UiColor + Copy> {
    pub border: Border<C>,
    pub rect: Rectangle,
    pub background: C,
}
