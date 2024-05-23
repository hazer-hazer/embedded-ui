use embedded_graphics::geometry::Point;
use embedded_graphics::primitives::Rectangle;

use crate::color::UiColor;
use crate::el::{El, ElId};
use crate::event::Event;
use crate::layout::Layout;
use crate::size::Size;
use crate::state::StateNode;
use crate::ui::UiCtx;
use crate::widget::Widget;
use crate::{block::Border, render::Renderer};

#[derive(Clone, Copy)]
pub enum CheckboxSign {
    Check,
    Dot,
    Cross,
}

pub struct Checkbox<R: Renderer> {
    id: ElId,
    sign: CheckboxSign,
    size: u32,
    roundness: u32,
    color: R::Color,
}

impl<R: Renderer> Checkbox<R> {
    pub fn new() -> Self {
        Self {
            id: ElId::unique(),
            sign: CheckboxSign::Check,
            size: 10,
            roundness: 0,
            color: R::Color::default_foreground(),
        }
    }

    pub fn sign(mut self, sign: CheckboxSign) -> Self {
        self.sign = sign;
        self
    }

    pub fn roundness(mut self, roundness: u32) -> Self {
        self.roundness = roundness;
        self
    }

    pub fn color(mut self, color: R::Color) -> Self {
        self.color = color;
        self
    }

    pub fn size(mut self, size: u32) -> Self {
        self.size = size;
        self
    }
}

impl<Message, R: Renderer, E: Event, S> Widget<Message, R, E, S> for Checkbox<R> {
    fn id(&self) -> Option<crate::el::ElId> {
        Some(self.id)
    }

    fn tree_ids(&self) -> alloc::vec::Vec<ElId> {
        vec![self.id]
    }

    fn size(&self) -> crate::size::Size<crate::size::Length> {
        // Size::new_equal(self.size)
        Size::fixed_length(self.size, self.size)
    }

    fn layout(
        &self,
        _ctx: &mut UiCtx<Message>,
        state_tree: &mut StateNode,
        styler: &S,
        limits: &crate::layout::Limits,
    ) -> crate::layout::LayoutNode {
        Layout::sized(limits, self.size, self.size, |_| Size::zero())
    }

    fn draw(
        &self,
        _ctx: &mut UiCtx<Message>,
        state_tree: &mut StateNode,
        renderer: &mut R,
        styler: &S,
        layout: crate::layout::Layout,
    ) {
        let bounds = layout.bounds();

        // TODO: Adjust border width by size
        let border = Border { color: self.color, width: 1, radius: self.roundness.into() };

        renderer.block(&crate::block::Block {
            border,
            rect: Rectangle::new(bounds.position, bounds.size.into()),
            background: R::Color::transparent(),
        });

        let sign_pos = bounds.position + Point::new_equal(border.width as i32 + 1);
        let sign_size = bounds.size - border.width * 2 - 2;

        // TODO: Adjust stroke with of sign
        // TODO: State
        match self.sign {
            CheckboxSign::Check => todo!(),
            CheckboxSign::Dot => {
                renderer.fill_rect(Rectangle::new(sign_pos, sign_size.into()), self.color)
            },
            CheckboxSign::Cross => {
                renderer.line(
                    bounds.position,
                    bounds.position
                        + Point::new_equal(self.size.saturating_sub(border.width) as i32),
                    self.color,
                    1,
                );
                renderer.line(
                    bounds.position + Point::new(self.size as i32 - 1, 0),
                    bounds.position
                        + Point::new_equal(self.size.saturating_sub(border.width) as i32)
                        - Point::new(self.size as i32 - 1, 0),
                    self.color,
                    1,
                )
            },
        }
    }
}

impl<'a, Message, R, E, S> From<Checkbox<R>> for El<'a, Message, R, E, S>
where
    Message: 'a,
    R: Renderer + 'a,
    E: Event + 'a,
{
    fn from(value: Checkbox<R>) -> Self {
        Self::new(value)
    }
}
