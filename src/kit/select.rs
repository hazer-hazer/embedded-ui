use alloc::{boxed::Box, vec::Vec};
use embedded_graphics::{geometry::Point, primitives::Rectangle};

use crate::{
    axis::{Axial, Axis},
    block::Block,
    color::UiColor,
    el::{El, ElId},
    event::{Capture, CommonEvent, Event, Propagate},
    font::FontSize,
    icons::IconKind,
    layout::{Layout, LayoutNode, Limits, Viewport},
    padding::Padding,
    palette::PaletteColor,
    render::Renderer,
    size::{Length, Size},
    state::{State, StateNode, StateTag},
    style::component_style,
    theme::Theme,
    ui::UiCtx,
    widget::Widget,
};

use super::{
    icon::{Icon, IconStyler},
    text::{Text, TextStyler},
};

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
pub struct SelectStatus {
    active: bool,
    pressed: bool,
    focused: bool,
}

// pub type SelectStyleFn<'a, C> = Box<dyn Fn(SelectStatus) -> SelectStyle<C> +
// 'a>;

component_style! {
    pub SelectStyle: SelectStyler(SelectStatus) default {primary} {
        background: background,
        border: border,
        selected_background: background,
        selected_border: border {
            selected_border_color: border_color,
            selected_border_width: border_width,
            selected_border_radius: border_radius,
        },
    }
}

pub fn primary<C: PaletteColor>(theme: &Theme<C>, status: SelectStatus) -> SelectStyle<C> {
    let palette = theme.palette();
    let base = SelectStyle::new(&palette)
        .background(palette.background)
        .border_color(palette.background)
        .selected_background(palette.primary)
        .selected_border_width(0);

    match status {
        SelectStatus { pressed: true, active: _, focused: _ } => {
            base.border_color(palette.selection_background).border_width(2).border_radius(3)
        },
        SelectStatus { active: true, pressed: false, focused: _ } => {
            base.border_color(palette.selection_background).border_width(1).border_radius(5)
        },
        SelectStatus { active: false, pressed: false, focused: true } => {
            base.border_color(palette.selection_background).border_width(1).border_radius(0)
        },
        SelectStatus { .. } => base.border_width(1).border_radius(0),
    }
}

// pub struct SelectOption<'a, Message, R, E, S, V>
// where
//     R: Renderer,
//     E: Event,
//     S: SelectStyler<R::Color>,
// {
//     value: V,
//     el: El<'a, Message, R, E, S>,
// }

// impl<'a, Message, R, E, S, V> SelectOption<'a, Message, R, E, S, V>
// where
//     R: Renderer,
//     E: Event,
//     S: SelectStyler<R::Color>,
// {
//     fn new(value: V, el: El<'a, Message, R, E, S>) -> Self {
//         Self { value, el }
//     }

//     fn el(&self) -> &El<'a, Message, R, E, S> {
//         &self.el
//     }
// }

// impl<'a, Message, R, E, S, V, T> From<T> for SelectOption<'a, Message, R, E,
// S, V> where
//     R: Renderer,
//     E: Event,
//     S: SelectStyler<R::Color>,
//     T: Into<(V, El<'a, Message, R, E, S>)>,
// {
//     fn from(value: T) -> Self {
//         let (value, el) = value.into();
//         Self::new(value, el)
//     }
// }

pub trait SelectOption<'a, Message, R, E, S>
where
    R: Renderer,
    E: Event,
{
    type Value: Copy;

    fn into_el(self) -> El<'a, Message, R, E, S>;
    fn value(&self) -> Self::Value;
}

impl<'a, Message, R, E, S, V: Copy> SelectOption<'a, Message, R, E, S> for (V, &'a str)
where
    Message: 'a,
    R: Renderer + 'a,
    E: Event + 'a,
    S: TextStyler<R::Color> + 'a,
{
    type Value = V;

    fn into_el(self) -> El<'a, Message, R, E, S> {
        self.1.into()
    }

    fn value(&self) -> Self::Value {
        self.0
    }
}

