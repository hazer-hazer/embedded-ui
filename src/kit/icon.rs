use embedded_graphics::geometry::Point;
use embedded_graphics::iterator::raw::RawDataSlice;
use embedded_graphics::pixelcolor::raw::{BigEndian, RawData, RawU1};

use crate::el::El;
use crate::icons::icons5::Icons5;
use crate::icons::icons6::Icons6;
use crate::icons::icons7::Icons7;
use crate::icons::icons8::Icons8;
use crate::icons::{IconData, IconKind, IconSet};
use crate::layout::{LayoutNode, Limits, Viewport};
use crate::log::logger::warning;
use crate::palette::PaletteColor;
use crate::size::Length;
use crate::style::{component_style, Styler};
use crate::theme::Theme;
use crate::{color::UiColor, event::Event, render::Renderer, size::Size, widget::Widget};

pub struct IconPicker;

impl IconPicker {
    pub fn by_size(&self, size: u32, kind: IconKind) -> Option<IconData> {
        match size {
            5 => Icons5.pick(kind),
            6 => Icons6.pick(kind),
            7 => Icons7.pick(kind),
            8 => Icons8.pick(kind),
            _ => None,
        }
    }

    pub fn flex_size(&self, length: Length, limits: &Limits) -> u32 {
        let fit_square = limits.resolve_square(length);
        match fit_square {
            size @ (5 | 6 | 7 | 8..) => size,
            _ => 0,
        }
    }

    pub fn flex(&self, length: Length, limits: &Limits, kind: IconKind) -> Option<IconData> {
        let size = self.flex_size(length, limits);
        self.by_size(size, kind)
    }
}

pub struct IconStatus;

pub fn default<C: PaletteColor>(theme: &Theme<C>, _status: IconStatus) -> IconStyle<C> {
    let palette = theme.palette();
    let base = IconStyle::new(&palette).background(palette.background).color(palette.foreground);
    base
}

component_style! {
    pub IconStyle: IconStyler(IconStatus) default {default} {
        background: background,
        color: color,
    }
}

pub struct Icon<'a, R, S>
where
    R: Renderer,
    S: IconStyler<R::Color>,
{
    size: Length,
    kind: IconKind,
    class: S::Class<'a>,
}

impl<'a, R, S> Icon<'a, R, S>
where
    R: Renderer,
    S: IconStyler<R::Color>,
{
    pub fn new(kind: IconKind) -> Self {
        // assert_eq!((size * size).div_ceil(8) as usize, data.len());

        Self { size: Length::Fill, kind, class: S::default() }
    }

    pub fn style(mut self, class: S::Class<'a>) -> Self {
        self.class = class;
        self
    }

    pub fn size(mut self, size: impl Into<Length>) -> Self {
        self.size = size.into();
        self
    }
}

impl<'a, Message, R, E, S> Widget<Message, R, E, S> for Icon<'a, R, S>
where
    R: Renderer + 'a,
    E: Event + 'a,
    S: IconStyler<R::Color> + 'a,
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
        _viewport: &Viewport,
    ) -> crate::layout::LayoutNode {
        let size = Size::new_equal(IconPicker.flex_size(self.size, limits));

        LayoutNode::new(limits.resolve_size(size.width, size.height, size))
    }

    fn draw(
        &self,
        _ctx: &mut crate::ui::UiCtx<Message>,
        _state: &mut crate::state::StateNode,
        renderer: &mut R,
        styler: &S,
        layout: crate::layout::Layout,
        _viewport: &Viewport,
    ) {
        let bounds = layout.bounds();
        let bounds_size = bounds.size.max_square();
        let icon = IconPicker.by_size(bounds_size, self.kind);

        // TODO: Warn that icon cannot be drawn because no fitted options found

        let style = styler.style(&self.class, IconStatus);

        if let Some(icon) = icon {
            let icon_size = icon.size;

            // Align icon to the center of bounds
            // TODO: This may be useless as layout is always of size of the icon
            let icon_position = bounds.position
                + Point::new(bounds.size.width as i32, bounds.size.height as i32) / 2
                - Point::new_equal(icon_size as i32) / 2;

            let bits_iter = RawDataSlice::<RawU1, BigEndian>::new(&icon.data).into_iter();

            let data_width = icon_size.max(8);

            for (index, bit) in bits_iter.enumerate() {
                if index % data_width as usize >= icon_size as usize {
                    continue;
                }

                let y = index / data_width as usize;
                let x = index % data_width as usize;

                let point = Point::new(x as i32, y as i32) + icon_position;

                let color = match bit.into_inner() {
                    0 => style.background,
                    1 => style.color,
                    _ => unreachable!(),
                };

                renderer.pixel(point, color);
            }
        } else {
            warning!(
                "Icon cannot be rendered: icon size: {:?} does not fit into box of size {:?}",
                bounds_size,
                bounds.size
            );
        }
    }
}

impl<'a, Message, R, E, S> From<Icon<'a, R, S>> for El<'a, Message, R, E, S>
where
    Message: 'a,
    R: Renderer + 'a,
    E: Event + 'a,
    S: IconStyler<R::Color> + 'a,
{
    fn from(value: Icon<'a, R, S>) -> Self {
        Self::new(value)
    }
}

impl<'a, Message, R, E, S> From<IconKind> for El<'a, Message, R, E, S>
where
    Message: 'a,
    R: Renderer + 'a,
    E: Event + 'a,
    S: IconStyler<R::Color> + 'a,
{
    fn from(value: IconKind) -> Self {
        El::new(Icon::new(value))
    }
}

impl<'a, R: Renderer, S: IconStyler<R::Color>> Into<Icon<'a, R, S>> for IconKind {
    fn into(self) -> Icon<'a, R, S> {
        Icon::new(self)
    }
}
