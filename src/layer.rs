use alloc::collections::BTreeMap;
use embedded_canvas::Canvas;
use embedded_graphics::{
    geometry::{Point, Size},
    image::{Image, ImageRaw},
    iterator::raw::RawDataSlice,
    mono_font::MonoTextStyle,
    pixelcolor::{raw::BigEndian, PixelColor},
    primitives::{Arc, Circle, PrimitiveStyle},
};

use crate::{block::Block, color::UiColor, render::Renderer};

pub struct Layering<C>
where
    C: UiColor,
{
    size: Size,
    z_index: i32,
    map: BTreeMap<i32, Canvas<C>>,
}

impl<C> Layering<C>
where
    C: UiColor,
{
    pub fn _new(size: Size) -> Self {
        Self { map: BTreeMap::from([(0, Canvas::new(size))]), z_index: 0, size }
    }

    fn layer(&mut self) -> &mut Canvas<C> {
        self.map.get_mut(&self.z_index).unwrap()
    }

    // fn base_layer(&mut self) -> &mut Canvas<C> {
    //     self.map.get_mut(&0).unwrap()
    // }
}

impl<C> Renderer for Layering<C>
where
    C: UiColor,
{
    type Color = C;

    fn clear(&mut self) {
        Renderer::clear(self.layer());
    }

    fn with_z_index(&mut self, z_index: i32, draw: impl Fn(&mut Self)) {
        let prev = self.z_index;
        self.z_index = z_index;

        if !self.map.contains_key(&self.z_index) {
            self.map.insert(self.z_index, Canvas::new(self.size));
        }

        draw(self);

        self.z_index = prev;
    }

    fn pixel(&mut self, point: Point, color: Self::Color) {
        Renderer::pixel(self.layer(), point, color);
    }

    fn line(&mut self, start: Point, end: Point, color: Self::Color, width: u32) {
        Renderer::line(self.layer(), start, end, color, width)
    }

    fn arc(&mut self, arc: Arc, style: PrimitiveStyle<Self::Color>) {
        Renderer::arc(self.layer(), arc, style)
    }

    fn circle(&mut self, circle: Circle, style: PrimitiveStyle<Self::Color>) {
        Renderer::circle(self.layer(), circle, style)
    }

    fn block(&mut self, block: Block<Self::Color>) {
        Renderer::block(self.layer(), block)
    }

    fn default_font() -> crate::font::Font {
        Canvas::<C>::default_font()
    }

    fn mono_text<'a>(&mut self, text: embedded_text::TextBox<'a, MonoTextStyle<'a, Self::Color>>) {
        Renderer::mono_text(self.layer(), text)
    }

    fn image<'a>(&mut self, image: Image<'a, ImageRaw<'a, Self::Color>>)
    where
        RawDataSlice<'a, <Self::Color as PixelColor>::Raw, BigEndian>:
            IntoIterator<Item = <Self::Color as PixelColor>::Raw>,
    {
        Renderer::image(self.layer(), image)
    }
}
