use alloc::vec::Vec;
use embedded_graphics::geometry::Point;

use crate::{
    align::{Alignment, Axis, HorizontalAlign, VerticalAlign},
    el::El,
    event::Event,
    padding::Padding,
    render::Renderer,
    size::{Bounds, Length, Size},
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

#[derive(Clone)]
pub struct LayoutNode {
    position: Position,
    bounds: Bounds,
    children: Vec<LayoutNode>,
}

impl LayoutNode {
    pub fn new(size: Size) -> Self {
        Self {
            position: Position::Relative,
            bounds: Bounds { position: Point::zero(), size },
            children: vec![],
        }
    }

    pub fn with_children(size: Size, children: impl IntoIterator<Item = LayoutNode>) -> Self {
        Self {
            position: Position::Relative,
            bounds: Bounds { position: Point::zero(), size },
            children: children.into_iter().collect(),
        }
    }

    pub fn absolute(size: Size) -> Self {
        Self {
            position: Position::Absolute,
            bounds: Bounds { position: Point::zero(), size },
            children: vec![],
        }
    }

    pub fn position(&self) -> Position {
        self.position
    }

    pub fn moved(mut self, to: impl Into<Point>) -> Self {
        self.move_mut(to);
        self
    }

    pub fn move_mut(&mut self, to: impl Into<Point>) -> &mut Self {
        self.bounds.position = to.into();
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
                self.bounds.position.x +=
                    (parent_size.width as i32 - self.bounds.size.width as i32) / 2;
            },
            Alignment::End => {
                self.bounds.position.x += parent_size.width as i32 - self.bounds.size.width as i32;
            },
        }

        match vertical {
            Alignment::Start => {},
            Alignment::Center => {
                self.bounds.position.y +=
                    (parent_size.height as i32 - self.bounds.size.height as i32) / 2;
            },
            Alignment::End => {
                self.bounds.position.y += parent_size.width as i32 - self.bounds.size.width as i32;
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
        self.bounds.size
    }
}

impl Default for LayoutNode {
    fn default() -> Self {
        Self::new(Size::zero())
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
        Self { viewport_position: node.bounds.position.into(), node }
    }

    pub fn with_offset(offset: Point, node: &'a LayoutNode) -> Self {
        let bounds = node.bounds;

        let offset = match node.position {
            Position::Relative => offset,
            Position::Absolute => Point::zero(),
        };

        Self { viewport_position: bounds.position + offset, node }
    }

    /// Get iterator of children with offset relative to parent
    pub fn children(self) -> impl DoubleEndedIterator<Item = Layout<'a>> {
        self.node
            .children
            .iter()
            .map(move |child| Layout::with_offset(self.viewport_position, child))
    }

    /// Bounds in viewport
    pub fn bounds(&self) -> Bounds {
        Bounds { position: self.viewport_position, size: self.node.bounds.size }
    }

    pub fn sized(
        limits: &Limits,
        size: impl Into<Size<Length>>,
        content_limits: impl FnOnce(&Limits) -> Size,
    ) -> LayoutNode {
        let size = size.into();

        let limits = limits.limit_width(size.width).limit_height(size.height);
        let content_size = content_limits(&limits);

        LayoutNode::new(limits.resolve_size(size.width, size.height, content_size))
    }

    pub fn container(
        limits: &Limits,
        size: impl Into<Size<Length>>,
        padding: impl Into<Padding>,
        border: impl Into<Padding>,
        content_align_h: Alignment,
        content_align_v: Alignment,
        content_layout: impl FnOnce(&Limits) -> LayoutNode,
        // place_content: impl FnOnce(LayoutNode, Size) -> LayoutNode,
    ) -> LayoutNode {
        let size = size.into();
        let padding = padding.into();
        let border = border.into();

        let full_padding = padding + border;

        let limits = limits.limit_width(size.width).limit_height(size.height);
        let content = content_layout(&limits.shrink(full_padding));
        let fit_padding = full_padding.fit(content.size(), limits.max());

        let size = limits.shrink(fit_padding).resolve_size(size.width, size.height, content.size());
        let content_offset = full_padding.top_left();

        let content = content.moved(content_offset).aligned(content_align_h, content_align_v, size);

        LayoutNode::with_children(size.expand(fit_padding), vec![content])
    }

    pub fn flex<Message, R: Renderer, E: Event, S>(
        ctx: &mut UiCtx<Message>,
        state_tree: &mut StateNode,
        styler: &S,
        axis: Axis,
        limits: &Limits,
        size: impl Into<Size<Length>>,
        padding: impl Into<Padding>,
        gap: u32,
        align: Alignment,
        children: &[El<'_, Message, R, E, S>],
    ) -> LayoutNode {
        let size = size.into();
        let padding = padding.into();

        let limits = limits.limit_width(size.width).limit_height(size.height).shrink(padding);
        let total_gap = gap * children.len().saturating_sub(1) as u32;
        let max_anti = axis.size_anti(limits.max());

        let mut layout_children = Vec::with_capacity(children.len());
        layout_children.resize(children.len(), LayoutNode::default());

        let mut total_main_divs = 0;

        let mut free_main = axis.size_main(limits.max()).saturating_sub(total_gap);
        let mut free_anti = match axis {
            Axis::X if size.width == Length::Shrink => 0,
            Axis::Y if size.height == Length::Shrink => 0,
            _ => max_anti,
        };

        // Calculate non-auto-sized children (main axis length is not Length::Fill or Length::Div)
        for ((i, child), child_state) in
            children.iter().enumerate().zip(state_tree.children.iter_mut())
        {
            let (fill_main_div, fill_anti_div) = {
                let size = child.size();
                axis.canon(size.width.div_factor(), size.height.div_factor())
            };

            if fill_main_div == 0 {
                let (max_width, max_height) =
                    axis.canon(free_main, if fill_anti_div == 0 { max_anti } else { free_anti });

                let child_limits = Limits::new(Size::zero(), Size::new(max_width, max_height));

                let layout = child.layout(ctx, child_state, styler, &child_limits);
                let size = layout.size();

                free_main -= axis.size_main(size);
                free_anti = free_anti.max(axis.size_anti(size));

                layout_children[i] = layout;
            } else {
                total_main_divs += fill_main_div as u32;
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
            let (fill_main_div, fill_anti_div) = {
                let size = child.size();

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
                let (max_width, max_height) =
                    axis.canon(max_main, if fill_anti_div == 0 { max_anti } else { free_anti });

                let child_limits =
                    Limits::new(Size::new(min_width, min_height), Size::new(max_width, max_height));

                let layout = child.layout(ctx, child_state, styler, &child_limits);
                free_anti = free_anti.max(axis.size_anti(layout.size()));
                layout_children[i] = layout;
            }
        }

        let (main_padding, anti_padding) = axis.canon(padding.left, padding.right);
        let mut main_offset = main_padding;

        for (i, node) in layout_children.iter_mut().enumerate() {
            if i > 0 {
                main_offset += gap;
            }

            let (x, y) = axis.canon(main_offset as i32, anti_padding as i32);
            node.move_mut(Point::new(x, y));

            match axis {
                Axis::X => node.align_mut(align, Alignment::Start, Size::new(0, free_anti)),
                Axis::Y => node.align_mut(Alignment::Start, align, Size::new(free_anti, 0)),
            };

            let size = node.size();

            main_offset += axis.size_main(size);
        }

        let (content_width, content_height) = axis.canon(main_offset - main_padding, free_anti);
        let size =
            limits.resolve_size(size.width, size.height, Size::new(content_width, content_height));

        LayoutNode::with_children(size.expand(padding), layout_children)
    }

    // pub fn flex<Message, R: Renderer>(
    //     axis: Axis,
    //     limits: &Limits,
    //     width: impl Into<Length>,
    //     height: impl Into<Length>,
    //     gap: u32,
    //     align: Alignment,
    //     children: &[El<'_, Message, R>],
    // ) -> LayoutNode {
    //     let width = width.into();
    //     let height = height.into();

    //     let limits = limits.limit_width(width).limit_height(height);
    //     let max_anti = axis.size_anti(limits.max());

    //     let total_gap = children.len().saturating_sub(1) as u32 * gap;

    //     let mut children_layouts = Vec::with_capacity(children.len());
    //     children_layouts.resize(children.len(), LayoutNode::default());

    //     let mut free_main = axis.size_main(limits.max()) - total_gap;
    //     let mut used_main = 0;
    //     let mut used_anti = 0;

    //     let total_main_div = children.iter().fold(0, |div, child| {
    //         div + axis.size_main(child.size()).div_factor()
    //     });

    //     for (i, child) in children.iter().enumerate() {
    //         // let child_size = child.size();
    //         let child_limits = Limits::new(Size::zero(), Size::new(free_main, max_anti));
    //         let child_layout = child.layout(&child_limits);
    //         let child_size = child_layout.size();

    //         used_anti = used_anti.max(axis.size_anti(child_layout.size()));

    //         free_main -= axis.size_main(child_size);
    //     }

    //     let (content_width, content_height) = axis.canon(used_main, used_anti);
    //     let size = limits.resolve_size(width, height, Size::new(content_width, content_height));

    //     LayoutNode::with_children(size, children_layouts)
    // }
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

impl From<Bounds> for Limits {
    fn from(value: Bounds) -> Self {
        Self::new(Size::zero(), value.size.into())
    }
}
