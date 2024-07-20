use crate::{
    align::Alignment,
    block::{Block, BoxModel},
    el::El,
    event::Event,
    layout::Layout,
    padding::Padding,
    palette::PaletteColor,
    render::Renderer,
    size::{Length, Size},
    style::component_style,
    theme::Theme,
    widget::Widget,
};

pub struct ContainerStatus;

component_style! {
    pub ContainerStyle: ContainerStyler(ContainerStatus) default {default} {
        background: background,
        border: border,
        padding: padding,
    }
}

pub fn default<C: PaletteColor>(theme: &Theme<C>, _status: ContainerStatus) -> ContainerStyle<C> {
    let palette = theme.palette();

    ContainerStyle::new(&palette)
}

pub struct Container<'a, Message, R, E, S>
where
    R: Renderer,
    E: Event,
    S: ContainerStyler<R::Color>,
{
    content: El<'a, Message, R, E, S>,
    size: Size<Length>,
    h_align: Alignment,
    v_align: Alignment,
    class: S::Class<'a>,
}

impl<'a, Message, R, E, S> Container<'a, Message, R, E, S>
where
    R: Renderer,
    E: Event,
    S: ContainerStyler<R::Color>,
{
    pub fn new(content: impl Into<El<'a, Message, R, E, S>>) -> Self {
        Self {
            content: content.into(),
            size: Size::fill(),
            h_align: Alignment::Start,
            v_align: Alignment::Start,
            class: S::default(),
        }
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.size.width = width.into();
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.size.height = height.into();
        self
    }

    pub fn h_align(mut self, h_align: impl Into<Alignment>) -> Self {
        self.h_align = h_align.into();
        self
    }

    pub fn v_align(mut self, v_align: impl Into<Alignment>) -> Self {
        self.v_align = v_align.into();
        self
    }
}

impl<'a, Message, R, E, S> Widget<Message, R, E, S> for Container<'a, Message, R, E, S>
where
    R: Renderer,
    E: Event,
    S: ContainerStyler<R::Color>,
{
    fn id(&self) -> Option<crate::el::ElId> {
        None
    }

    fn tree_ids(&self) -> alloc::vec::Vec<crate::el::ElId> {
        vec![]
    }

    fn size(&self, _viewport: &crate::layout::Viewport) -> Size<Length> {
        self.size
    }

    fn layout(
        &self,
        ctx: &mut crate::ui::UiCtx<Message>,
        state: &mut crate::state::StateNode,
        styler: &S,
        limits: &crate::layout::Limits,
        viewport: &crate::layout::Viewport,
    ) -> crate::layout::LayoutNode {
        let style = styler.style(&self.class, ContainerStatus);

        Layout::container(
            limits,
            self.size,
            crate::layout::Position::Relative,
            viewport,
            BoxModel::new().padding(style.padding).border(style.border),
            self.h_align,
            self.v_align,
            |limits| self.content.layout(ctx, state, styler, limits, viewport),
        )
    }

    fn draw(
        &self,
        ctx: &mut crate::ui::UiCtx<Message>,
        state: &mut crate::state::StateNode,
        renderer: &mut R,
        styler: &S,
        layout: crate::layout::Layout,
        viewport: &crate::layout::Viewport,
    ) {
        let bounds = layout.bounds();
        let style = styler.style(&self.class, ContainerStatus);

        renderer.block(Block {
            border: style.border,
            rect: bounds.into(),
            background: style.background,
        });

        self.content.draw(ctx, state, renderer, styler, layout, viewport);
    }
}

impl<'a, Message, R, E, S> From<Container<'a, Message, R, E, S>> for El<'a, Message, R, E, S>
where
    Message: Clone + 'a,
    R: Renderer + 'a,
    E: Event + 'a,
    S: ContainerStyler<R::Color> + 'a,
{
    fn from(value: Container<'a, Message, R, E, S>) -> Self {
        Self::new(value)
    }
}