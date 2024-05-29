// use core::ops::Range;

// use embedded_text::TextBox;

// use crate::{
//     el::ElId,
//     event::{Capture, CommonEvent, Event, Propagate},
//     layout::Layout,
//     render::Renderer,
//     size::{Length, Size},
//     state::{State, StateTag},
//     style::component_style,
//     ui::UiCtx,
//     value::Value,
//     widget::Widget,
// };

// #[derive(Clone, Copy)]
// struct FocusInputState {
//     is_active: bool,
//     is_pressed: bool,
// }

// impl Default for FocusInputState {
//     fn default() -> Self {
//         Self { is_active: false, is_pressed: false }
//     }
// }

// // const ALPHABET: &[char] = &[

// // ];

// #[derive(Clone, Copy)]
// pub enum FocusInputStatus {
//     Normal,
//     Focused,
//     Pressed,
//     Active,
// }

// component_style! {
//     pub FocusInputStyle: FocusInputStyler(FocusInputStatus) {
//         background: background,
//         border: border,
//     }
// }

// pub struct FocusInput<'a, Message, R, S>
// where
//     R: Renderer,
//     S: FocusInputStyler<R::Color>,
// {
//     id: ElId,
//     size: Size<Length>,
//     // length: usize,
//     value: Value<String>,
//     position: usize,
//     on_change: Option<Box<dyn Fn(&str) -> Message + 'a>>,
//     class: S::Class<'a>,
// }

// impl<'a, Message, R, S> FocusInput<'a, Message, R, S>
// where
//     R: Renderer,
//     S: FocusInputStyler<R::Color>,
// {
//     pub fn new(value: Value<String>) -> Self {
//         Self {
//             id: ElId::unique(),
//             size: Size::fill(),
//             // length: 128,
//             value,
//             position: 0,
//             on_change: None,
//             class: S::default(),
//         }
//     }

//     // pub fn length(mut self, length: usize) -> Self {
//     //     self.length = length;
//     //     self
//     // }

//     pub fn width(mut self, width: impl Into<Length>) -> Self {
//         self.size.width = width.into();
//         self
//     }

//     pub fn height(mut self, height: impl Into<Length>) -> Self {
//         self.size.height = height.into();
//         self
//     }

//     // Helpers //
//     fn status<E: Event>(&self, ctx: &UiCtx<Message>, state: &FocusInputState) -> FocusInputStatus {
//         match (UiCtx::is_focused::<R, E, S>(&ctx, self), state) {
//             (_, FocusInputState { is_active: true, .. }) => FocusInputStatus::Active,
//             (_, FocusInputState { is_pressed: true, .. }) => FocusInputStatus::Pressed,
//             (true, FocusInputState { is_active: false, is_pressed: false }) => {
//                 FocusInputStatus::Focused
//             },
//             (false, FocusInputState { is_active: false, is_pressed: false }) => {
//                 FocusInputStatus::Normal
//             },
//         }
//     }
// }

// impl<'a, Message, R, E, S> Widget<Message, R, E, S> for FocusInput<'a, Message, R, S>
// where
//     R: Renderer,
//     E: Event,
//     S: FocusInputStyler<R::Color>,
// {
//     fn id(&self) -> Option<ElId> {
//         Some(self.id)
//     }

//     fn tree_ids(&self) -> Vec<ElId> {
//         vec![self.id]
//     }

//     fn size(&self) -> Size<Length> {
//         self.size
//     }

//     fn state_tag(&self) -> crate::state::StateTag {
//         StateTag::of::<FocusInputState>()
//     }

//     fn state(&self) -> crate::state::State {
//         State::new(FocusInputState::default())
//     }

//     fn state_children(&self) -> Vec<crate::state::StateNode> {
//         vec![]
//     }

//     fn on_event(
//         &mut self,
//         ctx: &mut UiCtx<Message>,
//         event: E,
//         state: &mut crate::state::StateNode,
//     ) -> crate::event::EventResponse<E> {
//         let focused = ctx.is_focused::<R, E, S>(self);
//         let current_state = *state.get::<FocusInputState>();

//         if current_state.is_active {
//             if let Some(offset) = event.as_input_letter_scroll() {
//                 if self.position >= self.value.get().len() {
//                     // self.value.get_mut().
//                 }

//                 let prev_char = self.value.get().chars().nth(self.position).unwrap_or(' ');

//                 const CHAR_ASCII_RANGE: Range<u8> = 32..127;
//                 const ALPHABET_SIZE: i32 =
//                     CHAR_ASCII_RANGE.end as i32 - CHAR_ASCII_RANGE.start as i32;

//                 let new_char = ((prev_char as i32 + offset % ALPHABET_SIZE + ALPHABET_SIZE)
//                     % ALPHABET_SIZE) as u8 as char;

//                 self.value.get_mut()[self.position] = new_char;

//                 if prev_char != new_char {
//                     if let Some(on_change) = self.on_change.as_ref() {
//                         ctx.publish((on_change)(&String::from_iter(self.value.get().iter())))
//                     }
//                 }

//                 return Capture::Captured.into();
//             }
//         }

//         if let Some(common) = event.as_common() {
//             match common {
//                 CommonEvent::FocusMove(_) if focused => {
//                     return Propagate::BubbleUp(self.id, event).into()
//                 },
//                 CommonEvent::FocusClickDown if focused => {
//                     state.get_mut::<FocusInputState>().is_pressed = true;
//                     return Capture::Captured.into();
//                 },
//                 CommonEvent::FocusClickUp if focused => {
//                     state.get_mut::<FocusInputState>().is_pressed = false;

//                     if current_state.is_pressed {
//                         state.get_mut::<FocusInputState>().is_active =
//                             !state.get::<FocusInputState>().is_active;

//                         return Capture::Captured.into();
//                     }
//                 },
//                 CommonEvent::FocusClickDown
//                 | CommonEvent::FocusClickUp
//                 | CommonEvent::FocusMove(_) => {
//                     // Should we reset state on any event? Or only on common
//                     state.reset::<FocusInputState>();
//                 },
//             }
//         }

//         Propagate::Ignored.into()
//     }

//     fn layout(
//         &self,
//         ctx: &mut UiCtx<Message>,
//         state: &mut crate::state::StateNode,
//         styler: &S,
//         limits: &crate::layout::Limits,
//     ) -> crate::layout::LayoutNode {
//         Layout::sized(limits, self.size, |limits| {
//             limits.resolve_size(self.size.width, self.size.height, Size::zero())
//         })
//     }

//     fn draw(
//         &self,
//         ctx: &mut UiCtx<Message>,
//         state: &mut crate::state::StateNode,
//         renderer: &mut R,
//         styler: &S,
//         layout: crate::layout::Layout,
//     ) {
//         let state = state.get::<FocusInputState>();
//         let style = styler.style(&self.class, self.status::<E>(ctx, state));

//         let bounds = layout.bounds();

//         renderer.block(&crate::block::Block {
//             border: style.border,
//             rect: bounds.into(),
//             background: style.background,
//         });

//         renderer.text(TextBox::new(sel, bounds, character_style))
//     }
// }
