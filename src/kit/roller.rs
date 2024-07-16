// use alloc::{boxed::Box, vec::Vec};

// use crate::{
//     el::{El, ElId},
//     event::Event,
//     palette::PaletteColor,
//     render::Renderer,
//     size::{Length, Size},
//     style::component_style,
//     theme::Theme,
// };

// pub struct RollerStatus {}

// pub fn primary<C: PaletteColor>(theme: &Theme<C>, status: RollerStatus) ->
// RollerStyle<C> {     let palette = theme.palette();
//     let base = RollerStyle::new(&palette).background(palette.background);

//     // TODO

//     base
// }

// component_style! {
//     pub RollerStyle: RollerStyler(RollerStatus) default {primary} {
//         background: background,
//         border: border,
//     }
// }

// pub struct Roller<'a, Message, R, E, S>
// where
//     R: Renderer,
//     E: Event,
//     S: RollerStyler<R::Color>,
// {
//     id: ElId,
//     size: Size<Length>,
//     items: Vec<El<'a, Message, R, E, S>>,
//     on_change: Box<dyn Fn(usize) -> Message + 'a>,
//     class: S::Class<'a>,
// }

// impl<'a, Message, R, E, S> Roller<'a, Message, R, E, S>
// where
//     R: Renderer,
//     E: Event,
//     S: RollerStyler<R::Color>,
// {
//     pub fn new(options: impl Iterator<Item = impl Into>) -> Self {
//         Self { id, size, items, on_change, class }
//     }
// }
