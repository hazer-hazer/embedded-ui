use alloc::boxed::Box;

use crate::{
    block::{Block, Border, BorderRadius},
    color::UiColor,
    el::{El, ElId},
    event::{Capture, CommonEvent, Event, EventResponse, Propagate},
    layout::Layout,
    padding::Padding,
    render::Renderer,
    size::{Length, Size},
    state::{self, StateNode, StateTag},
    ui::UiCtx,
    widget::Widget,
};

// TODO: Double-click
struct ButtonState {
    is_pressed: bool,
}

impl Default for ButtonState {
    fn default() -> Self {
        Self { is_pressed: false }
    }
}

#[derive(Clone, Copy)]
pub enum ButtonStatus {
    Active,
    Focused,
    Pressed,
    // Disabled,
    // Hovered,
}

pub type ButtonStyleFn<'a, C> = Box<dyn Fn(ButtonStatus) -> ButtonStyle<C> + 'a>;

pub trait ButtonStyler<C: UiColor> {
    type Class<'a>;

    fn default<'a>() -> Self::Class<'a>;
    fn style(&self, class: &Self::Class<'_>, status: ButtonStatus) -> ButtonStyle<C>;
}

#[derive(Clone, Copy)]
pub struct ButtonStyle<C: UiColor> {
    // TODO: Inner content styles, like text color
    background: C,
    border: Border<C>,
}

impl<C: UiColor> ButtonStyle<C> {
    pub fn new() -> Self {
        Self { background: C::default_background(), border: Border::new() }
    }

    pub fn background(mut self, background: impl Into<C>) -> Self {
        self.background = background.into();
        self
    }

    pub fn border_color(mut self, color: impl Into<C>) -> Self {
        self.border.color = color.into();
        self
    }

    pub fn border_width(mut self, width: u32) -> Self {
        self.border.width = width;
        self
    }

    pub fn border_radius(mut self, radius: impl Into<BorderRadius>) -> Self {
        self.border.radius = radius.into();
        self
    }
}

pub struct Button<'a, Message, R: Renderer, E: Event, S: ButtonStyler<R::Color>> {
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

    fn status(&self, ctx: &UiCtx<Message>, state_tree: &mut StateNode) -> ButtonStatus {
        if state_tree.state.downcast_ref::<ButtonState>().is_pressed {
            ButtonStatus::Pressed
        } else if ctx.is_focused(self) {
            ButtonStatus::Focused
        } else {
            ButtonStatus::Active
        }
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

    fn tree_ids(&self) -> alloc::vec::Vec<ElId> {
        let mut ids = vec![self.id];
        ids.extend(self.content.tree_ids());
        ids
    }

    fn size(&self) -> Size<Length> {
        self.size
    }

    fn state_tag(&self) -> StateTag {
        StateTag::of::<ButtonState>()
    }

    fn state(&self) -> state::State {
        state::State::new(ButtonState::default())
    }

    fn state_children(&self) -> alloc::vec::Vec<state::StateNode> {
        vec![StateNode::new(&self.content)]
    }

    fn on_event(
        &mut self,
        ctx: &mut UiCtx<Message>,
        event: E,
        state_tree: &mut StateNode,
    ) -> EventResponse<E> {
        match self.content.on_event(ctx, event.clone(), &mut state_tree.children[0])? {
            Propagate::Ignored => match event.as_common() {
                Some(common) => match common {
                    // Tell parent that this child is the currently focused so parent can use it as an offset of focus
                    CommonEvent::FocusMove(_) if ctx.is_focused(self) => {
                        Propagate::BubbleUp(self.id, event).into()
                    },
                    CommonEvent::FocusClickDown if ctx.is_focused(self) => {
                        state_tree.state.downcast_mut::<ButtonState>().is_pressed = true;

                        Capture::Captured.into()
                    },
                    CommonEvent::FocusClickUp if ctx.is_focused(self) => {
                        // Button was clicked (focus click was down and now released)

                        state_tree.state.downcast_mut::<ButtonState>().is_pressed = false;

                        if let Some(on_press) = self.on_press.clone() {
                            ctx.publish(on_press)
                        }

                        Capture::Captured.into()
                    },
                    CommonEvent::FocusClickDown
                    | CommonEvent::FocusClickUp
                    | CommonEvent::FocusMove(_) => {
                        // Reset pressed state on click on other element
                        state_tree.state.downcast_mut::<ButtonState>().is_pressed = false;

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
        state_tree: &mut StateNode,
        styler: &S,
        limits: &crate::layout::Limits,
    ) -> crate::layout::LayoutNode {
        let style = styler.style(&self.class, self.status(ctx, state_tree));

        Layout::padded(
            limits,
            self.size.width,
            self.size.height,
            self.padding,
            style.border.width,
            |limits| self.content.layout(ctx, &mut state_tree.children[0], styler, limits),
        )
    }

    fn draw(
        &self,
        ctx: &mut UiCtx<Message>,
        state_tree: &mut StateNode,
        renderer: &mut R,
        styler: &S,
        layout: Layout,
    ) {
        let bounds = layout.bounds();

        let style = styler.style(&self.class, self.status(ctx, state_tree));

        renderer.block(&Block {
            border: style.border,
            rect: bounds.into(),
            background: R::Color::default_background(),
        });

        self.content.draw(
            ctx,
            &mut state_tree.children[0],
            renderer,
            styler,
            layout.children().next().unwrap(),
        )
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
