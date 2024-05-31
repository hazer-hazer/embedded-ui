use embedded_graphics::geometry::Point;
use embedded_graphics::primitives::Rectangle;

use crate::color::UiColor;
use crate::el::{El, ElId};
use crate::event::{Capture, CommonEvent, Event, Propagate};
use crate::icons::IconKind;
use crate::layout::Layout;
use crate::padding::Padding;
use crate::size::{Length, Size};
use crate::state::{State, StateNode, StateTag};
use crate::style::component_style;
use crate::ui::UiCtx;
use crate::widget::Widget;
use crate::{block::Border, render::Renderer};

use super::icon::Icon;

// #[derive(Clone, Copy)]
// pub enum CheckboxSign {
//     Check,
//     Dot,
//     Cross,
// }

#[derive(Clone, Copy)]
pub struct CheckboxState {
    is_pressed: bool,
    is_checked: bool,
}

impl Default for CheckboxState {
    fn default() -> Self {
        Self { is_pressed: false, is_checked: false }
    }
}

#[derive(Clone, Copy)]
pub enum CheckboxStatus {
    Normal,
    Pressed,
    Focused,

    // TODO: Think about compound statuses such as FocusedChecked, PressedChecked, etc.
    Checked,
}

component_style! {
    pub CheckboxStyle: CheckboxStyler(CheckboxStatus) {
        background: background,
        border: border,
    }
}

pub struct Checkbox<'a, Message, R, S>
where
    R: Renderer,
    S: CheckboxStyler<R::Color>,
{
    id: ElId,
    check_icon: Icon<R>,
    size: Size<Length>,
    on_change: Box<dyn Fn(bool) -> Message + 'a>,
    class: S::Class<'a>,
}

impl<'a, Message, R, S> Checkbox<'a, Message, R, S>
where
    R: Renderer,
    S: CheckboxStyler<R::Color>,
{
    pub fn new<F>(on_change: F) -> Self
    where
        F: 'a + Fn(bool) -> Message,
    {
        Self {
            id: ElId::unique(),
            check_icon: Icon::new(crate::icons::IconKind::Check),
            size: Size::fill(),
            on_change: Box::new(on_change),
            class: S::default(),
            // color: R::Color::default_foreground(),
        }
    }

    // pub fn sign(mut self, sign: CheckboxSign) -> Self {
    //     self.sign = sign;
    //     self
    // }

    pub fn icon(mut self, icon: IconKind) -> Self {
        self.check_icon = Icon::new(icon);
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.size.width = width.into();
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.size.height = height.into();
        self
    }

    // Helpers //
    fn status<E: Event>(&self, ctx: &UiCtx<Message>, state: &StateNode) -> CheckboxStatus {
        let focused = UiCtx::is_focused::<R, E, S>(&ctx, self);
        match (state.get::<CheckboxState>(), focused) {
            (CheckboxState { is_pressed: true, .. }, _) => CheckboxStatus::Pressed,
            (CheckboxState { is_checked: true, .. }, false) => CheckboxStatus::Checked,
            (CheckboxState { .. }, true) => CheckboxStatus::Focused,
            (CheckboxState { .. }, false) => CheckboxStatus::Normal,
        }
    }
}

impl<'a, Message, R, E, S> Widget<Message, R, E, S> for Checkbox<'a, Message, R, S>
where
    R: Renderer,
    E: Event,
    S: CheckboxStyler<R::Color>,
{
    fn id(&self) -> Option<crate::el::ElId> {
        Some(self.id)
    }

    fn tree_ids(&self) -> alloc::vec::Vec<ElId> {
        vec![self.id]
    }

    fn size(&self) -> crate::size::Size<crate::size::Length> {
        self.size
    }

    fn state_tag(&self) -> crate::state::StateTag {
        StateTag::of::<CheckboxState>()
    }

    fn state(&self) -> crate::state::State {
        State::new(CheckboxState::default())
    }

    fn state_children(&self) -> Vec<StateNode> {
        vec![]
    }

    fn on_event(
        &mut self,
        ctx: &mut UiCtx<Message>,
        event: E,
        state: &mut StateNode,
    ) -> crate::event::EventResponse<E> {
        let focused = UiCtx::is_focused::<R, E, S>(&ctx, self);
        let current_state = state.get::<CheckboxState>();

        if let Some(common) = event.as_common() {
            match common {
                CommonEvent::FocusMove(_) if focused => {
                    return Propagate::BubbleUp(self.id, event).into()
                },
                CommonEvent::FocusClickDown if focused => {
                    state.get_mut::<CheckboxState>().is_pressed = true;
                    return Capture::Captured.into();
                },
                CommonEvent::FocusClickUp if focused => {
                    let was_pressed = current_state.is_pressed;

                    state.get_mut::<CheckboxState>().is_pressed = false;

                    if was_pressed {
                        let new_state = !state.get::<CheckboxState>().is_checked;
                        state.get_mut::<CheckboxState>().is_checked = new_state;

                        ctx.publish((self.on_change)(new_state));

                        return Capture::Captured.into();
                    }
                },
                CommonEvent::FocusClickDown
                | CommonEvent::FocusClickUp
                | CommonEvent::FocusMove(_) => {
                    // Should we reset state on any event? Or only on common
                    state.get_mut::<CheckboxState>().is_pressed = false;
                },
            }
        }

        Propagate::Ignored.into()
    }

    fn layout(
        &self,
        ctx: &mut UiCtx<Message>,
        _state_tree: &mut StateNode,
        styler: &S,
        limits: &crate::layout::Limits,
    ) -> crate::layout::LayoutNode {
        Layout::container(
            limits,
            self.size,
            Padding::zero(),
            Padding::new_equal(1),
            crate::align::Alignment::Center,
            crate::align::Alignment::Center,
            |limits| {
                Widget::<Message, R, E, S>::layout(
                    &self.check_icon,
                    ctx,
                    &mut StateNode::stateless(),
                    styler,
                    limits,
                )
            },
        )
    }

    fn draw(
        &self,
        ctx: &mut UiCtx<Message>,
        state: &mut StateNode,
        renderer: &mut R,
        styler: &S,
        layout: crate::layout::Layout,
    ) {
        let style = styler.style(&self.class, self.status::<E>(ctx, state));
        let state = state.get::<CheckboxState>();

        let bounds = layout.bounds();

        renderer.block(crate::block::Block {
            border: style.border,
            rect: Rectangle::new(bounds.position, bounds.size.into()),
            background: style.background,
        });

        if state.is_checked {
            Widget::<Message, R, E, S>::draw(
                &self.check_icon,
                ctx,
                &mut StateNode::stateless(),
                renderer,
                styler,
                layout.children().next().unwrap(),
            )
        }
    }
}

impl<'a, Message, R, E, S> From<Checkbox<'a, Message, R, S>> for El<'a, Message, R, E, S>
where
    Message: 'a,
    R: Renderer + 'a,
    E: Event + 'a,
    S: CheckboxStyler<R::Color> + 'a,
{
    fn from(value: Checkbox<'a, Message, R, S>) -> Self {
        Self::new(value)
    }
}
