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
use embedded_text::TextBox;

use crate::{block::Block, color::UiColor, render::Renderer};

pub struct Layering<'a, C>
where
    C: UiColor,
    RawDataSlice<'a, <C as PixelColor>::Raw, BigEndian>:
        IntoIterator<Item = <C as PixelColor>::Raw>,
{
    z_index: i32,
    map: BTreeMap<i32, Layer<'a, C>>,
}

impl<'a, C> Layering<'a, C>
where
    C: UiColor,
    RawDataSlice<'a, <C as PixelColor>::Raw, BigEndian>:
        IntoIterator<Item = <C as PixelColor>::Raw>,
{
    pub fn new() -> Self {
        Self { map: BTreeMap::from([(0, Layer::new())]), z_index: 0 }
    }

    // fn base_layer(&mut self) -> &mut Layer<'a, C, S> {
    //     self.map.get_mut(&0).unwrap()
    // }

    fn on_current(&mut self, command: DrawCommand<'a, C>) {
        self.map.get_mut(&0).unwrap().command(command);
    }
}

impl<'a, C> Renderer for Layering<'a, C>
where
    C: UiColor,
    RawDataSlice<'a, <C as PixelColor>::Raw, BigEndian>:
        IntoIterator<Item = <C as PixelColor>::Raw>,
{
    type Color = C;
    type Text = TextBox<'a, MonoTextStyle<'a, C>>;
    type Image = Image<'a, ImageRaw<'a, C>>;

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

    fn mono_text(&mut self, text: Self::Text) {
        self.on_current(DrawCommand::MonoText(text))
    }

    fn image(&mut self, image: Self::Image) {
        self.on_current(DrawCommand::Image(image))
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

pub struct Layer<'a, C>
where
    C: UiColor,
{
    commands: Vec<DrawCommand<'a, C>>,
}

impl<'a, C> Layer<'a, C>
where
    C: UiColor,
    RawDataSlice<'a, <C as PixelColor>::Raw, BigEndian>:
        IntoIterator<Item = <C as PixelColor>::Raw>,
{
    fn new() -> Self {
        Self { commands: vec![] }
    }

    fn command(&mut self, command: DrawCommand<'a, C>) {
        self.commands.push(command);
    }

    pub fn draw<D>(&self, target: &mut D)
    where
        D: Renderer<
            Color = C,
            Text = TextBox<'a, MonoTextStyle<'a, C>>,
            Image = Image<'a, ImageRaw<'a, C>>,
        >,
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
                &DrawCommand::Image(image) => target.image(image),
            }
        }
    }
}

pub enum DrawCommand<'a, C>
where
    C: UiColor,
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
    MonoText(TextBox<'a, MonoTextStyle<'a, C>>),

    // Images //
    Image(Image<'a, ImageRaw<'a, C>>),
}
