// TODO

use embedded_graphics::primitives::Rectangle;

use crate::{
    axis::{Axial, Axis},
    block::{Block, Border},
    el::El,
    event::Event,
    layout::{Layout, Viewport},
    palette::PaletteColor,
    render::Renderer,
    size::{Length, Size},
    style::component_style,
    theme::Theme,
    widget::Widget,
};

#[derive(Clone, Copy)]
pub struct BarStatus {
    /// You can style bar depending on its value
    pub value: f32,
}

pub fn primary<C: PaletteColor>(theme: &Theme<C>, _status: BarStatus) -> BarStyle<C> {
    let palette = theme.palette();
    let base = BarStyle::new(&palette)
        .background(palette.background)
        .color(palette.primary)
        .border_radius(5)
        .border_width(1)
        .border_color(palette.foreground);

    base
}

component_style! {
    pub BarStyle: BarStyler(BarStatus) default {primary} {
        background: background,
        color: color,
        border: border,
    }
}

pub struct Bar<'a, R, S>
where
    R: Renderer,
    S: BarStyler<R::Color>,
{
    size: Size<Length>,
    class: S::Class<'a>,
    value: f32,
    axis: Axis,
}

impl<'a, Message, R, E, S> From<Bar<'a, R, S>> for El<'a, Message, R, E, S>
where
    R: Renderer + 'a,
    E: Event + 'a,
    S: BarStyler<R::Color> + 'a,
{
    fn from(value: Bar<'a, R, S>) -> Self {
        Self::new(value)
    }
}

impl<'a, R, S> Bar<'a, R, S>
where
    R: Renderer,
    S: BarStyler<R::Color>,
{
    pub fn new(axis: Axis) -> Self {
        Self {
            size: axis.canon(Length::Fill, Length::Fixed(10)),
            class: S::default(),
            value: 0.5,
            axis,
        }
    }

    pub fn horizontal() -> Self {
        Self::new(Axis::X)
    }

    pub fn vertical() -> Self {
        Self::new(Axis::Y)
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.size.width = width.into();
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.size.height = height.into();
        self
    }

    pub fn value(mut self, value: f32) -> Self {
        self.value = value;
        self
    }
}

impl<'a, Message, R, E, S> Widget<Message, R, E, S> for Bar<'a, R, S>
where
    R: Renderer,
    E: Event,
    S: BarStyler<R::Color>,
{
    fn id(&self) -> Option<crate::el::ElId> {
        None
    }

    fn tree_ids(&self) -> alloc::vec::Vec<crate::el::ElId> {
        vec![]
    }

    fn size(&self, _viewport: &Viewport) -> Size<Length> {
        self.size
    }

    // fn state_tag(&self) -> crate::state::StateTag {
    //     StateTag::of::<BarState>()
    // }

    // fn state(&self) -> crate::state::State {
    //     State::new(BarState::default())
    // }

    // fn state_children(&self) -> alloc::vec::Vec<crate::state::StateNode> {
    //     vec![]
    // }

    fn layout(
        &self,
        _ctx: &mut crate::ui::UiCtx<Message>,
        _state: &mut crate::state::StateNode,
        _styler: &S,
        limits: &crate::layout::Limits,
        viewport: &crate::layout::Viewport,
    ) -> crate::layout::LayoutNode {
        Layout::sized(limits, self.size, crate::layout::Position::Relative, viewport, |limits| {
            limits.resolve_size(self.size.width, self.size.height, Size::zero())
        })
    }

    fn draw(
        &self,
        _ctx: &mut crate::ui::UiCtx<Message>,
        _state: &mut crate::state::StateNode,
        renderer: &mut R,
        styler: &S,
        layout: crate::layout::Layout,
        _viewport: &crate::layout::Viewport,
    ) {
        let bounds = layout.bounds();

        let style = styler.style(&self.class, BarStatus { value: self.value });

        let size = bounds.size.into_axial(self.axis);
        let length = (self.value * size.main() as f32) as u32;
        let bar_rect = Into::<Rectangle>::into(bounds);
        let bar_rect = match self.axis {
            Axis::X => bar_rect.resized_width(length, embedded_graphics::geometry::AnchorX::Left),
            Axis::Y => {
                bar_rect.resized_height(length, embedded_graphics::geometry::AnchorY::Bottom)
            },
        };
        let bar = Block {
            border: Border::zero().radius(style.border.radius),
            rect: bar_rect,
            background: Some(style.color),
        };

        renderer.block(style.border.into_block(bounds, style.background));
        renderer.block(bar);
    }
}
