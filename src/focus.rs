// use crate::{
//     el::{El, ElId},
//     event::{Event, EventResponse},
//     render::Renderer,
//     ui::UiCtx,
// };

// #[derive(Clone, Copy)]
// enum FocusResult {
//     Child(ElId),
//     Outside(i32),
// }

// pub fn for_container<'a, Message, R: Renderer, E: Event, S>(
//     ctx: &mut UiCtx<Message>,
//     children: &[El<'a, Message, R, E, S>],
// ) -> EventResponse<E> {
//     let new_focus_index = child_index as i32 + focus_offset;

//     if new_focus_index < 0 {
//         return FocusResult::Outside(new_focus_index);
//     }

//     let new_focused_child =
//         children.iter().filter_map(|child| child.id()).nth(new_focus_index as usize);

//     if let Some(new_focused_child) = new_focused_child {
//         FocusResult::Child(new_focused_child)
//     } else {
//         FocusResult::Outside(new_focus_index)
//     }
// }
