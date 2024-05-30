use alloc::{collections::VecDeque, vec::Vec};
use embedded_graphics::pixelcolor::BinaryColor;

use crate::{
    el::{El, ElId},
    event::{Controls, Event, EventStub, NullControls, Propagate},
    layout::{Layout, LayoutNode, Limits},
    render::Renderer,
    size::Size,
    state::StateNode,
    style::{monochrome::Monochrome, Styler},
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

pub struct UI<'a, Message, R: Renderer, E: Event, S: Styler<R::Color>> {
    root: El<'a, Message, R, E, S>,
    root_node: LayoutNode,
    // viewport_size: Size,
    root_state: StateNode,
    overlay_node: LayoutNode,
    styler: S,
    // events: Vec<E>,
    ctx: UiCtx<Message>,
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
        );

        let overlay_node = root.overlay().layout(
            &mut ctx,
            &mut root_state,
            &Default::default(),
            &Limits::only_max(viewport_size),
        );

        Self {
            root,
            root_node,
            // viewport_size,
            root_state,
            overlay_node,
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

    pub fn tick(&mut self, events: &[E]) {
        for event in events {
            let result =
                self.root.overlay().on_event(&mut self.ctx, event.clone(), &mut self.root_state);

            let result = if let core::ops::ControlFlow::Continue(Propagate::Ignored) = result {
                self.root.on_event(&mut self.ctx, event.clone(), &mut self.root_state)
            } else {
                result
            };

            if let core::ops::ControlFlow::Continue(propagate) = result {
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

    pub fn draw(&mut self, renderer: &mut R) {
        // FIXME: Performance?
        // TODO: Maybe should clear only root bounds
        renderer.clear();

        self.root.draw(
            &mut self.ctx,
            &mut self.root_state,
            renderer,
            &self.styler,
            Layout::new(&self.root_node),
        );

        self.root.overlay().draw(
            &mut self.ctx,
            &mut self.root_state,
            renderer,
            &self.styler,
            Layout::new(&self.overlay_node),
        );
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

impl<'a, Message, R, E> UI<'a, Message, R, E, Monochrome>
where
    R: Renderer<Color = BinaryColor>,
    E: Event,
{
    pub fn monochrome(self) -> Self {
        self
    }
}
