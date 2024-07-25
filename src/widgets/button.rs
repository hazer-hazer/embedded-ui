use alloc::vec::Vec;

use crate::{
    align::Align,
    block::BoxModel,
    el::{El, ElId},
    event::{Capture, CommonEvent, Event, EventResponse, Propagate},
    layout::{Layout, Viewport},
    padding::Padding,
    palette::PaletteColor,
    render::Renderer,
    size::{Length, Size},
    state::{self, StateNode, StateTag},
    style::component_style,
    theme::Theme,
    ui::UiCtx,
    widget::Widget,
};

// TODO: Double-click (needs time source)
struct ButtonState {
    pressed: bool,
}

impl Default for ButtonState {
    fn default() -> Self {
        Self { pressed: false }
    }
}

#[derive(Clone, Copy)]
pub struct ButtonStatus {
    focused: bool,
    pressed: bool,
    // Disabled,
    // Hovered,
}

// pub type ButtonStyleFn<'a, C> = Box<dyn Fn(ButtonStatus) -> ButtonStyle<C> +
// 'a>;

component_style! {
    pub ButtonStyle: ButtonStyler(ButtonStatus) default {primary} {
        background: background,
        border: border,
        outline: outline,
    }
}

pub fn primary<C: PaletteColor>(theme: &Theme<C>, status: ButtonStatus) -> ButtonStyle<C> {
    let palette = theme.palette();
    let base = ButtonStyle::new(&palette)
        .background(palette.primary)
        .border_radius(3)
        .outline_width(0)
        .outline_color(palette.selection_outline);

    match status {
        crate::widgets::button::ButtonStatus { pressed: true, focused: _ } => {
            base.outline_width(2).outline_radius(7).background(palette.background)
        },
        crate::widgets::button::ButtonStatus { focused: true, pressed: _ } => {
            base.outline_width(1).outline_radius(5)
        },
        crate::widgets::button::ButtonStatus { .. } => base,
    }
}

pub struct Button<'a, Message, R, E, S>
where
    R: Renderer,
    E: Event,
    S: ButtonStyler<R::Color>,
{
    id: ElId,
    content: El<'a, Message, R, E, S>,
    size: Size<Length>,
    padding: Padding,
    class: S::Class<'a>,
    on_press: Option<Message>,
}

impl<'a, Message, R, E, S> Button<'a, Message, R, E, S>
where
    Message: Clone,
    R: Renderer,
    E: Event,
    S: ButtonStyler<R::Color>,
{
    pub fn new(content: impl Into<El<'a, Message, R, E, S>>) -> Self {
        let content = content.into();
        let padding = Padding::default();
        // let size = content.size();

        Self {
            id: ElId::unique(),
            content,
            size: Size::fill(),
            padding,
            class: S::default(),
            on_press: None,
        }
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.size.width = width.into();
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.size.height = height.into();
        self
    }

    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    pub fn on_press(mut self, on_press: impl Into<Message>) -> Self {
        self.on_press = Some(on_press.into());
        self
    }

    pub fn store_id(self, id: &mut ElId) -> Self {
        *id = self.id;
        self
    }

    pub fn identify(mut self, id: impl Into<ElId>) -> Self {
        self.id = id.into();
        self
    }

    fn status(&self, ctx: &UiCtx<Message>, state: &mut StateNode) -> ButtonStatus {
        ButtonStatus { focused: ctx.is_focused(self), pressed: state.get::<ButtonState>().pressed }
    }
}

