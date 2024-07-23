use alloc::vec::Vec;
use embedded_canvas::{Canvas, CanvasAt};
use embedded_graphics::{
    draw_target::{Clipped, DrawTargetExt},
    geometry::Point,
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
    font::{Font, FontFamily, FontStyle},
    size::Size,
};

#[derive(Clone, Copy)]
pub enum LayerKind {
    FullScreen,
    Clipped(Rectangle),
    Cropped(Rectangle),
}

pub trait Renderer {
    type Color: UiColor + Copy;

    // Renderer info
    fn clear(&mut self, color: Self::Color);

    fn start_layer(&mut self, kind: LayerKind);
    fn end_layer(&mut self);

    fn with_layer(&mut self, kind: LayerKind, f: impl FnOnce(&mut Self)) {
        self.start_layer(kind);
        f(self);
        self.end_layer();
    }

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

    fn start_layer(&mut self, _kind: LayerKind) {}
    fn end_layer(&mut self) {}

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

pub struct Layering<C: UiColor> {
    size: Size,
    layers: Vec<CanvasAt<C>>,
}

impl<C: UiColor> Layering<C> {
    pub fn new(size: Size) -> Self {
        Self { size, layers: vec![CanvasAt::new(Point::zero(), size.into())] }
    }

    fn layer(&mut self) -> &mut CanvasAt<C> {
        self.layers.last_mut().unwrap()
    }
}

impl<C: UiColor> Renderer for Layering<C> {
    type Color = C;

    fn clear(&mut self, color: Self::Color) {
        self.layer().clear(color).unwrap()
    }

    fn start_layer(&mut self, kind: LayerKind) {
        let layer = match kind {
            LayerKind::FullScreen => CanvasAt::new(Point::zero(), self.size.into()),
            LayerKind::Clipped(bounds) => CanvasAt::new(bounds.top_left, bounds.size),
            LayerKind::Cropped(_) => todo!(),
        };

        self.layers.push(layer);
    }

    fn end_layer(&mut self) {
        self.layers.pop();
    }

    fn pixel(&mut self, pixel: Pixel<Self::Color>) {
        pixel.draw(self.layer()).unwrap();
    }

    fn line(&mut self, start: Point, end: Point, color: Self::Color, width: u32) {
        Line::new(start, end)
            .draw_styled(
                &PrimitiveStyleBuilder::new().stroke_width(width).stroke_color(color).build(),
                self.layer(),
            )
            .unwrap();
    }

    fn arc(&mut self, arc: Arc, style: PrimitiveStyle<Self::Color>) {
        arc.draw_styled(&style, self.layer()).unwrap();
    }

    fn circle(&mut self, circle: Circle, style: PrimitiveStyle<Self::Color>) {
        circle.draw_styled(&style, self.layer()).unwrap();
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
                self.layer(),
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
        text.draw(self.layer()).unwrap();
    }

    fn image<'a>(&mut self, image: Image<'a, ImageRaw<'a, Self::Color>>)
    where
        RawDataSlice<'a, <Self::Color as PixelColor>::Raw, BigEndian>:
            IntoIterator<Item = <Self::Color as PixelColor>::Raw>,
    {
        image.draw(self.layer()).unwrap();
    }
}
