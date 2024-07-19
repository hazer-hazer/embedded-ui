use alloc::vec::Vec;
use embedded_graphics::{geometry::Point, primitives::Rectangle};

use crate::{
    align::Alignment,
    axis::{Axial, Axis},
    block::BoxModel,
    el::El,
    event::Event,
    padding::Padding,
    render::Renderer,
    size::{Length, Size},
    state::StateNode,
    ui::UiCtx,
    widget::Widget,
};

/// Positioning strategy, don't confuse with logic of CSS position.
/// For now, [`Position::Relative`] means "relative to the parent".
/// [`Position::Absolute`] is relative to viewport.
#[derive(Clone, Copy)]
pub enum Position {
    Relative,
    Absolute,
}

#[derive(Clone, Copy)]
pub struct Viewport {
    pub size: Size,
}

// #[derive(Clone, Copy)]
// pub struct Margin {

// }

#[derive(Clone)]
pub struct LayoutNode {
    position: Position,
    bounds: Rectangle,
    content: Size,
    children: Vec<LayoutNode>,
}

impl LayoutNode {
    pub fn childless(size: Size) -> Self {
        Self {
            position: Position::Relative,
            bounds: Rectangle { top_left: Point::zero(), size: size.into() },
            content: size,
            children: vec![],
        }
    }

    pub fn with_children(
        size: Size,
        margin: Padding,
        children: impl IntoIterator<Item = LayoutNode>,
    ) -> Self {
        Self {
            position: Position::Relative,
            bounds: Rectangle { top_left: Point::zero(), size: size.into() },
            content: size - margin,
            children: children.into_iter().collect(),
        }
    }

    // pub fn absolute(size: Size) -> Self {
    //     Self {
    //         position: Position::Absolute,
    //         bounds: Rectangle { top_left: Point::zero(), size: size.into() },
    //         children: vec![],
    //     }
    // }

    pub fn position(&self) -> Position {
        self.position
    }

    pub fn moved(mut self, to: impl Into<Point>) -> Self {
        self.move_mut(to);
        self
    }

    pub fn move_mut(&mut self, to: impl Into<Point>) -> &mut Self {
        self.bounds.top_left = to.into();
        self
    }

    pub fn align_mut(
        &mut self,
        horizontal: Alignment,
        vertical: Alignment,
        parent_size: Size,
    ) -> &mut Self {
        match horizontal {
            Alignment::Start => {},
            Alignment::Center => {
                self.bounds.top_left.x +=
                    (parent_size.width as i32 - self.bounds.size.width as i32) / 2;
            },
            Alignment::End => {
                self.bounds.top_left.x += parent_size.width as i32 - self.bounds.size.width as i32;
            },
        }

        match vertical {
            Alignment::Start => {},
            Alignment::Center => {
                self.bounds.top_left.y +=
                    (parent_size.height as i32 - self.bounds.size.height as i32) / 2;
            },
            Alignment::End => {
                self.bounds.top_left.y += parent_size.width as i32 - self.bounds.size.width as i32;
            },
        }

        self
    }

    pub fn aligned(
        mut self,
        horizontal: Alignment,
        vertical: Alignment,
        parent_size: Size,
    ) -> Self {
        self.align_mut(horizontal, vertical, parent_size);
        self
    }

    pub fn size(&self) -> Size {
        self.bounds.size.into()
    }
}

impl Default for LayoutNode {
    fn default() -> Self {
        Self::childless(Size::zero())
    }
}

#[derive(Clone)]
pub struct Layout<'a> {
    /// Position in viewport (display)
    viewport_position: Point,
    node: &'a LayoutNode,
}

impl<'a> Layout<'a> {
    pub fn new(node: &'a LayoutNode) -> Self {
        Self { viewport_position: node.bounds.top_left.into(), node }
    }

    pub fn with_offset(offset: Point, node: &'a LayoutNode) -> Self {
        let bounds = node.bounds;

        let offset = match node.position {
            Position::Relative => offset,
            Position::Absolute => Point::zero(),
        };

        Self { viewport_position: bounds.top_left + offset, node }
    }

