// use crate::{
//     el::ElId,
//     event::Event,
//     layout::{Layout, Viewport},
//     render::Renderer,
//     size::{Length, Size},
//     widget::Widget,
// };

// #[derive(Clone, Copy)]
// enum KeyAction {
//     CaseToggle,
//     Enter,
//     Char(char),
// }

// macro_rules! keyboard_layout {
//     (@key $key: ident) => {
//         KeyAction::$key
//     };

//     (@key $key: literal) => {
//         KeyAction::Char($key)
//     };

//     ($([$($key: tt),* $(,)?]),* $(,)?) => [
//         &[
//             $(
//                 &[$(keyboard_layout!(@key $key)),*]
//             ),*
//         ]
//     ];
// }

// const KEYBOARD_LAYOUT: &[&[KeyAction]] = keyboard_layout![
//     ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'],
//     ['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p'],
//     ['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', ';'],
//     ['z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '/'],
// ];

// pub struct VirtualKeyboard<'a, Message> {
//     id: ElId,
//     size: Size<Length>,
//     on_char: Box<dyn Fn(char) -> Message + 'a>,
// }

// impl<'a, Message> VirtualKeyboard<'a, Message> {
//     pub fn new<F>(on_char: F) -> Self
//     where
//         F: Fn(char) -> Message + 'a,
//     {
//         Self { id: ElId::unique(), size: Size::fill(), on_char:
// Box::new(on_char) }     }
// }

// impl<'a, Message, R, E, S> Widget<Message, R, E, S> for VirtualKeyboard<'a,
// Message> where
//     R: Renderer,
//     E: Event,
// {
//     fn id(&self) -> Option<crate::el::ElId> {
//         Some(self.id)
//     }

//     fn tree_ids(&self) -> Vec<crate::el::ElId> {
//         vec![]
//     }

//     fn size(&self, viewport: &Viewport) -> Size<Length> {
//         Size::fill()
//     }

//     fn position(&self) -> crate::layout::Position {
//         crate::layout::Position::Absolute
//     }

//     fn layout(
//         &self,
//         ctx: &mut crate::ui::UiCtx<Message>,
//         state: &mut crate::state::StateNode,
//         styler: &S,
//         limits: &crate::layout::Limits,
//         viewport: &Viewport,
//     ) -> crate::layout::LayoutNode {
//         Layout::flex(ctx, state, styler, axis, limits, size, position,
// viewport, padding, gap, align, children)     }

//     fn draw(
//         &self,
//         ctx: &mut crate::ui::UiCtx<Message>,
//         state: &mut crate::state::StateNode,
//         renderer: &mut R,
//         styler: &S,
//         layout: crate::layout::Layout,
//     ) {
//         todo!()
//     }
// }
