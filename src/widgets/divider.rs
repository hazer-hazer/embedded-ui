use embedded_graphics::geometry::Point;
use embedded_graphics::primitives::{Line, Primitive, PrimitiveStyleBuilder};

use crate::axis::{Axial, Axis};
use crate::color::UiColor;
use crate::el::El;
use crate::layout::{Layout, Viewport};
use crate::size::{Length, Size};
use crate::widget::Widget;
use crate::{event::Event, padding::Padding, render::Renderer};

pub struct Divider<R>
where
    R: Renderer,
{
    axis: Axis,
    thickness: u32,
    color: R::Color,
    padding: Padding,
}

// TODO: Styler
impl<R> Divider<R>
where
    R: Renderer,
{
    pub fn new(axis: Axis) -> Self {
        Self { axis, thickness: 1, color: R::Color::default_foreground(), padding: Padding::zero() }
    }

    pub fn vertical() -> Self {
        Self::new(Axis::Y)
    }

    pub fn horizontal() -> Self {
        Self::new(Axis::X)
    }

    pub fn thickness(mut self, thickness: u32) -> Self {
        self.thickness = thickness;
        self
    }

    pub fn color(mut self, color: R::Color) -> Self {
        self.color = color;
        self
    }

    /// Add a little padding on sides
    pub fn inset(mut self) -> Self {
        self.padding = self.padding + self.axis.canon(0, 5);
        self
    }

    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }
}

impl<Message, R, E, S> Widget<Message, R, E, S> for Divider<R>
where
    R: Renderer,
    E: Event,
{
    fn id(&self) -> Option<crate::el::ElId> {
        None
    }

    fn tree_ids(&self) -> alloc::vec::Vec<crate::el::ElId> {
        vec![]
    }

    fn size(&self, _viewport: &Viewport) -> crate::size::Size<crate::size::Length> {
        self.axis.canon(
            Length::Fill,
            Length::Fixed(self.thickness + self.padding.total_axis(self.axis.invert())),
        )
    }

    fn layout(
        &self,
        _ctx: &mut crate::ui::UiCtx<Message>,
        _state: &mut crate::state::StateNode,
        _styler: &S,
        limits: &crate::layout::Limits,
        viewport: &Viewport,
    ) -> crate::layout::LayoutNode {
        let size = self.axis.canon::<Size<Length>>(
            Length::Fill,
            Length::Fixed(self.thickness + self.padding.total_axis(self.axis.invert())),
        );

        Layout::sized(
            limits,
            size,
            crate::layout::Position::Relative,
            viewport,
            // self.padding,
            // Padding::zero(),
            |limits| {
                // let (main_axis, cross_axis) =
                //     self.axis.canon(limits.max().axis(self.axis), self.thickness);
                // LayoutNode::new(Size::new(main_axis, cross_axis))
                // LayoutNode::new(limits.max())
                limits.resolve_size(size.width, size.height, Size::zero())
            },
        )
    }

    fn draw(
        &self,
        _ctx: &mut crate::ui::UiCtx<Message>,
        _state: &mut crate::state::StateNode,
        renderer: &mut R,
        _styler: &S,
        layout: crate::layout::Layout,
        _viewport: &Viewport,
    ) {
        let bounds = layout.bounds();

        let size = bounds.size.into_axial(self.axis);
        let position = bounds.top_left.into_axial(self.axis);
        // let start = bounds.position + self.padding.top_left();
        let start =
            self.axis.canon::<Point>(position.main(), position.cross() + (size.cross() / 2) as i32);

        let end = self
            .axis
            .canon(start.main_for(self.axis) + size.main() as i32, start.cross_for(self.axis));

        renderer.line(
            Line::new(start, end).into_styled(
                PrimitiveStyleBuilder::new()
                    .stroke_color(self.color)
                    .stroke_width(self.thickness)
                    .build(),
            ),
        );
    }
}

impl<'a, Message, R, E, S> From<Divider<R>> for El<'a, Message, R, E, S>
where
    Message: 'a,
    R: Renderer + 'a,
    E: Event + 'a,
    S: 'a,
{
    fn from(value: Divider<R>) -> Self {
        El::new(value)
    }
}

impl<R: Renderer> Clone for Divider<R> {
    fn clone(&self) -> Self {
        Self {
            axis: self.axis,
            thickness: self.thickness,
            color: self.color,
            padding: self.padding,
        }
    }
}
impl<R: Renderer> Copy for Divider<R> {}
