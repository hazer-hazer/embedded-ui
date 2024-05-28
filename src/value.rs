use core::{
    cell::{Ref, RefCell, RefMut},
    ops::Deref as _,
};

use alloc::rc::Rc;

use crate::{el::El, event::Event, render::Renderer, widget::Widget};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Value<T> {
    Static(RefCell<T>),
    Dynamic(Rc<RefCell<T>>),
}

impl<T> Value<T> {
    pub fn new(value: T) -> Self {
        Self::Static(RefCell::new(value))
    }

    pub fn dynamic(value: T) -> Self {
        Self::Dynamic(Rc::new(RefCell::new(value)))
    }

    pub fn get(&self) -> Ref<'_, T> {
        match self {
            Value::Static(value) => value.borrow(),
            Value::Dynamic(value) => value.borrow(),
        }
    }

    pub fn get_mut(&mut self) -> RefMut<'_, T> {
        match self {
            Value::Static(value) => value.borrow_mut(),
            Value::Dynamic(value) => value.borrow_mut(),
        }
    }
}

impl<T> From<T> for Value<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T, Message, R, E, S> Widget<Message, R, E, S> for Value<T>
where
    R: Renderer,
    E: Event,
    T: Widget<Message, R, E, S>,
{
    fn id(&self) -> Option<crate::el::ElId> {
        self.get().id()
    }

    fn tree_ids(&self) -> Vec<crate::el::ElId> {
        self.get().tree_ids()
    }

    fn size(&self) -> crate::size::Size<crate::size::Length> {
        self.get().size()
    }

    fn layout(
        &self,
        ctx: &mut crate::ui::UiCtx<Message>,
        state: &mut crate::state::StateNode,
        styler: &S,
        limits: &crate::layout::Limits,
    ) -> crate::layout::LayoutNode {
        self.get().layout(ctx, state, styler, limits)
    }

    fn draw(
        &self,
        ctx: &mut crate::ui::UiCtx<Message>,
        state: &mut crate::state::StateNode,
        renderer: &mut R,
        styler: &S,
        layout: crate::layout::Layout,
    ) {
        self.get().draw(ctx, state, renderer, styler, layout)
    }
}

impl<'a, T, Message, R, E, S> From<Value<T>> for El<'a, Message, R, E, S>
where
    T: Widget<Message, R, E, S> + 'a,
    Message: Clone + 'a,
    R: Renderer + 'a,
    E: Event + 'a,
{
    fn from(value: Value<T>) -> Self {
        El::new(value)
    }
}