    /// Get iterator of children with offset relative to parent
    pub fn children(self) -> impl DoubleEndedIterator<Item = Layout<'a>> {
        self.node
            .children
            .iter()
            .map(move |child| Layout::with_offset(self.viewport_position, child))
    }

    /// Bounds in viewport
    #[inline]
    pub fn bounds(&self) -> Rectangle {
        Rectangle { top_left: self.viewport_position, size: self.node.bounds.size }
    }

    pub fn sized(
        limits: &Limits,
        size: impl Into<Size<Length>>,
        position: Position,
        viewport: &Viewport,
        content_limits: impl FnOnce(&Limits) -> Size,
    ) -> LayoutNode {
        let size = size.into();

        let limits = limits
            .for_position(position, viewport)
            .limit_width(size.width)
            .limit_height(size.height);
        let content_size = content_limits(&limits);

        LayoutNode::childless(limits.resolve_size(size.width, size.height, content_size))
    }

    pub fn container(
        limits: &Limits,
        size: impl Into<Size<Length>>,
        position: Position,
        viewport: &Viewport,
        box_model: BoxModel,
        content_align_h: Alignment,
        content_align_v: Alignment,
        content_layout: impl FnOnce(&Limits) -> LayoutNode,
        // place_content: impl FnOnce(LayoutNode, Size) -> LayoutNode,
    ) -> LayoutNode {
        let size = size.into();
        let padding = box_model.padding;
        let border = box_model.border;

        let full_padding = padding + border;

        let limits = limits
            .for_position(position, viewport)
            .limit_width(size.width)
            .limit_height(size.height);
        let content = content_layout(&limits.shrink(full_padding));
        let fit_padding = full_padding.fit(content.size(), limits.max());

        let size = limits.shrink(fit_padding).resolve_size(size.width, size.height, content.size());
        let content_offset = full_padding.top_left();

        let content = content.moved(content_offset).aligned(content_align_h, content_align_v, size);

        LayoutNode::with_children(size.expand(fit_padding), box_model.margin, vec![content])
    }

    pub fn flex<Message, R: Renderer, E: Event, S>(
        ctx: &mut UiCtx<Message>,
        state_tree: &mut StateNode,
        styler: &S,
        axis: Axis,
        limits: &Limits,
        size: impl Into<Size<Length>>,
        position: Position,
        viewport: &Viewport,
        box_model: BoxModel,
        gap: u32,
        align: Alignment,
        children: &[El<'_, Message, R, E, S>],
    ) -> LayoutNode {
        let size = size.into();
        let padding = box_model.padding;

        let limits = limits
            .for_position(position, viewport)
            .limit_width(size.width)
            .limit_height(size.height)
            .shrink(padding);
        let total_gap = gap * children.len().saturating_sub(1) as u32;
        let max_cross = limits.max().cross_for(axis);

        let mut layout_children = Vec::with_capacity(children.len());
        layout_children.resize(children.len(), LayoutNode::default());

        let mut total_main_divs = 0;

        let mut free_main = limits.max().main_for(axis).saturating_sub(total_gap);
        let mut free_cross = match axis {
            Axis::X if size.width == Length::Shrink => 0,
            Axis::Y if size.height == Length::Shrink => 0,
            _ => max_cross,
        };

        // Calculate non-auto-sized children (main axis length is not Length::Fill or
        // Length::Div)
        for ((i, child), child_state) in
            children.iter().enumerate().zip(state_tree.children.iter_mut())
        {
            match child.position() {
                Position::Absolute => {
                    layout_children[i] = child.layout(ctx, child_state, styler, &limits, viewport);
                },
                Position::Relative => {
                    let (fill_main_div, fill_cross_div) = {
                        let size = child.size(viewport);
                        axis.canon(size.width.div_factor(), size.height.div_factor())
                    };

                    if fill_main_div == 0 {
                        let (max_width, max_height) = axis.canon(
                            free_main,
                            if fill_cross_div == 0 { max_cross } else { free_cross },
                        );

                        let child_limits =
                            Limits::new(Size::zero(), Size::new(max_width, max_height));

                        let layout =
                            child.layout(ctx, child_state, styler, &child_limits, viewport);
                        let size = layout.size();

                        free_main -= size.main_for(axis);
                        free_cross = free_cross.max(size.cross_for(axis));

                        layout_children[i] = layout;
                    } else {
                        total_main_divs += fill_main_div as u32;
                    }
                },
            }
        }

        // Remaining main axis length after calculating sizes of non-auto-sized children
        let remaining = match axis {
            Axis::X => match size.width {
                Length::Shrink => 0,
                _ => free_main.max(0),
            },
            Axis::Y => match size.height {
                Length::Shrink => 0,
                _ => free_main.max(0),
            },
        };
        let remaining_div = remaining.checked_div(total_main_divs).unwrap_or(0);
        let mut remaining_mod = remaining.checked_rem(total_main_divs).unwrap_or(0);

        // Calculate auto-sized children (Length::Fill, Length::Div(N))
        for ((i, child), child_state) in
            children.iter().enumerate().zip(state_tree.children.iter_mut())
        {
            if let Position::Relative = child.position() {
                let (fill_main_div, fill_cross_div) = {
                    let size = child.size(viewport);

                    axis.canon(size.width.div_factor(), size.height.div_factor())
                };

                if fill_main_div != 0 {
                    let max_main = if total_main_divs == 0 {
                        remaining
                    } else {
                        remaining_div * fill_main_div as u32
                            + if remaining_mod > 0 {
                                remaining_mod -= 1;
                                1
                            } else {
                                0
                            }
                    };
                    let min_main = 0;

                    let (min_width, min_height) = axis.canon(min_main, 0);
                    let (max_width, max_height) = axis
                        .canon(max_main, if fill_cross_div == 0 { max_cross } else { free_cross });

                    let child_limits = Limits::new(
                        Size::new(min_width, min_height),
                        Size::new(max_width, max_height),
                    );

                    let layout = child.layout(ctx, child_state, styler, &child_limits, viewport);
                    free_cross = free_cross.max(layout.size().cross_for(axis));
                    layout_children[i] = layout;
                }
            }
        }

        let (main_padding, cross_padding) = axis.canon(padding.left, padding.right);
        let mut main_offset = main_padding;

        for (i, node) in layout_children.iter_mut().enumerate() {
            if let Position::Relative = node.position() {
                if i > 0 {
                    main_offset += gap;
                }

                let (x, y) = axis.canon(main_offset as i32, cross_padding as i32);
                node.move_mut(Point::new(x, y));

                match axis {
                    Axis::X => node.align_mut(align, Alignment::Start, Size::new(0, free_cross)),
                    Axis::Y => node.align_mut(Alignment::Start, align, Size::new(free_cross, 0)),
                };

                let size = node.size();

                main_offset += size.main_for(axis);
            }
        }

        let (content_width, content_height) = axis.canon(main_offset - main_padding, free_cross);
        let size =
            limits.resolve_size(size.width, size.height, Size::new(content_width, content_height));

        LayoutNode::with_children(size.expand(padding), box_model.margin, layout_children)
    }
}

