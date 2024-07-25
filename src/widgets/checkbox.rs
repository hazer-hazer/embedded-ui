use alloc::boxed::Box;
use alloc::vec::Vec;

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
    pressed: bool,
    checked: bool,
}

impl Default for CheckboxState {
    fn default() -> Self {
        Self { pressed: false, checked: false }
    }
}

#[derive(Clone, Copy)]
pub struct CheckboxStatus {
    focused: bool,
    pressed: bool,
    checked: bool,
}

component_style! {
    pub CheckboxStyle: CheckboxStyler(CheckboxStatus) default {primary} {
        background: background,
        border: border,
        outline: outline,
    }
}

pub fn primary<C: PaletteColor>(theme: &Theme<C>, status: CheckboxStatus) -> CheckboxStyle<C> {
    let palette = theme.palette();
    let base = CheckboxStyle::new(&palette)
        .background(palette.background)
        .outline_color(palette.selection_outline)
        .outline_width(0);

    match status {
        CheckboxStatus { pressed: true, focused: _, checked: _ } => {
            base.outline_width(2).outline_radius(5)
        },
        CheckboxStatus { focused: true, pressed: _, checked: _ } => {
            base.outline_width(1).outline_radius(3)
        },
        CheckboxStatus { checked: true, focused: _, pressed: _ } => {
            base.outline_width(1).outline_radius(0)
        },
        CheckboxStatus { focused: _, pressed: _, checked: _ } => {
            base.outline_width(1).outline_radius(0)
        },
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

    fn status<E: Event + 'a>(&self, ctx: &UiCtx<Message>, state: &CheckboxState) -> CheckboxStatus {
        let focused = UiCtx::is_focused::<R, E, S>(&ctx, self);
        let &CheckboxState { pressed, checked } = state;

        CheckboxStatus { focused, pressed, checked }
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
        _layout: Layout,
    ) -> crate::event::EventResponse<E> {
        let focused = UiCtx::is_focused::<R, E, S>(&ctx, self);
        let current_state = state.get::<CheckboxState>();

        if let Some(common) = event.as_common() {
            match common {
                CommonEvent::FocusMove(_) if focused => {
                    return Propagate::BubbleUp(self.id, event).into()
                },
                CommonEvent::FocusButtonDown if focused => {
                    state.get_mut::<CheckboxState>().pressed = true;
                    return Capture::Captured.into();
                },
                CommonEvent::FocusButtonUp if focused => {
                    let was_pressed = current_state.pressed;

                    state.get_mut::<CheckboxState>().pressed = false;

                    if was_pressed {
                        let new_state = !state.get::<CheckboxState>().checked;
                        state.get_mut::<CheckboxState>().checked = new_state;

                        ctx.publish((self.on_change)(new_state));

                        return Capture::Captured.into();
                    }
                },
                CommonEvent::FocusButtonDown
                | CommonEvent::FocusButtonUp
                | CommonEvent::FocusMove(_)
                | CommonEvent::Exit => {
                    // Should we reset state on any event? Or only on common
                    state.get_mut::<CheckboxState>().pressed = false;
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
            crate::align::Align::Center,
            crate::align::Align::Center,
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
        let state = state.get::<CheckboxState>();

        let style = CheckboxStyler::style(styler, &self.class, self.status::<E>(ctx, state));

        let bounds = layout.bounds();

        renderer.block(style.border.into_block(bounds, style.background));

        if state.checked {
            Widget::<Message, R, E, S>::draw(
                &self.check_icon,
                ctx,
                &mut StateNode::stateless(),
                renderer,
                styler,
                layout.first_child(),
                viewport,
            )
        }

        renderer.block(style.outline.into_outline(bounds));
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
