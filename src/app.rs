use core::marker::PhantomData;

use crate::{
    el::El,
    event::{Controls, Event, NullControls},
    render::Renderer,
    state::State,
    style::Styler,
    ui::UI,
};

pub trait Update<State, Message> {
    fn update(&self, state: &mut State, message: Message);
}

pub trait View<'a, State, Message, R: Renderer, E: Event, S> {
    fn view(&self, state: &'a State) -> impl Into<El<'a, Message, R, E, S>>;
}

pub trait App:
    Update<Self::State, Self::Message>
    + for<'a> View<'a, Self::State, Self::Message, Self::R, Self::E, Self::S>
{
    type State;
    type Message;
    type R: Renderer;
    type E: Event;
    type S;
}

pub fn app<State, Message, R: Renderer, E: Event, S>(
    update: impl Update<State, Message>,
    view: impl for<'a> View<'a, State, Message, R, E, S>,
) -> impl App {
    struct Instance<U, V> {
        update: U,
        view: V,
    }

    // struct Instance<
    //     State,
    //     Message,
    //     R: Renderer,
    //     E: Event,
    //     S,
    //     U: Update<State, Message>,
    //     V: for<'a> View<'a, State, Message, R, E, S>,
    // > {
    //     update: U,
    //     view: V,
    //     _state: PhantomData<State>,
    //     _message: PhantomData<Message>,
    //     _r: PhantomData<R>,
    //     _e: PhantomData<E>,
    //     _s: PhantomData<S>,
    // }

    // impl<
    //         State,
    //         Message,
    //         R: Renderer,
    //         E: Event,
    //         S,
    //         U: Update<State, Message>,
    //         V: for<'a> View<'a, State, Message, R, E, S>,
    //     > Update<State, Message> for Instance<State, Message, R, E, S, U, V>
    // {
    //     #[inline]
    //     fn update(&self, state: &mut State, message: Message) {
    //         self.update.update(state, message)
    //     }
    // }

    // impl<
    //         'a,
    //         State,
    //         Message,
    //         R: Renderer,
    //         E: Event,
    //         S,
    //         U: Update<State, Message>,
    //         V: View<'a, State, Message, R, E, S>,
    //     > View<'a, State, Message, R, E, S> for Instance<State, Message, R, E, S, U, V>
    // {
    //     fn view(&'a self, state: &'a State) -> impl Into<El<'a, Message, R, E, S>> {
    //         self.view.view(state)
    //     }
    // }

    // impl<
    //         State,
    //         Message,
    //         R: Renderer,
    //         E: Event,
    //         S,
    //         U: Update<State, Message>,
    //         V: for<'a> View<'a, State, Message, R, E, S>,
    //     > App for Instance<State, Message, R, E, S, U, V>
    // {
    //     type State = State;

    //     type Message = Message;

    //     type R = R;

    //     type E = E;

    //     type S = S;
    // }

    Instance { update, view }
}
