use alloc::{boxed::Box, vec::Vec};
use core::{
    any::{Any, TypeId},
    borrow::Borrow,
};

use crate::{color::UiColor, event::Event, render::Renderer, widget::Widget};

// TODO: Not just any, but specific "State" trait with Default, etc.??? Is it
// possible

pub enum State {
    None,
    Some(Box<dyn Any>),
}

impl State {
    pub fn new<T: 'static>(state: T) -> Self {
        Self::Some(Box::new(state))
    }

    pub fn downcast_ref<T: 'static>(&self) -> &T {
        match self {
            State::None => panic!("Downcast of stateless state"),
            State::Some(state) => state.downcast_ref().expect("Downcast state"),
        }
    }

    pub fn downcast_mut<T: 'static>(&mut self) -> &mut T {
        match self {
            State::None => panic!("Downcast of stateless state"),
            State::Some(state) => state.downcast_mut().expect("Downcast mut state"),
        }
    }

    pub fn reset<T: 'static>(&mut self)
    where
        T: Default,
    {
        match self {
            State::None => panic!("Reset of stateless state"),
            State::Some(state) => {
                *state.downcast_mut::<T>().expect("Downcast reset state") = T::default();
            },
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct StateTag(pub TypeId);

impl StateTag {
    pub fn stateless() -> Self {
        Self::of::<()>()
    }

    pub fn of<T: 'static>() -> Self {
        Self(TypeId::of::<T>())
    }
}

pub struct StateNode {
    pub tag: StateTag,
    pub state: State,
    pub children: Vec<StateNode>,
}

impl StateNode {
    pub fn stateless() -> Self {
        Self { tag: StateTag::stateless(), state: State::None, children: vec![] }
    }

    pub fn new<'a, Message, C: UiColor, E: Event, S>(
        widget: impl Borrow<dyn Widget<Message, C, E, S> + 'a>,
    ) -> Self {
        let widget = widget.borrow();

        Self { tag: widget.state_tag(), state: widget.state(), children: widget.state_children() }
    }

    pub fn get<T: 'static>(&self) -> &T {
        self.state.downcast_ref()
    }

    pub fn get_mut<T: 'static>(&mut self) -> &mut T {
        self.state.downcast_mut()
    }

    pub fn reset<T: 'static>(&mut self)
    where
        T: Default,
    {
        self.state.reset::<T>()
    }
}
