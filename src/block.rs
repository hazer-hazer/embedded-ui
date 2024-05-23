use embedded_graphics::primitives::{CornerRadii, Rectangle};

use crate::render::Renderer;
use crate::size::Size;

use crate::color::UiColor;

#[derive(Clone, Copy, Debug)]
pub struct BorderRadius {
    pub top_left: Size,
    pub top_right: Size,
    pub bottom_right: Size,
    pub bottom_left: Size,
}

impl BorderRadius {
    pub fn new(top_left: Size, top_right: Size, bottom_right: Size, bottom_left: Size) -> Self {
        Self { top_left, top_right, bottom_right, bottom_left }
    }

    pub fn new_equal(ellipse: Size) -> Self {
        Self::new(ellipse, ellipse, ellipse, ellipse)
    }
}

impl Into<CornerRadii> for BorderRadius {
    fn into(self) -> CornerRadii {
        CornerRadii {
            top_left: self.top_left.into(),
            top_right: self.top_right.into(),
            bottom_right: self.bottom_right.into(),
            bottom_left: self.bottom_left.into(),
        }
    }
}

impl From<u32> for BorderRadius {
    fn from(value: u32) -> Self {
        Self::new_equal(Size::new_equal(value))
    }
}

impl From<[u32; 4]> for BorderRadius {
    fn from(value: [u32; 4]) -> Self {
        Self::new(
            Size::new_equal(value[0]),
            Size::new_equal(value[1]),
            Size::new_equal(value[2]),
            Size::new_equal(value[3]),
        )
    }
}

impl Default for BorderRadius {
    fn default() -> Self {
        Self::new_equal(Size::zero())
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
