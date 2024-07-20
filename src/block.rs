use embedded_graphics::primitives::{CornerRadii, Rectangle};

use crate::padding::Padding;
use crate::size::Size;

use crate::color::UiColor;

#[derive(Clone, Copy)]
pub struct BoxModel {
    pub margin: Padding,
    pub border: Padding,
    pub padding: Padding,
}

impl BoxModel {
    pub fn new() -> Self {
        Self { margin: Padding::zero(), border: Padding::zero(), padding: Padding::zero() }
    }

    pub fn margin(mut self, margin: impl Into<Padding>) -> Self {
        self.margin = margin.into();
        self
    }

    pub fn border(mut self, border: impl Into<Padding>) -> Self {
        self.border = border.into();
        self
    }

    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }
}

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

impl<T> From<T> for BorderRadius
where
    T: Into<Radius>,
{
    fn from(value: T) -> Self {
        Self::new_equal(value.into())
    }
}

impl<T> From<[T; 4]> for BorderRadius
where
    T: Into<Radius> + Copy,
{
    fn from(value: [T; 4]) -> Self {
        Self::new(value[0].into(), value[1].into(), value[2].into(), value[3].into())
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

    pub fn zero() -> Self {
        Self { color: C::default_foreground(), width: 0, radius: 0.into() }
    }

    pub fn color(mut self, color: impl Into<C>) -> Self {
        self.color = color.into();
        self
    }

    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    pub fn radius(mut self, radius: impl Into<BorderRadius>) -> Self {
        self.radius = radius.into();
        self
    }
}

impl<C: UiColor> Into<Padding> for Border<C> {
    fn into(self) -> Padding {
        self.width.into()
    }
}

#[derive(Clone, Copy)]
pub struct Block<C: UiColor + Copy> {
    pub border: Border<C>,
    pub rect: Rectangle,
    pub background: C,
}