#[derive(Clone, Copy)]
pub struct Limits {
    min: Size<u32>,
    max: Size<u32>,
}

impl Limits {
    pub fn new(min: Size<u32>, max: Size<u32>) -> Self {
        Self { min, max }
    }

    pub fn only_max(max: Size<u32>) -> Self {
        Self { min: Size::zero(), max }
    }

    pub fn min(&self) -> Size<u32> {
        self.min
    }

    pub fn max(&self) -> Size<u32> {
        self.max
    }

    pub fn min_square(&self) -> u32 {
        self.min().width.min(self.min().height)
    }

    pub fn max_square(&self) -> u32 {
        self.max().width.min(self.max().height)
    }

    pub fn for_position(&self, position: Position, viewport: &Viewport) -> Self {
        match position {
            Position::Relative => *self,
            // TODO: Review this, may be only_max(viewport.size)
            Position::Absolute => Limits::new(self.min, viewport.size),
        }
    }

    pub fn limit_width(self, width: impl Into<Length>) -> Self {
        match width.into() {
            Length::Shrink | Length::Div(_) | Length::Fill => self,
            Length::Fixed(fixed) => {
                let new_width = fixed.min(self.max.width).max(self.min.width);

                Self::new(self.min.new_width(new_width), self.max.new_width(new_width))
            },
        }
    }

    pub fn limit_height(self, height: impl Into<Length>) -> Self {
        match height.into() {
            Length::Shrink | Length::Div(_) | Length::Fill => self,
            Length::Fixed(fixed) => {
                let new_height = fixed.min(self.max.height).max(self.min.height);

                Self::new(self.min.new_height(new_height), self.max.new_height(new_height))
            },
        }
    }

    pub fn limit_axis(self, axis: Axis, length: impl Into<Length>) -> Self {
        match axis {
            Axis::X => self.limit_width(length),
            Axis::Y => self.limit_height(length),
        }
    }

    pub fn shrink(self, by: impl Into<Size>) -> Self {
        let by = by.into();

        Limits::new(self.min() - by, self.max() - by)
    }

    pub fn resolve_size(
        &self,
        width: impl Into<Length>,
        height: impl Into<Length>,
        content_size: Size<u32>,
    ) -> Size<u32> {
        let width = match width.into() {
            Length::Fill | Length::Div(_) => self.max.width,
            Length::Fixed(fixed) => fixed.min(self.max.width).max(self.min.width),
            Length::Shrink => content_size.width.min(self.max.width).max(self.min.width),
        };

        let height = match height.into() {
            Length::Fill | Length::Div(_) => self.max.height,
            Length::Fixed(fixed) => fixed.min(self.max.height).max(self.min.height),
            Length::Shrink => content_size.height.min(self.max.height).max(self.min.height),
        };

        Size::new(width, height)
    }

    pub fn resolve_square(&self, size: impl Into<Length>) -> u32 {
        let min_square = self.min_square();
        let max_square = self.max_square();

        match size.into() {
            Length::Fill | Length::Div(_) => max_square,
            Length::Fixed(fixed) => fixed.min(max_square).max(min_square),
            Length::Shrink => min_square,
        }
    }
}

impl From<Rectangle> for Limits {
    fn from(value: Rectangle) -> Self {
        Self::new(Size::zero(), value.size.into())
    }
}
