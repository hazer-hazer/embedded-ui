use core::marker::PhantomData;

use alloc::vec::Vec;
use embedded_canvas::{Canvas, CanvasAt};
use embedded_graphics::{
    draw_target::{Clipped, DrawTargetExt},
    geometry::{Dimensions, Point},
    image::{Image, ImageRaw},
    iterator::raw::RawDataSlice,
    mono_font::MonoTextStyle,
    pixelcolor::{raw::BigEndian, BinaryColor, PixelColor},
    primitives::{
        Arc, Circle, Line, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, RoundedRectangle,
        StyledDrawable,
    },
    Pixel,
};
use embedded_graphics_core::draw_target::DrawTarget;
use embedded_graphics_core::Drawable;
use embedded_text::TextBox;

use crate::{
    block::Block,
    color::UiColor,
    el::El,
    event::Event,
    font::{Font, FontFamily, FontStyle},
    size::Size,
};

#[derive(Clone, Copy)]
pub enum LayerKind {
    Normal,
    Clipped(Rectangle),
    Cropped(Rectangle),
}

pub trait Renderer {
    type Color: UiColor + Copy;

    // Renderer info
    fn clear(&mut self, color: Self::Color);

    fn clipped(&mut self, bounds: Rectangle, f: impl FnOnce(&mut Self));

    // Primitives //
    fn pixel(&mut self, pixel: Pixel<Self::Color>);
    fn line(&mut self, start: Point, end: Point, color: Self::Color, width: u32);

    // TODO: Own Arc, Circle and Sector structs might be needed
    fn arc(&mut self, arc: Arc, style: PrimitiveStyle<Self::Color>);
    fn circle(&mut self, circle: Circle, style: PrimitiveStyle<Self::Color>);

    // High-level primitives //
    fn block(&mut self, block: Block<Self::Color>);

    // Text //
    fn default_font() -> Font;
    fn mono_text<'a>(&mut self, text: TextBox<'a, MonoTextStyle<'a, Self::Color>>);

    // Images //
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

    fn clear(&mut self, _color: Self::Color) {}
    fn clipped(&mut self, _bounds: Rectangle, _f: impl FnOnce(&mut Self)) {}

    fn pixel(&mut self, _pixel: Pixel<Self::Color>) {}
    fn line(&mut self, _from: Point, _to: Point, _color: Self::Color, _width: u32) {}
    fn arc(&mut self, _arc: Arc, _style: PrimitiveStyle<Self::Color>) {}
    fn circle(&mut self, _circle: Circle, _style: PrimitiveStyle<Self::Color>) {}

    fn block(&mut self, _block: Block<Self::Color>) {}

    fn default_font() -> Font {
        Font {
            family: crate::font::FontFamily::Mono,
            size: crate::font::FontSize::Relative(1.0),
            style: FontStyle::Normal,
        }
    }
    fn mono_text<'a>(&mut self, _text: TextBox<'a, MonoTextStyle<'a, Self::Color>>) {}
    fn image<'a>(&mut self, _image: Image<'a, ImageRaw<'a, Self::Color>>)
    where
        RawDataSlice<'a, <Self::Color as PixelColor>::Raw, BigEndian>:
            IntoIterator<Item = <Self::Color as PixelColor>::Raw>,
    {
    }
}

pub struct DrawTargetRenderer<C: UiColor> {
    layers: Vec<LayerKind>,
    result: CanvasAt<C>,

    color: PhantomData<C>,
}

impl<C: UiColor> DrawTargetRenderer<C> {
    pub fn new(size: embedded_graphics_core::geometry::Size, default_bg: C) -> Self {
        Self {
            layers: vec![LayerKind::Normal],
            result: CanvasAt::with_default_color(Point::zero(), size, default_bg),
            color: PhantomData,
        }
    }

    pub fn finish<D>(self, target: &mut D)
    where
        D::Error: core::fmt::Debug,
        D: DrawTarget<Color = C>,
    {
        self.result.draw(target).unwrap();
    }
}

impl<C: UiColor> Dimensions for DrawTargetRenderer<C> {
    fn bounding_box(&self) -> Rectangle {
        self.result.bounding_box()
    }
}

impl<C: UiColor> DrawTarget for DrawTargetRenderer<C> {
    type Color = C;

    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        match self.layers.last().unwrap() {
            LayerKind::Normal => self.result.draw_iter(pixels),
            LayerKind::Clipped(bounds) => self.result.clipped(bounds).draw_iter(pixels),
            LayerKind::Cropped(bounds) => self.result.cropped(bounds).draw_iter(pixels),
        }
    }
}

impl<C: UiColor> Renderer for DrawTargetRenderer<C> {
    type Color = C;

    fn clear(&mut self, color: Self::Color) {
        DrawTarget::clear(self, color).unwrap()
    }

    fn clipped(&mut self, bounds: Rectangle, f: impl FnOnce(&mut Self)) {
        self.layers.push(LayerKind::Clipped(bounds));
        f(self);
        self.layers.pop();
    }

    fn pixel(&mut self, pixel: Pixel<Self::Color>) {
        pixel.draw(self).unwrap();
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
        let corner_radii = block.border.radius.into_corner_radii(block.rect.size.into());
        RoundedRectangle::new(block.rect, corner_radii)
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

    fn default_font() -> Font {
        Font {
            family: FontFamily::Mono,
            size: crate::font::FontSize::Relative(1.0),
            style: FontStyle::Normal,
        }
    }

    fn mono_text(&mut self, text: TextBox<'_, MonoTextStyle<'_, Self::Color>>) {
        text.draw(self).unwrap();
    }

    fn image<'b>(&mut self, image: Image<'b, ImageRaw<'b, Self::Color>>)
    where
        RawDataSlice<'b, <Self::Color as PixelColor>::Raw, BigEndian>:
            IntoIterator<Item = <Self::Color as PixelColor>::Raw>,
    {
        image.draw(self).unwrap();
    }
}
