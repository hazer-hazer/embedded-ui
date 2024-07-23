use embedded_graphics::geometry::Point;

use crate::axis::{Axial, Axis};
use crate::color::UiColor;
use crate::el::El;
use crate::layout::{Layout, Viewport};
use crate::size::{Length, Size};
use crate::widget::Widget;
use crate::{event::Event, padding::Padding, render::Renderer};

pub struct Divider<C: UiColor> {
    axis: Axis,
    thickness: u32,
    color: C,
    padding: Padding,
}

impl<C> Divider<C>
where
    C: UiColor,
{
    pub fn new(axis: Axis) -> Self {
        Self { axis, thickness: 1, color: C::default_foreground(), padding: Padding::zero() }
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

    pub fn color(mut self, color: C) -> Self {
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

impl<Message, C, E, S> Widget<Message, C, E, S> for Divider<C>
where
    C: UiColor,
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
        renderer: &mut Renderer<C>,
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

        renderer.line(start, end, self.color, self.thickness)
    }
}

impl<'a, Message, C, E, S> From<Divider<C>> for El<'a, Message, C, E, S>
where
    Message: 'a,
    C: UiColor + 'a,
    E: Event + 'a,
    S: 'a,
{
    fn from(value: Divider<C>) -> Self {
        El::new(value)
    }
}

impl<C: UiColor> Clone for Divider<C> {
    fn clone(&self) -> Self {
        Self {
            axis: self.axis,
            thickness: self.thickness,
            color: self.color,
            padding: self.padding,
        }
    }
}
impl<C: UiColor> Copy for Divider<C> {}
