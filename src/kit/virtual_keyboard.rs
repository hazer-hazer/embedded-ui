use crate::{
    el::ElId,
    event::Event,
    render::Renderer,
    size::{Length, Size},
    widget::Widget,
};

// macro_rules! keys {
//     ($($key: ident: $char: expr),* $(,)?) => {
//         pub enum Key {
//             $($key),*
//         }

//         impl Key {
//             pub fn char(&self) -> char {
//                 match self {
//                     $(Self::$key => $char),*
//                 }
//             }
//         }
//     };
// }

pub struct VirtualKeyboard<'a, Message> {
    id: ElId,
    size: Size<Length>,
    on_char: Box<dyn Fn(char) -> Message + 'a>,
}

impl<'a, Message> VirtualKeyboard<'a, Message> {
    pub fn new<F>(on_char: F) -> Self
    where
        F: Fn(char) -> Message + 'a,
    {
        Self { id: ElId::unique(), size: Size::fill(), on_char: Box::new(on_char) }
    }
}

impl<'a, Message, R, E, S> Widget<Message, R, E, S> for VirtualKeyboard<'a, Message>
where
    R: Renderer,
    E: Event,
{
    fn id(&self) -> Option<crate::el::ElId> {
        Some(self.id)
    }

    fn tree_ids(&self) -> Vec<crate::el::ElId> {
        vec![]
    }

    fn size(&self) -> Size<Length> {
        self.size
    }

    fn layout(
        &self,
        ctx: &mut crate::ui::UiCtx<Message>,
        state: &mut crate::state::StateNode,
        styler: &S,
        limits: &crate::layout::Limits,
    ) -> crate::layout::LayoutNode {
        todo!()
    }

    fn draw(
        &self,
        ctx: &mut crate::ui::UiCtx<Message>,
        state: &mut crate::state::StateNode,
        renderer: &mut R,
        styler: &S,
        layout: crate::layout::Layout,
    ) {
        todo!()
    }
}
