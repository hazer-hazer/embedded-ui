use crate::{el::El, event::Event, kit::icon::Icon, render::Renderer, size::Size};

use super::{IconData, IntoIcon};

pub mod icons5x7;

pub struct MonoIcon<const WIDTH: u32, const HEIGHT: u32> {
    data: IconData,
}

impl<R: Renderer, const WIDTH: u32, const HEIGHT: u32> IntoIcon<R> for MonoIcon<WIDTH, HEIGHT> {
    fn into_icon<'a>(self) -> crate::kit::icon::Icon<'a, R> {
        Icon::new(Size::new(WIDTH, HEIGHT), &self.data)
    }
}

impl<'a, Message, R, E, S, const WIDTH: u32, const HEIGHT: u32> Into<El<'a, Message, R, E, S>>
    for MonoIcon<WIDTH, HEIGHT>
where
    Message: 'a,
    R: Renderer + 'a,
    E: Event + 'a,
    S: 'a,
{
    fn into(self) -> El<'a, Message, R, E, S> {
        El::new(self.into_icon())
    }
}

pub trait MonoIconSet {
    const SIZE: Size;
}
