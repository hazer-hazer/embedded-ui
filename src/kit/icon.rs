use alloc::string::String;
use embedded_graphics::geometry::Point;
use embedded_graphics::iterator::raw::RawDataSlice;
use embedded_graphics::pixelcolor::raw::{BigEndian, RawData, RawU1};

use crate::el::El;
use crate::icons::IntoIcon;
use crate::{
    color::UiColor, event::Event, layout::Layout, render::Renderer, size::Size, widget::Widget,
};

pub struct Icon<'a, R>
where
    R: Renderer,
{
    size: Size,
    data: &'a [u8],
    color: R::Color,
    background: R::Color,
}

impl<'a, R> Icon<'a, R>
where
    R: Renderer,
{
    pub fn new(size: Size, data: &'a [u8]) -> Self {
        assert_eq!(size.height as usize, data.len());

        Self {
            size,
            data,
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

    pub fn do_invert(self, invert: bool) -> Self {
        if invert {
            self.invert()
        } else {
            self
        }
    }
}

impl<'a, Message, R, E, S> Widget<Message, R, E, S> for Icon<'a, R>
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
        self.size.as_fixed_length()
    }

    fn layout(
        &self,
        _ctx: &mut crate::ui::UiCtx<Message>,
        _state: &mut crate::state::StateNode,
        _styler: &S,
        limits: &crate::layout::Limits,
    ) -> crate::layout::LayoutNode {
        let size = self.size.as_fixed_length();

        Layout::sized(limits, size.width, size.height, |limits| {
            limits.resolve_size(size.width, size.height, Size::zero())
        })
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

        let bits_iter = RawDataSlice::<RawU1, BigEndian>::new(&self.data).into_iter();
        // let skip_bits = 8usize.saturating_sub(self.size.width as usize);

        // let image =
        //     bits_iter.map(|b| if b.into_inner() == 1 { "#" } else { "_" }).collect::<String>();

        for (index, bit) in bits_iter.enumerate() {
            if index % 8 >= self.size.width as usize {
                continue;
            }

            let y = index / self.size.height as usize;
            let x = index % self.size.width as usize;

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

impl<'a, Message, R, E, S> From<Icon<'a, R>> for El<'a, Message, R, E, S>
where
    Message: 'a,
    R: Renderer + 'a,
    E: Event + 'a,
    S: 'a,
{
    fn from(value: Icon<'a, R>) -> Self {
        Self::new(value)
    }
}
