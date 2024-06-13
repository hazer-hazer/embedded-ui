use alloc::{
    boxed::Box,
    vec::{self, Vec},
};
use embedded_graphics::geometry::Point;

use crate::{
    block::Block,
    el::{El, ElId},
    event::{Capture, CommonEvent, Event, Propagate},
    icons::IconKind,
    layout::{Layout, LayoutNode, Limits, Viewport},
    padding::Padding,
    render::Renderer,
    size::{Length, Size},
    state::{State, StateNode, StateTag},
    style::component_style,
    ui::UiCtx,
    widget::Widget,
};

use super::icon::Icon;

pub struct SelectState {
    is_pressed: bool,
    is_active: bool,
}

impl Default for SelectState {
    fn default() -> Self {
        Self { is_pressed: false, is_active: false }
    }
}

#[derive(Clone, Copy)]
pub enum SelectStatus {
    Normal,
    Active,
    Pressed,
    Focused,
}

// pub type SelectStyleFn<'a, C> = Box<dyn Fn(SelectStatus) -> SelectStyle<C> + 'a>;

component_style! {
    pub SelectStyle: SelectStyler(SelectStatus) {
        background: background,
        border: border,
    }
}

pub struct SelectOption<'a, Message, R, E, S, V>
where
    R: Renderer,
    E: Event,
    S: SelectStyler<R::Color>,
{
    value: V,
    el: El<'a, Message, R, E, S>,
}

impl<'a, Message, R, E, S, V> SelectOption<'a, Message, R, E, S, V>
where
    R: Renderer,
    E: Event,
    S: SelectStyler<R::Color>,
{
    pub fn new(value: V, el: El<'a, Message, R, E, S>) -> Self {
        Self { value, el }
    }
}

