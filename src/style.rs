pub mod oled;

use crate::{color::UiColor, kit::button::ButtonStyler};

pub trait Styler<C: UiColor>: ButtonStyler<C> + Default {}
