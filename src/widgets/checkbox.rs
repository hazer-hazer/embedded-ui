use alloc::boxed::Box;
use alloc::vec::Vec;
use embedded_graphics::primitives::Rectangle;

use crate::block::BoxModel;
use crate::el::{El, ElId};
use crate::event::{Capture, CommonEvent, Event, Propagate};
use crate::font::FontSize;
use crate::icons::IconKind;
use crate::layout::{Layout, Viewport};
use crate::palette::PaletteColor;
use crate::render::Renderer;
use crate::size::{Length, Size};
use crate::state::{State, StateNode, StateTag};
use crate::style::component_style;
use crate::theme::Theme;
use crate::ui::UiCtx;
use crate::widget::Widget;

use super::icon::{Icon, IconStyler};

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
    pub CheckboxStyle: CheckboxStyler(CheckboxStatus) default {primary} {
        background: background,
        border: border,
    }
}

pub fn primary<C: PaletteColor>(theme: &Theme<C>, status: CheckboxStatus) -> CheckboxStyle<C> {
    let palette = theme.palette();
    let base = CheckboxStyle::new(&palette)
        .background(palette.background)
        .border_color(palette.foreground);

    match status {
        CheckboxStatus::Normal => base.border_width(1).border_radius(0),
        CheckboxStatus::Pressed => base.border_width(2).border_radius(5),
        CheckboxStatus::Focused => {
            base.border_width(1).border_radius(3).border_color(palette.selection_background)
        },
        CheckboxStatus::Checked => base.border_width(1).border_radius(0),
    }
}

const PADDING: u32 = 2;
const BORDER: u32 = 1;

pub struct Checkbox<'a, Message, R, S>
where
    R: Renderer,
    S: CheckboxStyler<R::Color> + IconStyler<R::Color>,
{
    id: ElId,
    check_icon: Icon<'a, R, S>,
    // size: Size<Length>,
    size: FontSize,
    on_change: Box<dyn Fn(bool) -> Message + 'a>,
    class: <S as CheckboxStyler<R::Color>>::Class<'a>,
}

impl<'a, Message, R, S> Checkbox<'a, Message, R, S>
where
    R: Renderer + 'a,
    S: CheckboxStyler<R::Color> + IconStyler<R::Color> + 'a,
{
    pub fn new<F>(on_change: F) -> Self
    where
        F: 'a + Fn(bool) -> Message,
    {
        Self {
            id: ElId::unique(),
            check_icon: Icon::new(crate::icons::IconKind::Check),
            // size: Size::fill(),
            size: FontSize::Relative(1.0),
            on_change: Box::new(on_change),
            class: <S as CheckboxStyler<R::Color>>::default(),
            // color: R::Color::default_foreground(),
        }
    }

    pub fn icon(mut self, icon: IconKind) -> Self {
        self.check_icon = Icon::new(icon);
        self
    }

    pub fn size(mut self, font_size: impl Into<FontSize>) -> Self {
        self.size = font_size.into();
        self
    }

    // Helpers //
    fn outer_size(&self, viewport: &Viewport) -> Size {
        Size::new_equal(self.size.to_real(viewport) + BORDER + PADDING)
    }

    fn status<E: Event + 'a>(&self, ctx: &UiCtx<Message>, state: &StateNode) -> CheckboxStatus {
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
    R: Renderer + 'a,
    E: Event + 'a,
    S: CheckboxStyler<R::Color> + IconStyler<R::Color> + 'a,
{
    fn id(&self) -> Option<ElId> {
        Some(self.id)
    }

    fn tree_ids(&self) -> Vec<ElId> {
        vec![self.id]
    }

    fn size(&self, viewport: &Viewport) -> Size<Length> {
        self.outer_size(viewport).into()
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
        viewport: &Viewport,
    ) -> crate::layout::LayoutNode {
        Layout::container(
            limits,
            self.outer_size(viewport),
            crate::layout::Position::Relative,
            viewport,
            BoxModel::new().border(BORDER).padding(PADDING),
            crate::align::Alignment::Center,
            crate::align::Alignment::Center,
            |limits| {
                Widget::<Message, R, E, S>::layout(
                    &self.check_icon,
                    ctx,
                    &mut StateNode::stateless(),
                    styler,
                    limits,
                    viewport,
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
        viewport: &Viewport,
    ) {
        let style = CheckboxStyler::style(styler, &self.class, self.status::<E>(ctx, state));
        let state = state.get::<CheckboxState>();

        let bounds = layout.bounds();

        renderer.block(crate::block::Block {
            border: style.border,
            rect: Rectangle::new(bounds.top_left, bounds.size.into()),
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
                viewport,
            )
        }
    }
}

impl<'a, Message, R, E, S> From<Checkbox<'a, Message, R, S>> for El<'a, Message, R, E, S>
where
    Message: 'a,
    R: Renderer + 'a,
    E: Event + 'a,
    S: CheckboxStyler<R::Color> + IconStyler<R::Color> + 'a,
{
    fn from(value: Checkbox<'a, Message, R, S>) -> Self {
        Self::new(value)
    }
}
