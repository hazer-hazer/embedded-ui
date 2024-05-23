#![no_std]

pub mod align;
pub mod block;
pub mod color;
pub mod el;
pub mod event;
pub mod font;
pub mod helpers;
pub mod kit;
pub mod layout;
pub mod linear;
pub mod padding;
pub mod render;
pub mod size;
pub mod state;
pub mod style;
pub mod text;
pub mod ui;
pub mod widget;
pub mod focus;
pub mod icons;

// TODO: Feature to switch to fixed-sized heapless
#[macro_use]
extern crate alloc;
