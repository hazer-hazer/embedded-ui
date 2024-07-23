// use core::marker::PhantomData;

// use crate::{action::Action, el::El, event::Event, render::Renderer};

// pub trait App<'a> {
//     type State;
//     type Message;
//     type C: UiColor;
//     type E: Event;
//     type S;

//     fn update(&self, state: &mut Self::State, message: Self::Message) -> impl
// Into<Action>;

//     fn view(
//         &self,
//         state: &'a Self::State,
//     ) -> impl Into<El<'a, Self::Message, Self::R, Self::E, Self::S>>;
// }

// pub fn app<'a, State, Message, C: UiColor, E: Event, S>(
//     update: impl Fn(&mut State, Message) -> Action,
//     view: impl Fn(&State),
// ) -> impl App<'a> {
//     struct Instance<State, Message, C: UiColor, E: Event, S, U, V> {
//         update: U,
//         view: V,
//         _state: PhantomData<State>,
//         _message: PhantomData<Message>,
//         _r: PhantomData<R>,
//         _e: PhantomData<E>,
//         _s: PhantomData<S>,
//     }

//     impl<'a, State, Message, C, E, S, U, V> App<'a> for Instance<State,
// Message, C, E, S, U, V>     where
//         //         E: Event,
//         U: Fn(&mut State, Message),
//         V: Fn(&State) -> Into<El<'a, Self::Message, Self::R, Self::E,
// Self::S>>,     {
//         type State = State;
//         type Message = Message;
//         type R = R;
//         type E = E;
//         type S = S;

//         fn update(&self, state: &mut Self::State, message: Self::Message) ->
// impl Into<Action> {             (self.update)(state, message)
//         }

//         fn view(
//             &self,
//             state: &'a Self::State,
//         ) -> impl Into<El<'a, Self::Message, Self::R, Self::E, Self::S>> {
//             (self.view)(state)
//         }
//     }

//     Instance {
//         update,
//         view,
//         _state: PhantomData,
//         _message: PhantomData,
//         _r: PhantomData,
//         _e: PhantomData,
//         _s: PhantomData,
//     }
// }
