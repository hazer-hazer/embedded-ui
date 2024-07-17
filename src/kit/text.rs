use alloc::string::ToString as _;
use core::{fmt::Display, marker::PhantomData};

use embedded_graphics::mono_font::MonoTextStyleBuilder;
use embedded_text::{style::TextBoxStyleBuilder, TextBox};

use crate::{
    align::{HorizontalAlign, VerticalAlign},
    color::UiColor,
    el::El,
    event::Event,
    font::{Font, FontSize},
    layout::{Layout, Viewport},
    palette::PaletteColor,
    render::Renderer,
    size::{Length, Size},
    state::StateNode,
    style::component_style,
    theme::Theme,
    ui::UiCtx,
    value::Value,
    widget::Widget,
};

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

#[derive(Clone, Copy)]
pub enum TextStatus {
    Normal,
}

pub fn default<C: PaletteColor>(theme: &Theme<C>, _status: TextStatus) -> TextStyle<C> {
    let palette = theme.palette();
    TextStyle { background: palette.background, text_color: palette.foreground }
}

component_style! {
    pub TextStyle: TextStyler(TextStatus) default {default} {
        background: background,
        text_color: color,
    }
}

pub struct Text<'a, T, R, S>
where
    R: Renderer,
    T: Display,
    S: TextStyler<R::Color>,
{
    content: Value<T>,
    marker: PhantomData<&'a str>,

    align: HorizontalAlign,
    vertical_align: VerticalAlign,
    line_height: LineHeight,
    font: Font,

    class: S::Class<'a>,

    font_size: FontSize,

    /// Precomputed size, does not need to be set by user
    size: Size<Length>,
}

impl<'a, T, R, S> Text<'a, T, R, S>
where
    T: Display,
    R: Renderer,
    S: TextStyler<R::Color>,
{
    pub fn new(content: Value<T>) -> Self {
        Self {
            content,
            marker: PhantomData,
            line_height: LineHeight::default(),
            align: HorizontalAlign::Center,
            vertical_align: VerticalAlign::Center,
            font: R::default_font(),
            class: S::default(),
            font_size: FontSize::Relative(1.0),
            // TODO: Should be computed by text size?
            size: Size::fill(),
        }
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

    pub fn font_size(mut self, font_size: impl Into<FontSize>) -> Self {
        self.font_size = font_size.into();
        self
    }

    pub fn update(&mut self, new_value: T) {
        *self.content.get_mut() = new_value;
    }
}

impl<'a, T, Message, R, E: Event, S> Widget<Message, R, E, S> for Text<'a, T, R, S>
where
    T: Display,
    R: Renderer,
    S: TextStyler<R::Color>,
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
        viewport: &Viewport,
    ) -> crate::layout::LayoutNode {
        Layout::sized(limits, self.size, crate::layout::Position::Relative, viewport, |limits| {
            let text_size =
                self.font.to_real(viewport).measure_text_size(&self.content.get().to_string());
            limits.resolve_size(self.size.width, self.size.height, text_size)
        })
    }

    fn draw(
        &self,
        _ctx: &mut UiCtx<Message>,
        _state_tree: &mut StateNode,
        renderer: &mut R,
        styler: &S,
        layout: Layout,
        viewport: &Viewport,
    ) {
        let style = styler.style(&self.class, TextStatus::Normal);

        let real_font = self.font.to_real(viewport);
        let mono_text_style = MonoTextStyleBuilder::new()
            .font(&real_font.font())
            .text_color(style.text_color)
            // .background_color(style.background)
            .build();

        renderer.mono_text(TextBox::with_textbox_style(
            &self.content.get().to_string(),
            layout.bounds().into(),
            mono_text_style,
            TextBoxStyleBuilder::new()
                .alignment(self.align.into())
                .vertical_alignment(self.vertical_align.into())
                .line_height(self.line_height.into())
                .build(),
        ))
    }
}

impl<'a, T, R, S> From<Value<T>> for Text<'a, T, R, S>
where
    T: Display + 'a,
    R: Renderer,
    S: TextStyler<R::Color>,
{
    fn from(value: Value<T>) -> Self {
        Text::new(value)
    }
}

// impl<'a, R: Renderer> From<&'a str> for Text<'a, &'a str, R> {
//     fn from(value: &'a str) -> Self {
//         Self::new(Value::new(value))
//     }
// }

impl<'a, T, R: Renderer, S> From<T> for Text<'a, T, R, S>
where
    T: Display + 'a,
    R: Renderer,
    S: TextStyler<R::Color>,
{
    fn from(value: T) -> Self {
        Text::new(Value::new(value))
    }
}

impl<'a, Message, R, E, S> From<&'a str> for El<'a, Message, R, E, S>
where
    Message: 'a,
    R: Renderer + 'a,
    E: Event + 'a,
    S: TextStyler<R::Color> + 'a,
{
    fn from(value: &'a str) -> Self {
        Text::new(Value::new(value)).into()
    }
}

impl<'a, Message, R, E, S> From<alloc::string::String> for El<'a, Message, R, E, S>
where
    Message: 'a,
    R: Renderer + 'a,
    E: Event + 'a,
    S: TextStyler<R::Color> + 'a,
{
    fn from(value: alloc::string::String) -> Self {
        Text::new(Value::new(value)).into()
    }
}

impl<'a, T, Message, R, E, S> From<Text<'a, T, R, S>> for El<'a, Message, R, E, S>
where
    T: Display + 'a,
    Message: 'a,
    R: Renderer + 'a,
    E: Event + 'a,
    S: TextStyler<R::Color> + 'a,
{
    fn from(value: Text<'a, T, R, S>) -> Self {
        Self::new(value)
    }
}

// impl<'a, T, Message, R, E, S> From<T> for El<'a, Message, R, E, S>
// where
//     Message: 'a,
//     R: Renderer + 'a,
//     E: Event + 'a,
//     S: 'a,
//     T: ToString,
// {
//     fn from(value: T) -> Self {
//         Text::from(value.to_string().as_str()).into()
//     }
// }

// impl<'a, F, T, Message, R, E, S> From<F> for El<'a, Message, R, E, S>
// where
//     Message: 'a,
//     R: Renderer + 'a,
//     E: Event + 'a,
//     S: 'a,
//     F: FnMut() -> T,
//     T: for<'b> Into<Text<'b, R>>,
// {
//     fn from(mut value: F) -> Self {
//         Text::from((value)().into()).into()
//     }
// }

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
