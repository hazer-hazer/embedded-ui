use alloc::collections::VecDeque;
use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::{BinaryColor, Rgb555, Rgb565, Rgb666, Rgb888},
};

use crate::{
    color::UiColor,
    el::{El, ElId},
    event::{Event, EventStub, Propagate},
    layout::{Layout, LayoutNode, Limits, Viewport},
    palette::PaletteColor,
    render::{DrawTargetRenderer, Renderer},
    size::Size,
    state::StateNode,
    style::Styler,
    theme::Theme,
    widget::Widget,
};

/// Global UI states collection
pub struct UiCtx<Message> {
    message_pool: VecDeque<Message>,
    focused: Option<ElId>,
}

impl<Message> UiCtx<Message> {
    pub fn new() -> Self {
        Self { message_pool: VecDeque::new(), focused: None }
    }

    pub fn focus(&mut self, id: ElId) {
        self.focused = Some(id)
    }

    pub fn is_focused<R: Renderer, E: Event, S>(
        &self,
        widget: &impl Widget<Message, R, E, S>,
    ) -> bool {
        match (self.focused, widget.id()) {
            (Some(focus), Some(id)) if focus == id => true,
            _ => false,
        }
    }

    pub fn no_focus(&self) -> bool {
        self.focused.is_none()
    }

    pub fn publish(&mut self, message: Message) {
        self.message_pool.push_back(message)
    }
}

pub struct UI<
    'a,
    Message,
    R: Renderer,
    E: Event,
    S: Styler<R::Color> = Theme<<R as Renderer>::Color>,
> {
    root: El<'a, Message, R, E, S>,
    root_node: LayoutNode,
    viewport_size: Size,
    root_state: StateNode,
    styler: S,
    // events: Vec<E>,
    ctx: UiCtx<Message>,
}

impl<'a, Message, C, E, S> UI<'a, Message, DrawTargetRenderer<C>, E, S>
where
    C: UiColor,
    E: Event,
    S: Styler<C>,
{
    pub fn draw<D>(&mut self, target: &mut D)
    where
        D: DrawTarget<Color = C>,
        D::Error: core::fmt::Debug,
    {
        // FIXME: Performance?
        // TODO: Maybe should clear only root bounds
        // renderer.clear(self.styler.background());
        let mut renderer =
            DrawTargetRenderer::new(target.bounding_box().size, self.styler.background());

        self.root.draw(
            &mut self.ctx,
            &mut self.root_state,
            &mut renderer,
            &self.styler,
            Layout::new(&self.root_node),
            &Viewport { size: self.viewport_size },
        );

        renderer.finish(target);
    }
}

impl<'a, Message, R: Renderer, E: Event, S: Styler<R::Color>> UI<'a, Message, R, E, S> {
    pub fn new(root: impl Widget<Message, R, E, S> + 'a, viewport_size: Size) -> Self {
        let mut ctx = UiCtx::new();

        let root = El::new(root);
        let mut root_state = StateNode::new(&root);

        let root_node = root.layout(
            &mut ctx,
            &mut root_state,
            &Default::default(),
            &Limits::only_max(viewport_size),
            &Viewport { size: viewport_size },
        );

        Self {
            root,
            root_node,
            viewport_size,
            root_state,
            // events: Vec::new(),
            styler: Default::default(),
            ctx,
        }
    }

    // pub fn feed_events(&mut self, events: impl Iterator<Item = E>) {
    //     self.events.extend(events)
    // }

    pub fn deque_message(&mut self) -> Option<Message> {
        self.ctx.message_pool.pop_back()
    }

    pub fn tick(&mut self, events: impl Iterator<Item = E>) {
        for event in events {
            if let core::ops::ControlFlow::Continue(propagate) = self.root.on_event(
                &mut self.ctx,
                event.clone(),
                &mut self.root_state,
                Layout::new(&self.root_node),
            ) {
                match propagate {
                    Propagate::BubbleUp(bubble_origin, bubbled) => {
                        // debug!("Capture Bubble up event {bubbled:?} from {bubble_origin:?}");
                        if let Some(common) = bubbled.as_common() {
                            match common {
                                crate::event::CommonEvent::FocusMove(offset) => {
                                    let tree = self.root.tree_ids();
                                    let current = tree
                                        .iter()
                                        .position(|&id| id == bubble_origin)
                                        .unwrap_or(0)
                                        as i32;
                                    let next_focus = current
                                        .saturating_add(offset)
                                        .clamp(0, tree.len().saturating_sub(1) as i32);
                                    self.ctx.focus(tree[next_focus as usize]);
                                    // return Capture::Captured.into();
                                    return;
                                },
                                _ => {},
                            }
                        }
                    },
                    Propagate::Ignored => {
                        // debug!("Ignored event {event:?}");
                    },
                }
            } else {
                // debug!("Some element captured event {event:?}");
            }
            // TODO: Debug log "ignored event"
            // Propagate::Ignored.into()
        }
    }

    pub fn auto_focus(&mut self) {
        if let Some(first_el) = self.root.tree_ids().first().copied() {
            self.ctx.focus(first_el)
        }
    }

    pub fn focus(&mut self, id: ElId) {
        self.ctx.focus(id)
    }
}

/// Does not have events
impl<'a, Message, R: Renderer, S: Styler<R::Color>> UI<'a, Message, R, EventStub, S> {
    pub fn no_events(self) -> Self {
        self
    }
}

/// Does not allow messages
impl<'a, R: Renderer, E: Event, S: Styler<R::Color>> UI<'a, (), R, E, S> {
    pub fn static_ui(self) -> Self {
        self
    }
}

impl<'a, Message, R: Renderer, E: Event> UI<'a, Message, R, E, Theme<R::Color>>
where
    <R as Renderer>::Color: PaletteColor + 'static,
{
    pub fn theme(mut self, theme: Theme<R::Color>) -> Self {
        self.styler = theme;
        self
    }
}

macro_rules! renderer_colors {
    ($($method:ident : $color_ty:ty),*) => {
        $(
            impl<'a, Message, R, E> UI<'a, Message, R, E>
            where
                R: Renderer<Color = $color_ty>,
                E: Event,
            {
                pub fn $method(self) -> Self {
                    self
                }
            }
        )*
    };
}

renderer_colors! {
    monochrome: BinaryColor,
    rgb555: Rgb555,
    rgb565: Rgb565,
    rgb666: Rgb666,
    rgb888: Rgb888
}
