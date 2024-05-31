use core::fmt::Display;

use alloc::collections::BTreeMap;
use embedded_graphics::{
    geometry::Point,
    image::{Image, ImageRaw},
    iterator::raw::RawDataSlice,
    mono_font::MonoTextStyle,
    pixelcolor::{raw::BigEndian, PixelColor},
    primitives::{Arc, Circle, PrimitiveStyle},
    text::renderer::CharacterStyle,
};

use crate::{block::Block, color::UiColor, render::Renderer, text::TextBox};

pub struct Layering<'a, Text, C>
where
    C: UiColor,
    Text: Display + Clone,
    RawDataSlice<'a, <C as PixelColor>::Raw, BigEndian>:
        IntoIterator<Item = <C as PixelColor>::Raw>,
{
    z_index: i32,
    map: BTreeMap<i32, Layer<'a, Text, C>>,
}

impl<'a, Text, C> Layering<'a, Text, C>
where
    C: UiColor,
    Text: Display + Clone,
    RawDataSlice<'a, <C as PixelColor>::Raw, BigEndian>:
        IntoIterator<Item = <C as PixelColor>::Raw>,
{
    pub fn new() -> Self {
        Self { map: BTreeMap::from([(0, Layer::new())]), z_index: 0 }
    }

    // fn base_layer(&mut self) -> &mut Layer<'a, C, S> {
    //     self.map.get_mut(&0).unwrap()
    // }

    fn on_current(&mut self, command: DrawCommand<'a, Text, C>) {
        self.map.get_mut(&0).unwrap().command(command);
    }
}

impl<'a, Text, C> Renderer for Layering<'a, Text, C>
where
    C: UiColor,
    Text: Display + Clone,
    RawDataSlice<'a, <C as PixelColor>::Raw, BigEndian>:
        IntoIterator<Item = <C as PixelColor>::Raw>,
{
    type Color = C;

    fn clear(&mut self) {
        // self.base_layer().command(DrawCommand::Clear)
        self.on_current(DrawCommand::Clear)
    }

    fn with_z_index(&mut self, z_index: i32, draw: impl Fn(&mut Self)) {
        let prev = self.z_index;
        self.z_index = z_index;

        draw(self);

        self.z_index = prev;
    }

    fn pixel(&mut self, point: Point, color: Self::Color) {
        self.on_current(DrawCommand::Pixel(point, color))
    }

    fn line(&mut self, start: Point, end: Point, color: Self::Color, width: u32) {
        self.on_current(DrawCommand::Line { start, end, color, width })
    }

    fn arc(&mut self, arc: Arc, style: PrimitiveStyle<Self::Color>) {
        self.on_current(DrawCommand::Arc { arc, style })
    }

    fn circle(&mut self, circle: Circle, style: PrimitiveStyle<Self::Color>) {
        self.on_current(DrawCommand::Circle { circle, style })
    }

    fn block(&mut self, block: Block<Self::Color>) {
        self.on_current(DrawCommand::Block(block))
    }

    fn mono_text<T: core::fmt::Display + Clone>(
        &mut self,
        text: crate::text::TextBox<T, Self::Color>,
    ) {
        self.on_current(DrawCommand::MonoText(text))
    }

    // fn text<'t>(&mut self, text: TextBox<'t, MonoTextStyle<'t, Self::Color>>)
    // where
    //     'a: 't,
    // {
    //     self.on_current(DrawCommand::MonoText(text))
    // }

    // fn image<'a>(&mut self, image: Image<'a, ImageRaw<'a, Self::Color>>)
    // where
    //     RawDataSlice<'a, <Self::Color as PixelColor>::Raw, BigEndian>:
    //         IntoIterator<Item = <Self::Color as PixelColor>::Raw>,
    // {
    // }
}

// impl<'a, C, S> TextRenderer<'a, S> for Layering<'a, C, S>
// where
//     C: UiColor + Copy,
//     S: embedded_graphics::text::renderer::TextRenderer<Color = C> + CharacterStyle<Color = C>,
//     RawDataSlice<'a, <C as PixelColor>::Raw, BigEndian>:
//         IntoIterator<Item = <C as PixelColor>::Raw>,
// {
//     type Color = C;

//     fn text(&mut self, text: TextBox<'a, S>) {
//         self.on_current(DrawCommand::Text(text))
//     }
// }

// impl<'a, C, S> ImageRenderer<'a> for Layering<'a, C, S>
// where
//     C: UiColor + Copy,
//     S: embedded_graphics::text::renderer::TextRenderer<Color = C> + CharacterStyle<Color = C>,
//     RawDataSlice<'a, <C as PixelColor>::Raw, BigEndian>:
//         IntoIterator<Item = <C as PixelColor>::Raw>,
// {
//     type Color = C;

//     fn image(&mut self, image: Image<'a, ImageRaw<'a, Self::Color>>) {
//         self.on_current(DrawCommand::Image(image))
//     }
// }

pub struct Layer<'a, Text, C>
where
    Text: Display + Clone,
    C: UiColor,
{
    commands: Vec<DrawCommand<'a, Text, C>>,
}

impl<'a, Text, C> Layer<'a, Text, C>
where
    C: UiColor,
    Text: Display + Clone,
    RawDataSlice<'a, <C as PixelColor>::Raw, BigEndian>:
        IntoIterator<Item = <C as PixelColor>::Raw>,
{
    fn new() -> Self {
        Self { commands: vec![] }
    }

    fn command(&mut self, command: DrawCommand<'a, Text, C>) {
        self.commands.push(command);
    }

    pub fn draw<D>(&self, target: &mut D)
    where
        D: Renderer<Color = C>,
    {
        for command in &self.commands {
            match command {
                DrawCommand::Clear => target.clear(),
                &DrawCommand::Pixel(point, color) => target.pixel(point, color),
                &DrawCommand::Line { start, end, color, width } => {
                    target.line(start, end, color, width)
                },
                &DrawCommand::Arc { arc, style } => target.arc(arc, style),
                &DrawCommand::Circle { circle, style } => target.circle(circle, style),
                &DrawCommand::Block(block) => target.block(block),
                DrawCommand::MonoText(text) => target.mono_text(text.clone()),
                // &DrawCommand::Image(image) => target.image(image),
                &DrawCommand::Image(image) => todo!(),
            }
        }
    }
}

pub enum DrawCommand<'a, Text, C>
where
    C: UiColor,
    Text: Display + Clone,
    // S: embedded_graphics::text::renderer::TextRenderer<Color = C> + CharacterStyle<Color = C>,
{
    Clear,
    // Primitives //
    Pixel(Point, C),
    Line { start: Point, end: Point, color: C, width: u32 },
    Arc { arc: Arc, style: PrimitiveStyle<C> },
    Circle { circle: Circle, style: PrimitiveStyle<C> },

    // High-level primitives //
    Block(Block<C>),
    MonoText(TextBox<'a, Text, C>),

    // Images //
    Image(Image<'a, ImageRaw<'a, C>>),
}
