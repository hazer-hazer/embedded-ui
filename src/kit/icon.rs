use alloc::string::String;
use embedded_graphics::geometry::Point;
use embedded_graphics::iterator::raw::RawDataSlice;
use embedded_graphics::pixelcolor::raw::{BigEndian, RawData, RawU1};

use crate::el::El;
use crate::icons::icons5::Icons5;
use crate::icons::{IconData, IconKind, IconSet};
use crate::layout::{LayoutNode, Limits};
use crate::size::Length;
use crate::{
    color::UiColor, event::Event, layout::Layout, render::Renderer, size::Size, widget::Widget,
};

pub struct IconPicker;

impl IconPicker {
    pub fn by_size(&self, size: u32, kind: IconKind) -> Option<IconData> {
        match size {
            5.. => Icons5.pick(kind),
            _ => None,
        }
    }

    pub fn flex_size(&self, length: Length, limits: &Limits) -> u32 {
        let fit_square = limits.resolve_square(length);
        match fit_square {
            5.. => 5,
            _ => 0,
        }
    }

    pub fn flex(&self, length: Length, limits: &Limits, kind: IconKind) -> Option<IconData> {
        let size = self.flex_size(length, limits);
        self.by_size(size, kind)
    }
}

#[derive(Clone, Copy)]
pub struct Icon<R>
where
    R: Renderer,
{
    size: Length,
    kind: IconKind,
    color: R::Color,
    background: R::Color,
}

impl<R> Icon<R>
where
    R: Renderer,
{
    pub fn new(kind: IconKind) -> Self {
        // assert_eq!((size * size).div_ceil(8) as usize, data.len());

        Self {
            size: Length::Fill,
            kind,
            color: R::Color::default_foreground(),
            background: R::Color::default_background(),
        }
    }

    pub fn color(mut self, color: impl Into<R::Color>) -> Self {
        self.color = color.into();
        self
    }

    pub fn background(mut self, background: impl Into<R::Color>) -> Self {
        self.background = background.into();
        self
    }

    pub fn invert(mut self) -> Self {
        self.color = self.background;
        self.background = self.color;
        self
    }

    pub fn size(mut self, size: impl Into<Length>) -> Self {
        self.size = size.into();
        self
    }

    pub fn do_invert(self, invert: bool) -> Self {
        if invert {
            self.invert()
        } else {
            self
        }
    }
}

impl<Message, R, E, S> Widget<Message, R, E, S> for Icon<R>
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
        Size::new_equal(self.size)
    }

    fn layout(
        &self,
        _ctx: &mut crate::ui::UiCtx<Message>,
        _state: &mut crate::state::StateNode,
        _styler: &S,
        limits: &crate::layout::Limits,
    ) -> crate::layout::LayoutNode {
        let size = Size::new_equal(IconPicker.flex_size(self.size, limits));

        LayoutNode::new(limits.resolve_size(size.width, size.height, size))
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
        let icon = IconPicker.by_size(bounds.size.max_square(), self.kind);

        // TODO: Warn that icon cannot be drawn because no fitted options found

        if let Some(icon) = icon {
            let size = icon.size;
            let bits_iter = RawDataSlice::<RawU1, BigEndian>::new(&icon.data).into_iter();

            let data_width = size.max(8);

            for (index, bit) in bits_iter.enumerate() {
                if index % data_width as usize >= size as usize {
                    continue;
                }

                let y = index / data_width as usize;
                let x = index % data_width as usize;

                let point = Point::new(x as i32, y as i32) + bounds.position;

                let color = match bit.into_inner() {
                    0 => self.background,
                    1 => self.color,
                    _ => unreachable!(),
                };

                renderer.pixel(point, color);
            }
        }
    }
}

impl<'a, Message, R, E, S> From<Icon<R>> for El<'a, Message, R, E, S>
where
    Message: 'a,
    R: Renderer + 'a,
    E: Event + 'a,
    S: 'a,
{
    fn from(value: Icon<R>) -> Self {
        Self::new(value)
    }
}

impl<'a, Message, R, E, S> From<IconKind> for El<'a, Message, R, E, S>
where
    Message: 'a,
    R: Renderer + 'a,
    E: Event + 'a,
    S: 'a,
{
    fn from(value: IconKind) -> Self {
        El::new(Icon::new(value))
    }
}

impl<R: Renderer> Into<Icon<R>> for IconKind {
    fn into(self) -> Icon<R> {
        Icon::new(self)
    }
}