pub struct Select<'a, Message, R, E, S, O>
where
    R: Renderer,
    E: Event,
    S: SelectStyler<R::Color>,
    O: SelectOption<'a, Message, R, E, S>,
{
    id: ElId,
    size: Size<Length>,
    icon_prev: IconKind,
    icon_next: IconKind,
    option_els: Vec<El<'a, Message, R, E, S>>,
    option_values: Vec<O::Value>,
    chosen: usize,
    on_change: Option<Box<dyn Fn(&O::Value) -> Message + 'a>>,
    class: S::Class<'a>,
    cycle: bool,
    axis: Axis,
}

impl<'a, Message, R, E, S, O> Select<'a, Message, R, E, S, O>
where
    R: Renderer,
    E: Event,
    S: SelectStyler<R::Color> + IconStyler<R::Color>,
    O: SelectOption<'a, Message, R, E, S>,
{
    pub fn new(axis: Axis, options: impl Iterator<Item = O>) -> Self {
        let (icon_prev, icon_next) = Self::icons_by_axis(axis);

        let (option_values, option_els) =
            options.map(|option| (option.value(), option.into_el())).unzip();

        Self {
            id: ElId::unique(),
            size: Size::fill(),
            icon_prev,
            icon_next,
            option_els,
            option_values,
            chosen: 0,
            on_change: None,
            class: <S as SelectStyler<R::Color>>::default(),
            cycle: false,
            axis,
        }
    }

    pub fn horizontal(options: impl Iterator<Item = O>) -> Self {
        Self::new(Axis::X, options)
    }

    pub fn vertical(options: impl Iterator<Item = O>) -> Self {
        Self::new(Axis::Y, options)
    }

    pub fn initial(mut self, index: impl Into<usize>) -> Self {
        self.chosen = index.into();
        self
    }

    pub fn on_change<F>(mut self, on_change: F) -> Self
    where
        F: Fn(&O::Value) -> Message + 'a,
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

    pub fn icon_prev(mut self, icon_prev: IconKind) -> Self {
        self.icon_prev = icon_prev;
        self
    }

    pub fn icon_next(mut self, icon_next: IconKind) -> Self {
        self.icon_next = icon_next;
        self
    }

    // Helpers //
    fn icons_by_axis(axis: Axis) -> (IconKind, IconKind) {
        match axis {
            Axis::X => (IconKind::ArrowLeft, IconKind::ArrowRight),
            Axis::Y => (IconKind::ArrowUp, IconKind::ArrowDown),
        }
    }

    fn current_value(&self) -> &O::Value {
        &self.option_values[self.chosen]
    }

    fn current_el(&self) -> &El<'a, Message, R, E, S> {
        &self.option_els[self.chosen]
    }

    fn arrow_icon_size(&self, viewport: &Viewport) -> u32 {
        FontSize::Relative(1.0).to_real(viewport)
    }

    fn status(&self, ctx: &UiCtx<Message>, state: &StateNode) -> SelectStatus {
        let state = state.get::<SelectState>();

        SelectStatus {
            active: state.is_active,
            pressed: state.is_pressed,
            focused: ctx.is_focused(self),
        }
    }
}

