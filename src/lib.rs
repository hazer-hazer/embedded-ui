#![cfg_attr(not(feature = "std"), no_std)]

pub mod align;
pub mod block;
pub mod color;
pub mod el;
pub mod event;
pub mod focus;
pub mod font;
pub mod helpers;
pub mod icons;
pub mod kit;
pub mod layout;
mod log;
pub mod padding;
pub mod render;
pub mod size;
pub mod state;
pub mod style;
pub mod text;
pub mod ui;
pub mod widget;
pub mod app;
pub mod action;
pub mod lazy;
pub mod value;

// TODO: Feature to switch to fixed-sized heapless
#[macro_use]
extern crate alloc;

#[cfg(not(feature = "no_std"))]
extern crate std;
