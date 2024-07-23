use alloc::vec::Vec;
use embedded_canvas::{Canvas, CanvasAt};
use embedded_graphics::{
    draw_target::DrawTargetExt,
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

pub enum LayerKind {
    Normal,
    Clipped(Rectangle),
    Cropped(Rectangle),
}

struct Layer<C: UiColor> {
    canvas: CanvasAt<C>,
}

impl<C: UiColor> Layer<C> {
    fn new(screen: Size, kind: LayerKind) -> Self {
        Self {
            canvas: match kind {
                LayerKind::Normal => CanvasAt::new(Point::zero(), screen.into()),
                LayerKind::Clipped(bounds) => CanvasAt::new(bounds.top_left, bounds.size),
                LayerKind::Cropped(_) => todo!(),
            },
        }
    }
}

impl<C: UiColor> Dimensions for Layer<C> {
    fn bounding_box(&self) -> Rectangle {
        self.canvas.bounding_box()
    }
}

impl<C: UiColor> DrawTarget for Layer<C> {
    type Color = C;

    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<C>>,
    {
        self.canvas.draw_iter(pixels)
    }
}

impl<C: UiColor> Drawable for Layer<C> {
    type Color = C;

    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        self.canvas.draw(target)
    }
}

pub struct Renderer<C: UiColor> {
    layers: Vec<Layer<C>>,
    result: CanvasAt<C>,
}

impl<C: UiColor> Dimensions for Renderer<C> {
    fn bounding_box(&self) -> Rectangle {
        self.layers.first().unwrap().bounding_box()
    }
}

impl<C: UiColor> DrawTarget for Renderer<C> {
    type Color = C;

    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        self.layer_mut().draw_iter(pixels)
    }
}

/*
 * Idea:
 * - Don't implement Renderer for DrawTarget
 * - Create structure storing DrawTarget and clip/crop state
 * - Implement draw methods so:
 * match self.state {
 *  Clipped(bounds) => DrawOn (self.clipped())
 * ...
 * }
 *
 * This cannot be generalized because:
 * - Function `state` cannot be implemented such as returning `impl Renderer`
 *   or `Self` because return values differ.
 * - We cannot make a match with different arms types
 *
 *
 * In some cases, like for Line, we can create StyledPixelsIterator from it
 * and draw this iterator.
 */

impl<C: UiColor> Renderer<C> {
    pub fn new(default_bg: C, target: &impl DrawTarget<Color = C>) -> Self {
        let screen_size = target.bounding_box().size.into();
        Self {
            layers: vec![Layer::new(screen_size, LayerKind::Normal)],
            result: CanvasAt::with_default_color(Point::zero(), screen_size.into(), default_bg),
        }
    }

    pub fn finish<D>(mut self, target: &mut D)
    where
        D: DrawTarget<Color = C>,
        D::Error: core::fmt::Debug,
    {
        self.end_layer();
        self.result.draw(target).unwrap();
    }

    fn layer_mut(&mut self) -> &mut Layer<C> {
        self.layers.last_mut().unwrap()
    }

    fn start_layer(&mut self, kind: LayerKind) {
        // FIXME: Remove useless into for sizes
        self.layers.push(Layer::new(self.bounding_box().size.into(), kind));
    }

    fn end_layer(&mut self) {
        let drawn = self.layers.pop().unwrap();
        drawn.draw(&mut self.result).unwrap();
    }

    pub fn with_layer(&mut self, kind: LayerKind, f: impl FnOnce(&mut Self)) {
        self.start_layer(kind);
        f(self);
        self.end_layer();
    }

    pub fn clear(&mut self, color: C) {
        DrawTarget::clear(self, color).unwrap()
    }

    // fn clipped(&mut self, bounds: Rectangle) -> Self::Clipped<'_> {
    //     DrawTargetExt::clipped(self, &bounds)
    // }

    // fn clipped(&mut self, bounds: Rectangle) -> Self::Clipped {
    //     DrawTargetExt::clipped(self, &bounds)
    // }

    pub fn pixel(&mut self, pixel: Pixel<C>) {
        pixel.draw(self).unwrap();
    }

    pub fn line(&mut self, start: Point, end: Point, color: C, width: u32) {
        Line::new(start, end)
            .draw_styled(
                &PrimitiveStyleBuilder::new().stroke_width(width).stroke_color(color).build(),
                self,
            )
            .unwrap();
    }

    pub fn arc(&mut self, arc: Arc, style: PrimitiveStyle<C>) {
        arc.draw_styled(&style, self).unwrap();
    }

    pub fn circle(&mut self, circle: Circle, style: PrimitiveStyle<C>) {
        circle.draw_styled(&style, self).unwrap();
    }

    pub fn block(&mut self, block: Block<C>)
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

    // fn default_font() -> Font {
    //     Font {
    //         family: FontFamily::Mono,
    //         size: crate::font::FontSize::Relative(1.0),
    //         style: FontStyle::Normal,
    //     }
    // }

    pub fn mono_text(&mut self, text: TextBox<'_, MonoTextStyle<'_, C>>) {
        text.draw(self).unwrap();
    }

    pub fn image<'a>(&mut self, image: Image<'a, ImageRaw<'a, C>>)
    where
        RawDataSlice<'a, <C as PixelColor>::Raw, BigEndian>:
            IntoIterator<Item = <C as PixelColor>::Raw>,
    {
        image.draw(self).unwrap();
    }
}
