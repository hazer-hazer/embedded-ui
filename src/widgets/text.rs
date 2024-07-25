use alloc::{string::ToString as _, vec::Vec};
use core::fmt::Display;

use embedded_graphics::mono_font::{MonoTextStyle, MonoTextStyleBuilder};
use embedded_text::{
    style::{TextBoxStyle, TextBoxStyleBuilder},
    TextBox,
};

use crate::{
    align::VerticalAlign,
    el::{El, ElId},
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
    let base =
        TextStyle::new(&palette).background(palette.background).text_color(palette.foreground);

    base
}

component_style! {
    pub TextStyle: TextStyler(TextStatus) default {default} {
        background: background,
        text_color: color,
    }
}

#[derive(Clone, Copy)]
pub enum TextAlign {
    Left,
    Center,
    Right,
    Justified,
}

impl Into<embedded_text::alignment::HorizontalAlignment> for TextAlign {
    fn into(self) -> embedded_text::alignment::HorizontalAlignment {
        match self {
            TextAlign::Left => embedded_text::alignment::HorizontalAlignment::Left,
            TextAlign::Center => embedded_text::alignment::HorizontalAlignment::Center,
            TextAlign::Right => embedded_text::alignment::HorizontalAlignment::Right,
            TextAlign::Justified => embedded_text::alignment::HorizontalAlignment::Justified,
        }
    }
}

pub struct Text<'a, T, R, S>
where
    R: Renderer,
    T: Display,
    S: TextStyler<R::Color>,
{
    content: T,

    align: TextAlign,
    vertical_align: VerticalAlign,
    line_height: LineHeight,
    font: Font,
    size: Size<Length>,

    class: S::Class<'a>,
    // TODO: Cache size?
    // /// Precomputed size, does not need to be set by user
    // size: Size,
}

impl<'a, T, R, S> Text<'a, T, R, S>
where
    T: Display,
    R: Renderer,
    S: TextStyler<R::Color>,
{
    pub fn new(content: T) -> Self {
        Self {
            content,
            line_height: LineHeight::default(),
            align: TextAlign::Center,
            vertical_align: VerticalAlign::Center,
            size: Size::shrink(),
            font: R::default_font(),
            class: S::default(),
        }
    }

    pub fn align(mut self, align: impl Into<TextAlign>) -> Self {
        self.align = align.into();
        self
    }

    pub fn vertical_align(mut self, vertical_align: impl Into<VerticalAlign>) -> Self {
        self.vertical_align = vertical_align.into();
        self
    }

    pub fn line_height(mut self, line_height: impl Into<LineHeight>) -> Self {
        self.line_height = line_height.into();
        self
    }

    pub fn font_size(mut self, font_size: impl Into<FontSize>) -> Self {
        self.font.size = font_size.into();
        self
    }

    pub fn update(&mut self, new_value: T) {
        self.content = new_value;
    }

    // Helpers //
    fn text_style(
        &self,
        style: &TextStyle<R::Color>,
        viewport: &Viewport,
    ) -> MonoTextStyle<'_, R::Color> {
        let real_font = self.font.to_real(viewport);
        let mono_text_style = MonoTextStyleBuilder::new()
            .font(&real_font.font())
            .text_color(style.text_color)
            // .background_color(style.background)
            .build();

        mono_text_style
    }

    fn textbox_style(&self) -> TextBoxStyle {
        TextBoxStyleBuilder::new()
            .alignment(self.align.into())
            .vertical_alignment(self.vertical_align.into())
            .line_height(self.line_height.into())
            .height_mode(embedded_text::style::HeightMode::ShrinkToText(
                embedded_text::style::VerticalOverdraw::Hidden,
            ))
            .build()
    }

    // fn compute_size(&self, text: &str, style: &TextStyle<R::Color>, viewport:
    // &Viewport) -> Size {     // self.font.to_real(viewport).
    // measure_text_size(&self.content.to_string())

    // }
}

impl<'a, T, Message, R, E: Event, S> Widget<Message, R, E, S> for Text<'a, T, R, S>
where
    T: Display,
    R: Renderer,
    S: TextStyler<R::Color>,
{
    fn id(&self) -> Option<ElId> {
        None
    }

    fn tree_ids(&self) -> Vec<ElId> {
        vec![]
    }

    fn size(&self, _viewport: &Viewport) -> Size<Length> {
        // self.compute_size(viewport).into()
        self.size
    }

    fn layout(
        &self,
        _ctx: &mut UiCtx<Message>,
        _state_tree: &mut StateNode,
        styler: &S,
        limits: &crate::layout::Limits,
        viewport: &Viewport,
    ) -> crate::layout::LayoutNode {
        let style = styler.style(&self.class, TextStatus::Normal);

        Layout::sized(limits, self.size, crate::layout::Position::Relative, viewport, |limits| {
            let width = limits.max().width;
            // FIXME: Works wrong. embedded_text hides max_line_width from LineMeasurement
            // and we cannot get what is the actual text width, so it takes all the width is
            // can. This can be seen when putting text into container and setting text
            // horizontal alignment to Center.
            let text_height = self.textbox_style().measure_text_height(
                &self.text_style(&style, viewport),
                &self.content.to_string(),
                limits.max().width,
            );

            limits.resolve_size(self.size.width, self.size.height, Size::new(width, text_height))
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

        renderer.mono_text(TextBox::with_textbox_style(
            &self.content.to_string(),
            layout.bounds(),
            self.text_style(&style, viewport),
            self.textbox_style(),
        ))
    }
}

impl<'a, T, R, S> From<T> for Text<'a, T, R, S>
where
    T: Display + 'a,
    R: Renderer,
    S: TextStyler<R::Color>,
{
    fn from(value: T) -> Self {
        Text::new(value)
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
        Text::new(value).into()
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
        Text::new(value).into()
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
