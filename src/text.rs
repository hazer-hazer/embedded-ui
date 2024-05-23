use embedded_graphics::{
    mono_font::MonoTextStyleBuilder,
    text::renderer::{CharacterStyle, TextRenderer},
};
use embedded_text::{style::TextBoxStyleBuilder, TextBox};

use crate::{
    align::{HorizontalAlign, VerticalAlign},
    el::El,
    event::Event,
    font::Font,
    layout::Layout,
    render::Renderer,
    size::{Length, Size},
    state::StateNode,
    ui::UiCtx,
    widget::Widget,
};

use crate::color::UiColor;

#[derive(Clone, Copy, Debug)]
pub enum LineHeight {
    Pixels(u32),
    Percent(u32),
}

impl Into<embedded_graphics::text::LineHeight> for LineHeight {
    fn into(self) -> embedded_graphics::text::LineHeight {
        match self {
            LineHeight::Pixels(pixels) => embedded_graphics::text::LineHeight::Pixels(pixels),
            LineHeight::Percent(percent) => embedded_graphics::text::LineHeight::Percent(percent),
        }
    }
}

impl Default for LineHeight {
    fn default() -> Self {
        Self::Percent(100)
    }
}

impl From<u32> for LineHeight {
    fn from(value: u32) -> Self {
        Self::Pixels(value)
    }
}

impl From<f32> for LineHeight {
    fn from(value: f32) -> Self {
        Self::Percent((value.clamp(0.0, 1.0) * 100.0) as u32)
    }
}

pub struct Text<'a, R>
where
    R: Renderer,
{
    content: &'a str,

    // style: TextStyle<R::Color>,
    align: HorizontalAlign,
    vertical_align: VerticalAlign,
    line_height: LineHeight,
    text_color: R::Color,
    font: Font,

    /// Precomputed size, does not need to be set by user
    size: Size<Length>,
}

impl<'a, R: Renderer> Text<'a, R> {
    pub fn new(content: &'a str) -> Self {
        let font = Font::default();

        Self {
            content,
            // style: TextStyle {
            //     font: font,
            //     text_color: R::Color::default_foreground(),
            // },
            text_color: R::Color::default_foreground(),
            line_height: LineHeight::default(),
            align: HorizontalAlign::Center,
            vertical_align: VerticalAlign::Center,
            font,
            size: Size::fill(),
        }
    }

    pub fn text_color(mut self, text_color: R::Color) -> Self {
        self.text_color = text_color;
        self
    }

    pub fn align(mut self, align: HorizontalAlign) -> Self {
        self.align = align;
        self
    }

    pub fn vertical_align(mut self, vertical_align: VerticalAlign) -> Self {
        self.vertical_align = vertical_align;
        self
    }

    pub fn line_height(mut self, line_height: impl Into<LineHeight>) -> Self {
        self.line_height = line_height.into();
        self
    }

    fn char_style(
        &self,
    ) -> impl CharacterStyle<Color = R::Color> + TextRenderer<Color = R::Color> + '_ {
        match &self.font {
            Font::Mono(mono) => {
                MonoTextStyleBuilder::new().font(&mono).text_color(self.text_color).build()
            },
        }
    }
}

impl<'a, Message, R, E: Event, S> Widget<Message, R, E, S> for Text<'a, R>
where
    R: Renderer,
{
    fn id(&self) -> Option<crate::el::ElId> {
        None
    }

    fn tree_ids(&self) -> alloc::vec::Vec<crate::el::ElId> {
        vec![]
    }

    fn size(&self) -> Size<Length> {
        self.size.into()
    }

    fn layout(
        &self,
        _ctx: &mut UiCtx<Message>,
        _state_tree: &mut StateNode,
        _styler: &S,
        limits: &crate::layout::Limits,
    ) -> crate::layout::LayoutNode {
        Layout::sized(limits, self.size.width, self.size.height, |limits| {
            let text_size = self.font.measure_text_size(&self.content);
            limits.resolve_size(self.size.width, self.size.height, text_size)
        })
    }

    fn draw(
        &self,
        _ctx: &mut UiCtx<Message>,
        _state_tree: &mut StateNode,
        renderer: &mut R,
        _styler: &S,
        layout: Layout,
    ) {
        // renderer.text(&TextBox {
        //     position: layout.bounds().position,
        //     align: self.align,
        //     style: self.style,
        //     text: &self.content,
        // })
        renderer.text(&TextBox::with_textbox_style(
            &self.content,
            layout.bounds().into(),
            self.char_style(),
            TextBoxStyleBuilder::new()
                .alignment(self.align.into())
                .vertical_alignment(self.vertical_align.into())
                .line_height(self.line_height.into())
                .build(),
        ))
    }
}

impl<'a, Message, R, E, S> From<Text<'a, R>> for El<'a, Message, R, E, S>
where
    Message: 'a,
    R: Renderer + 'a,
    E: Event + 'a,
    S: 'a,
{
    fn from(value: Text<'a, R>) -> Self {
        El::new(value)
    }
}

impl<'a, R: Renderer> From<&'a str> for Text<'a, R> {
    fn from(value: &'a str) -> Self {
        Self::new(&value)
    }
}

impl<'a, Message, R, E, S> From<&'a str> for El<'a, Message, R, E, S>
where
    Message: 'a,
    R: Renderer + 'a,
    E: Event + 'a,
    S: 'a,
{
    fn from(value: &'a str) -> Self {
        Text::from(value).into()
    }
}

// #[derive(Clone, Copy)]
// pub struct TextStyle<C: UiColor> {
//     pub font: Font,
//     pub text_color: C,
// }

// #[derive(Clone, Copy)]
// pub struct TextBox<'a, R: Renderer> {
//     pub position: Point,
//     pub align: HorizontalAlign,
//     pub style: TextStyle<R::Color>,
//     pub text: &'a str,
// }
