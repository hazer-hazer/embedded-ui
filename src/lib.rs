#![cfg_attr(not(feature = "std"), no_std)]

pub mod action;
pub mod align;
pub mod app;
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
pub mod lazy;
mod log;
pub mod padding;
pub mod render;
pub mod size;
pub mod state;
pub mod style;
pub mod ui;
pub mod value;
pub mod widget;
pub mod overlay;

// TODO: Feature to switch to fixed-sized heapless
#[macro_use]
extern crate alloc;

#[cfg(not(feature = "no_std"))]
extern crate std;