impl<'a, Message, R, E, S, V, T> From<T> for SelectOption<'a, Message, R, E, S, V>
where
    R: Renderer,
    E: Event,
    S: SelectStyler<R::Color>,
    T: Into<(V, El<'a, Message, R, E, S>)>,
{
    fn from(value: T) -> Self {
        let (value, el) = value.into();
        Self::new(value, el)
    }
}

pub struct Select<'a, Message, R, E, S, V>
where
    R: Renderer,
    E: Event,
    S: SelectStyler<R::Color>,
{
    id: ElId,
    size: Size<Length>,
    icon_left: IconKind,
    icon_right: IconKind,
    options: Vec<SelectOption<'a, Message, R, E, S, V>>,
    chosen: usize,
    on_change: Option<Box<dyn Fn(&V) -> Message + 'a>>,
    class: S::Class<'a>,
    cycle: bool,
}

// impl<'a, Message, R, E, S, T> From<T> for Select<'a, Message, R, E, S, usize>
// where
//     R: Renderer,
//     E: Event,
//     S: SelectStyler<R::Color>,
//     T: IntoIterator<Item = El<'a, Message, R, E, S>>,
// {
//     fn from(value: T) -> Self {
//         Self::new_inner(value.into_iter().enumerate().map(Into::into).collect())
//     }
// }

// impl<'a, Message, R, E, S, T, V> From<T> for Select<'a, Message, R, E, S, V>
// where
//     R: Renderer,
//     E: Event,
//     S: SelectStyler<R::Color>,
//     T: IntoIterator<Item = (V, El<'a, Message, R, E, S>)>,
// {
//     fn from(value: T) -> Self {
//         Self::new_inner(value.into_iter().map(Into::into).collect())
//     }
// }

// impl<'a, Message, R, E, S, T, V> From<T> for Select<'a, Message, R, E, S, V>
// where
//     R: Renderer,
//     E: Event,
//     S: SelectStyler<R::Color>,
//     T: IntoIterator<Item = SelectOption<'a, Message, R, E, S, V>>,
// {
//     fn from(value: T) -> Self {
//         Self::new_inner(value.into_iter().collect())
//     }
// }

// impl<'a, Message, R, E, S, V> Select<'a, Message, R, E, S, V>
// where
//     R: Renderer,
//     E: Event,
//     S: SelectStyler<R::Color>,
// {
//     pub fn new_keyed(options: Vec<(V, El<'a, Message, R, E, S>)>) -> Self {
//         Self::new_inner(options.into_iter().map(Into::into).collect())
//     }
// }

impl<'a, Message, R, E, S, V> Select<'a, Message, R, E, S, V>
where
    R: Renderer,
    E: Event,
    S: SelectStyler<R::Color>,
{
    pub fn new(
        options: impl Iterator<Item = impl Into<SelectOption<'a, Message, R, E, S, V>>>,
    ) -> Self {
        Self {
            id: ElId::unique(),
            size: Size::fill(),
            icon_left: IconKind::ArrowLeft,
            icon_right: IconKind::ArrowRight,
            options: options.into_iter().map(Into::into).collect(),
            chosen: 0,
            on_change: None,
            class: S::default(),
            cycle: false,
        }
    }

    pub fn initial(mut self, index: impl Into<usize>) -> Self {
        self.chosen = index.into();
        self
    }

    pub fn on_change<F>(mut self, on_change: F) -> Self
    where
        F: Fn(&V) -> Message + 'a,
    {
        self.on_change = Some(Box::new(on_change));
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

    pub fn cycle(mut self, cycle: bool) -> Self {
        self.cycle = cycle;
        self
    }

    pub fn icon_left(mut self, icon_left: IconKind) -> Self {
        self.icon_left = icon_left;
        self
    }

    pub fn icon_right(mut self, icon_right: IconKind) -> Self {
        self.icon_right = icon_right;
        self
    }

    // Helpers //
    fn current(&self) -> &SelectOption<'a, Message, R, E, S, V> {
        &self.options[self.chosen]
    }

    fn arrow_icon_size(&self, _limits: &Limits) -> u32 {
        // limits.max().height
        // TODO
        5
    }

    fn status(&self, ctx: &UiCtx<Message>, state: &StateNode) -> SelectStatus {
        match state.get::<SelectState>() {
            SelectState { is_active: true, .. } => return SelectStatus::Active,
            SelectState { is_pressed: true, .. } => return SelectStatus::Pressed,
            SelectState { is_pressed: false, is_active: false } => {},
        }

        if ctx.is_focused(self) {
            return SelectStatus::Focused;
        }

        SelectStatus::Normal
    }
}

impl<'a, Message, R, E, S, V> Widget<Message, R, E, S> for Select<'a, Message, R, E, S, V>
where
    R: Renderer,
    E: Event,
    S: SelectStyler<R::Color>,
{
    fn id(&self) -> Option<crate::el::ElId> {
        Some(self.id)
    }

    fn tree_ids(&self) -> Vec<crate::el::ElId> {
        vec![self.id]
        // TODO: Maybe Select should hide ids of its children or we might fail on focusing them
        // self.options.iter().map(|option| option.tree_ids()).flatten().collect()
    }

    fn size(&self) -> Size<Length> {
        self.size
    }

    fn state_tag(&self) -> crate::state::StateTag {
        StateTag::of::<SelectState>()
    }

    fn state(&self) -> State {
        State::new(SelectState::default())
    }

    fn state_children(&self) -> Vec<StateNode> {
        // TODO: Do we need to tell about children?
        vec![StateNode::new(&self.current().el)]
    }

    fn on_event(
        &mut self,
        ctx: &mut crate::ui::UiCtx<Message>,
        event: E,
        state: &mut StateNode,
    ) -> crate::event::EventResponse<E> {
        // TODO: Think about need of passing events to children, is it safe?

        let focused = ctx.is_focused(self);
        let current_state = state.get::<SelectState>();

        if let Some(offset) = event.as_select_shift() {
            if focused && current_state.is_active {
                let prev = self.chosen;
                if self.cycle {
                    let len = self.options.len() as i32;
                    self.chosen = ((self.chosen as i32 + offset % len + len) % len) as usize;
                } else {
                    self.chosen = (self.chosen as i32 + offset)
                        .clamp(0, self.options.len() as i32 - 1)
                        as usize;
                }
                if let Some(on_change) = self.on_change.as_ref() {
                    if prev != self.chosen {
                        ctx.publish((on_change)(&self.current().value));
                    }
                }
                return Capture::Captured.into();
            }
        }

        if let Some(common) = event.as_common() {
            match common {
                CommonEvent::FocusMove(_) if focused => {
                    return Propagate::BubbleUp(self.id, event).into()
                },
                CommonEvent::FocusClickDown if focused => {
                    state.get_mut::<SelectState>().is_pressed = true;
                    return Capture::Captured.into();
                },
                CommonEvent::FocusClickUp if focused => {
                    let was_pressed = current_state.is_pressed;

                    state.get_mut::<SelectState>().is_pressed = false;

                    if was_pressed {
                        state.get_mut::<SelectState>().is_active =
                            !state.get::<SelectState>().is_active;
                        return Capture::Captured.into();
                    }
                },
                CommonEvent::FocusClickDown
                | CommonEvent::FocusClickUp
                | CommonEvent::FocusMove(_) => {
                    // Should we reset state on any event? Or only on common
                    state.state.reset::<SelectState>();
                },
            }
        }

        Propagate::Ignored.into()
    }

    fn layout(
        &self,
        ctx: &mut crate::ui::UiCtx<Message>,
        state: &mut crate::state::StateNode,
        styler: &S,
        limits: &crate::layout::Limits,
        viewport: &Viewport,
    ) -> crate::layout::LayoutNode {
        // Layout::container(limits, self.size.width, self.size.height, |limits| {
        //     // TODO: Use real icons layouts to be accurate?

        //     // Reserve some space for arrows on the sides
        //     let shrink_by_arrows = limits.max_square() * 2;
        //     self.options[self.chosen].layout(ctx, state, styler, &limits.shrink(shrink_by_arrows))
        // })

        let style = styler.style(&self.class, self.status(ctx, state));

        // TODO: Smarter icon size
        let padding_for_icons = self.arrow_icon_size(limits);

        Layout::container(
            limits,
            self.size,
            crate::layout::Position::Relative,
            viewport,
            Padding::new_axis(0, padding_for_icons),
            style.border.width,
            crate::align::Alignment::Center,
            crate::align::Alignment::Center,
            |limits| {
                self.options[self.chosen].el.layout(
                    ctx,
                    &mut state.children[0],
                    styler,
                    limits,
                    viewport,
                )
            },
        )
    }

    fn draw(
        &self,
        ctx: &mut crate::ui::UiCtx<Message>,
        state: &mut crate::state::StateNode,
        renderer: &mut R,
        styler: &S,
        layout: crate::layout::Layout,
    ) {
        let bounds = layout.bounds();
        let icons_limits = Limits::new(Size::zero(), Size::new_equal(bounds.size.height));
        let icons_node = LayoutNode::new(self.arrow_icon_size(&icons_limits).into());
        let icons_vertical_center =
            bounds.size.height as i32 / 2 - icons_node.size().height as i32 / 2;

        let style = styler.style(&self.class, self.status(ctx, state));

        renderer.block(Block {
            border: style.border,
            rect: bounds.into(),
            background: style.background,
        });

        if self.cycle || self.chosen != 0 {
            Widget::<Message, R, E, S>::draw(
                &Icon::new(self.icon_left),
                ctx,
                &mut StateNode::stateless(),
                renderer,
                styler,
                Layout::with_offset(
                    bounds.position + Point::new(style.border.width as i32, icons_vertical_center),
                    &icons_node,
                ),
            );
        }

        self.current().el.draw(
            ctx,
            &mut state.children[0],
            renderer,
            styler,
            layout.children().next().unwrap(),
        );

        if self.cycle || self.chosen != self.options.len() - 1 {
            Widget::<Message, R, E, S>::draw(
                &Icon::new(self.icon_right),
                ctx,
                &mut StateNode::stateless(),
                renderer,
                styler,
                Layout::with_offset(
                    bounds.position
                        + Point::new(
                            bounds.size.width as i32
                                - icons_node.size().width as i32
                                - style.border.width as i32,
                            icons_vertical_center,
                        ),
                    &icons_node,
                ),
            );
        }
    }
}

impl<'a, Message, R, E, S, V> From<Select<'a, Message, R, E, S, V>> for El<'a, Message, R, E, S>
where
    Message: Clone + 'a,
    R: Renderer + 'a,
    E: Event + 'a,
    S: 'a,
    S: SelectStyler<R::Color> + 'a,
    V: 'a,
{
    fn from(value: Select<'a, Message, R, E, S, V>) -> Self {
        El::new(value)
    }
}