impl<'a, Message, R, E, S, O> Widget<Message, R, E, S> for Select<'a, Message, R, E, S, O>
where
    R: Renderer,
    E: Event,
    S: SelectStyler<R::Color> + IconStyler<R::Color>,
    O: SelectOption<'a, Message, R, E, S>,
{
    fn id(&self) -> Option<crate::el::ElId> {
        Some(self.id)
    }

    fn tree_ids(&self) -> Vec<crate::el::ElId> {
        vec![self.id]
        // TODO: Maybe Select should hide ids of its children or we might fail
        // on focusing them self.options.iter().map(|option|
        // option.tree_ids()).flatten().collect()
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
        vec![StateNode::new(self.current_el())]
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
                    let len = self.option_els.len() as i32;
                    self.chosen = ((self.chosen as i32 + offset % len + len) % len) as usize;
                } else {
                    self.chosen = (self.chosen as i32 + offset)
                        .clamp(0, self.option_els.len() as i32 - 1)
                        as usize;
                }
                if let Some(on_change) = self.on_change.as_ref() {
                    if prev != self.chosen {
                        ctx.publish((on_change)(self.current_value()));
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
        let style = SelectStyler::style(styler, &self.class, self.status(ctx, state));

        let padding_for_icons = self.arrow_icon_size(viewport);

        Layout::sized(limits, self.size, self.position(), viewport, |limits| limits.max())

        // Layout::flex(
        //     ctx,
        //     state,
        //     styler,
        //     self.axis,
        //     limits,
        //     self.size,
        //     crate::layout::Position::Relative,
        //     viewport,
        //     Padding::zero(),
        //     1,
        //     crate::align::Alignment::Center,
        //     self.current_siblings().as_slice(),
        // )

        // Layout::container(
        //     limits,
        //     self.size,
        //     crate::layout::Position::Relative,
        //     viewport,
        //     self.axis.canon::<Padding>(0, padding_for_icons),
        //     style.border.width,
        //     crate::align::Alignment::Center,
        //     crate::align::Alignment::Center,
        //     |limits| {
        //         self.options[self.chosen].el.layout(
        //             ctx,
        //             &mut state.children[0],
        //             styler,
        //             limits,
        //             viewport,
        //         )
        //     },
        // )
    }

    fn draw(
        &self,
        ctx: &mut crate::ui::UiCtx<Message>,
        state: &mut crate::state::StateNode,
        renderer: &mut R,
        styler: &S,
        layout: crate::layout::Layout,
        viewport: &Viewport,
    ) {
        let bounds = layout.bounds();
        let icons_node = LayoutNode::new(self.arrow_icon_size(viewport).into());
        let icons_cross_center = bounds.size.cross_for(self.axis) as i32 / 2
            - icons_node.size().cross_for(self.axis) as i32 / 2;

        let style = SelectStyler::style(styler, &self.class, self.status(ctx, state));

        renderer.block(Block {
            border: style.border,
            rect: bounds.into(),
            background: style.selected_background,
        });

        if self.cycle || self.chosen != 0 {
            Widget::<Message, R, E, S>::draw(
                &Icon::new(self.icon_prev),
                ctx,
                &mut StateNode::stateless(),
                renderer,
                styler,
                Layout::with_offset(
                    bounds.position
                        + self.axis.canon::<Point>(style.border.width as i32, icons_cross_center),
                    &icons_node,
                ),
                viewport,
            );
        }

        let value_layout = layout.children().next().unwrap();

        renderer.block(Block {
            border: style.selected_border,
            rect: Into::<Rectangle>::into(value_layout.bounds()).resized(
                (value_layout.bounds().size - 1).into(),
                embedded_graphics::geometry::AnchorPoint::Center,
            ),
            background: style.background,
        });

        self.current_el().draw(
            ctx,
            &mut state.children[0],
            renderer,
            styler,
            value_layout,
            viewport,
        );

        if self.cycle || self.chosen != self.option_els.len() - 1 {
            Widget::<Message, R, E, S>::draw(
                &Icon::new(self.icon_next),
                ctx,
                &mut StateNode::stateless(),
                renderer,
                styler,
                Layout::with_offset(
                    bounds.position
                        + self.axis.canon::<Point>(
                            bounds.size.main_for(self.axis) as i32
                                - icons_node.size().main_for(self.axis) as i32
                                - style.border.width as i32,
                            icons_cross_center,
                        ),
                    &icons_node,
                ),
                viewport,
            );
        }
    }
}

impl<'a, Message, R, E, S, O> From<Select<'a, Message, R, E, S, O>> for El<'a, Message, R, E, S>
where
    Message: Clone + 'a,
    R: Renderer + 'a,
    E: Event + 'a,
    S: 'a,
    S: SelectStyler<R::Color> + IconStyler<R::Color> + 'a,
    O: SelectOption<'a, Message, R, E, S> + 'a,
{
    fn from(value: Select<'a, Message, R, E, S, O>) -> Self {
        El::new(value)
    }
}
