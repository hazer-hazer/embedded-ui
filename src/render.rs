use core::{fmt::Display, marker::PhantomData};

use embedded_graphics::{
    geometry::Point,
    image::{Image, ImageRaw},
    iterator::raw::RawDataSlice,
    mono_font::MonoTextStyle,
    pixelcolor::{raw::BigEndian, BinaryColor, PixelColor},
    primitives::{
        Arc, Circle, Line, PrimitiveStyle, PrimitiveStyleBuilder, RoundedRectangle, StyledDrawable,
    },
    text::renderer::CharacterStyle,
    Pixel,
};
use embedded_graphics_core::Drawable;
use embedded_graphics_core::{draw_target::DrawTarget, primitives::Rectangle};
use embedded_text::TextBox;

use crate::{
    block::Block,
    color::UiColor,
    size::{Bounds, Size},
};

pub trait Renderer {
    type Color: UiColor + Copy;

    // Renderer info
    fn clear(&mut self);

    fn z_index(&self) -> i32 {
        0
    }

    #[inline]
    fn with_z_index(&mut self, z_index: i32, draw: impl Fn(&mut Self)) {
        let _z_index = z_index;
        draw(self)
    }

    #[inline]
    fn relative_z_index(&mut self, z_index_offset: i32, draw: impl Fn(&mut Self)) {
        self.with_z_index(self.z_index() + z_index_offset, draw);
    }

    #[inline]
    fn under(&mut self, draw: impl Fn(&mut Self)) {
        self.relative_z_index(-1, draw);
    }

    #[inline]
    fn above(&mut self, draw: impl Fn(&mut Self)) {
        self.relative_z_index(1, draw);
    }

    fn topmost(&mut self, draw: impl Fn(&mut Self)) {
        self.with_z_index(i32::MAX, draw);
    }

    // Primitives //
    fn pixel(&mut self, point: Point, color: Self::Color);
    fn line(&mut self, start: Point, end: Point, color: Self::Color, width: u32);

    // TODO: Own Arc, Circle and Sector structs might be needed
    fn arc(&mut self, arc: Arc, style: PrimitiveStyle<Self::Color>);
    fn circle(&mut self, circle: Circle, style: PrimitiveStyle<Self::Color>);

    // High-level primitives //
    fn block(&mut self, block: Block<Self::Color>);

    fn mono_text<'a>(&mut self, text: TextBox<'a, MonoTextStyle<'a, Self::Color>>);
    fn image<'a>(&mut self, image: Image<'a, ImageRaw<'a, Self::Color>>)
    where
        RawDataSlice<'a, <Self::Color as PixelColor>::Raw, BigEndian>:
            IntoIterator<Item = <Self::Color as PixelColor>::Raw>;
}

pub struct NullRenderer;

// impl<'a> TextRenderer<'a, MonoTextStyle<'a, BinaryColor>> for NullRenderer {
//     type Color = BinaryColor;

//     fn text(&mut self, _text: TextBox<'a, MonoTextStyle<'a, BinaryColor>>) {}
// }

impl Renderer for NullRenderer {
    type Color = BinaryColor;

    fn clear(&mut self) {}

    fn pixel(&mut self, _point: Point, _color: Self::Color) {}
    fn line(&mut self, _from: Point, _to: Point, _color: Self::Color, _width: u32) {}
    fn arc(&mut self, _arc: Arc, _style: PrimitiveStyle<Self::Color>) {}
    fn circle(&mut self, _circle: Circle, _style: PrimitiveStyle<Self::Color>) {}

    fn block(&mut self, _block: Block<Self::Color>) {}

    fn mono_text<'a>(&mut self, _text: TextBox<'a, MonoTextStyle<'a, Self::Color>>) {}
    fn image<'a>(&mut self, _image: Image<'a, ImageRaw<'a, Self::Color>>)
    where
        RawDataSlice<'a, <Self::Color as PixelColor>::Raw, BigEndian>:
            IntoIterator<Item = <Self::Color as PixelColor>::Raw>,
    {
    }
}

impl<D, C: UiColor> Renderer for D
where
    D: DrawTarget<Color = C>,
    D::Error: core::fmt::Debug,
{
    type Color = C;

    fn clear(&mut self) {
        self.clear(Self::Color::default_background()).unwrap()
    }

    fn pixel(&mut self, point: Point, color: Self::Color) {
        Pixel(point, color).draw(self).unwrap();
    }

    fn line(&mut self, start: Point, end: Point, color: Self::Color, width: u32) {
        Line::new(start, end)
            .draw_styled(
                &PrimitiveStyleBuilder::new().stroke_width(width).stroke_color(color).build(),
                self,
            )
            .unwrap();
    }

    fn arc(&mut self, arc: Arc, style: PrimitiveStyle<Self::Color>) {
        arc.draw_styled(&style, self).unwrap();
    }

    fn circle(&mut self, circle: Circle, style: PrimitiveStyle<Self::Color>) {
        circle.draw_styled(&style, self).unwrap();
    }

    fn block(&mut self, block: Block<Self::Color>)
    where
        Self: Sized,
    {
        RoundedRectangle::new(
            block.rect,
            block.border.radius.resolve_for_size(block.rect.size.into()).into(),
        )
        .draw_styled(
            &PrimitiveStyleBuilder::new()
                .fill_color(block.background)
                .stroke_color(block.border.color)
                .stroke_width(block.border.width)
                .build(),
            self,
        )
        .unwrap();
    }

    fn mono_text(&mut self, text: TextBox<'_, MonoTextStyle<'_, Self::Color>>) {
        text.draw(self).unwrap();
    }

    fn image<'a>(&mut self, image: Image<'a, ImageRaw<'a, Self::Color>>)
    where
        RawDataSlice<'a, <Self::Color as PixelColor>::Raw, BigEndian>:
            IntoIterator<Item = <Self::Color as PixelColor>::Raw>,
    {
        image.draw(self).unwrap();
    }
}