impl<'a, Message, R, E, S> Widget<Message, R, E, S> for Button<'a, Message, R, E, S>
where
    Message: Clone,
    R: Renderer,
    E: Event,
    S: ButtonStyler<R::Color>,
{
    fn id(&self) -> Option<ElId> {
        Some(self.id)
    }

    fn tree_ids(&self) -> Vec<ElId> {
        let mut ids = vec![self.id];
        ids.extend(self.content.tree_ids());
        ids
    }

    fn size(&self, _viewport: &Viewport) -> Size<Length> {
        self.size
    }

    fn state_tag(&self) -> StateTag {
        StateTag::of::<ButtonState>()
    }

    fn state(&self) -> state::State {
        state::State::new(ButtonState::default())
    }

    fn state_children(&self) -> Vec<state::StateNode> {
        vec![StateNode::new(&self.content)]
    }

    fn on_event(
        &mut self,
        ctx: &mut UiCtx<Message>,
        event: E,
        state: &mut StateNode,
        layout: Layout,
    ) -> EventResponse<E> {
        match self.content.on_event(
            ctx,
            event.clone(),
            &mut state.children[0],
            layout.first_child(),
        )? {
            Propagate::Ignored => match event.as_common() {
                Some(common) => match common {
                    // Tell parent that this child is the currently focused so parent can use it
                    // as an offset of focus
                    CommonEvent::FocusMove(_) if ctx.is_focused(self) => {
                        Propagate::BubbleUp(self.id, event).into()
                    },
                    CommonEvent::FocusButtonDown if ctx.is_focused(self) => {
                        state.get_mut::<ButtonState>().pressed = true;

                        Capture::Captured.into()
                    },
                    CommonEvent::FocusButtonUp if ctx.is_focused(self) => {
                        // Button was clicked only if
                        // - Focus wasn't moved
                        // - Focus button was down on it
                        // - Focus button released on it

                        let pressed = state.get::<ButtonState>().pressed;

                        state.get_mut::<ButtonState>().pressed = false;

                        if pressed {
                            if let Some(on_press) = self.on_press.clone() {
                                ctx.publish(on_press);
                                return Capture::Captured.into();
                            }
                        }

                        Propagate::Ignored.into()
                    },
                    CommonEvent::FocusButtonDown
                    | CommonEvent::FocusButtonUp
                    | CommonEvent::FocusMove(_)
                    | CommonEvent::Exit => {
                        // Reset pressed state on click on other element
                        state.get_mut::<ButtonState>().pressed = false;

                        Propagate::Ignored.into()
                    },
                },
                None => Propagate::Ignored.into(),
            },
            bubbled @ Propagate::BubbleUp(..) => bubbled.into(),
        }
    }

    fn layout(
        &self,
        ctx: &mut UiCtx<Message>,
        state: &mut StateNode,
        styler: &S,
        limits: &crate::layout::Limits,
        viewport: &Viewport,
    ) -> crate::layout::LayoutNode {
        let style = styler.style(&self.class, self.status(ctx, state));

        Layout::container(
            limits,
            self.size,
            crate::layout::Position::Relative,
            viewport,
            BoxModel::new().padding(self.padding).border(style.border),
            Align::Center,
            Align::Center,
            |limits| self.content.layout(ctx, &mut state.children[0], styler, limits, viewport),
        )
    }

    fn draw(
        &self,
        ctx: &mut UiCtx<Message>,
        state: &mut StateNode,
        renderer: &mut R,
        styler: &S,
        layout: Layout,
        viewport: &Viewport,
    ) {
        let bounds = layout.bounds();

        let style = styler.style(&self.class, self.status(ctx, state));

        renderer.block(style.border.into_block(bounds, style.background));

        self.content.draw(
            ctx,
            &mut state.children[0],
            renderer,
            styler,
            layout.first_child(),
            viewport,
        );

        renderer.block(style.outline.into_outline(bounds));
    }
}

impl<'a, Message, R, E, S> From<Button<'a, Message, R, E, S>> for El<'a, Message, R, E, S>
where
    Message: Clone + 'a,
    R: Renderer + 'a,
    E: Event + 'a,
    S: ButtonStyler<R::Color> + 'a,
{
    fn from(value: Button<'a, Message, R, E, S>) -> Self {
        Self::new(value)
    }
}
