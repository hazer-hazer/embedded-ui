use alloc::{boxed::Box, vec::Vec};
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
    el::El,
    event::Event,
    font::{Font, FontFamily, FontStyle},
    size::Size,
};

#[derive(Clone, Copy)]
pub enum LayerKind {
    FullScreen,
    Clipped(Rectangle),
    Cropped(Rectangle),
}

impl LayerKind {
    #[inline]
    fn target(&self, target: &mut impl DrawTarget) -> impl DrawTarget {
        match self {
            LayerKind::FullScreen => target,
            LayerKind::Clipped(bounds) => target.clipped(bounds),
            LayerKind::Cropped(bounds) => target.cropped(bounds),
        }
    }
}

// struct Layer<C: UiColor> {
//     canvas: CanvasAt<C>,
// }

// impl<C: UiColor> Layer<C> {
//     fn new(screen: Size, kind: LayerKind) -> Self {
//         Self {
//             canvas: match kind {
//                 LayerKind::FullScreen => CanvasAt::new(Point::zero(),
// screen.into()),                 LayerKind::Clipped(bounds) =>
// CanvasAt::new(bounds.top_left, bounds.size),
// LayerKind::Cropped(bounds) => todo!(),             },
//         }
//     }
// }

// impl<C: UiColor> DrawTarget for Layer<C> {
//     type Color = C;

//     type Error = ();

//     fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
//     where
//         I: IntoIterator<Item = Pixel<Self::Color>>,
//     {
//         self.canvas.draw_iter(pixels)
//     }
// }

pub struct Renderer<C: UiColor> {
    layers: Vec<LayerKind>,
}

impl<C: UiColor> Renderer<C> {
    fn layer(&mut self) -> &mut impl DrawTarget {
        self.layers.last().unwrap().target(self)
    }

    // Renderer info
    fn clear(&mut self, color: Self::Color);

    fn clipped(&mut self, bounds: Rectangle) -> Self::Clipped<'_>;

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
    type Clipped<'a> = Self;

    fn clear(&mut self, _color: Self::Color) {}
    fn clipped(&mut self, _bounds: Rectangle) -> Self::Clipped<'_> {
        NullRenderer
    }

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

impl<D, C: UiColor> Renderer for D
where
    D: DrawTarget<Color = C>,
    D::Error: core::fmt::Debug,
{
    type Color = C;
    type Clipped<'a> = Clipped<'a, Self> where Self: 'a;

    fn clear(&mut self, color: Self::Color) {
        DrawTarget::clear(self, color).unwrap()
    }

    fn clipped(&mut self, bounds: Rectangle) -> Self::Clipped<'_> {
        DrawTargetExt::clipped(self, &bounds)
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

    fn image<'a>(&mut self, image: Image<'a, ImageRaw<'a, Self::Color>>)
    where
        RawDataSlice<'a, <Self::Color as PixelColor>::Raw, BigEndian>:
            IntoIterator<Item = <Self::Color as PixelColor>::Raw>,
    {
        image.draw(self).unwrap();
    }
}
