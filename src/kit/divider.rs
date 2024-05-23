use embedded_graphics::geometry::Point;

use crate::align::Axis;
use crate::color::UiColor;
use crate::el::El;
use crate::layout::{Layout, LayoutNode};
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

impl<R> Divider<R>
where
    R: Renderer,
{
    pub fn new(axis: Axis) -> Self {
        let (padding_main, padding_anti) = axis.canon(0, 1);
        Self {
            axis,
            thickness: 1,
            color: R::Color::default_foreground(),
            padding: Padding::new_axis(padding_anti, padding_main),
        }
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
        let (padding_main, _padding_anti) =
            self.axis.canon(self.padding.total_x(), self.padding.total_y());
        let (main_axis, anti_axis) = self.axis.canon(padding_main, 5);
        self.padding = self.padding + Padding::new_axis(anti_axis, main_axis);
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

    fn size(&self) -> crate::size::Size<crate::size::Length> {
        let (main_axis, anti_axis) = self.axis.canon(
            Length::Fill,
            Length::Fixed(self.thickness + self.padding.total_axis(self.axis.invert())),
        );
        Size::new(main_axis, anti_axis)
    }

    fn layout(
        &self,
        _ctx: &mut crate::ui::UiCtx<Message>,
        _state: &mut crate::state::StateNode,
        _styler: &S,
        limits: &crate::layout::Limits,
    ) -> crate::layout::LayoutNode {
        let (main_axis, anti_axis) = self.axis.canon(
            Length::Fill,
            Length::Fixed(self.thickness + self.padding.total_axis(self.axis.invert())),
        );
        let size = Size::new(main_axis, anti_axis);

        Layout::sized(
            limits,
            size.width,
            size.height,
            // self.padding,
            // Padding::zero(),
            |limits| {
                // let (main_axis, anti_axis) =
                //     self.axis.canon(limits.max().axis(self.axis), self.thickness);
                // LayoutNode::new(Size::new(main_axis, anti_axis))
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
    ) {
        let bounds = layout.bounds();

        let (main_axis_size, anti_axis_size) =
            self.axis.canon(bounds.size.width, bounds.size.height);
        // let start = bounds.position + self.padding.top_left();
        let start = Point::new(bounds.position.x, bounds.position.y + anti_axis_size as i32 / 2);
        let (main_axis_start, anti_axis_start) = self.axis.canon(start.x, start.y);
        let end = self.axis.canon(main_axis_start + main_axis_size as i32, anti_axis_start);
        let end = Point::new(end.0, end.1);

        renderer.line(start, end, self.color, self.thickness)
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
