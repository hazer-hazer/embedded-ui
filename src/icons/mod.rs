use crate::{kit::icon::Icon, render::Renderer};

pub mod mono;

pub enum InternalIcon {
    ArrowLeft,
    ArrowRight,
}

pub type IconData = &'static [u8];

pub trait IntoIcon<R: Renderer> {
    fn into_icon<'a>(self) -> Icon<'a, R>;
}

pub trait InternalIconSet<R: Renderer> {
    fn internal(icon: InternalIcon) -> impl IntoIcon<R>;
}
