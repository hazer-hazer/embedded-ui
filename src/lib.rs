#![cfg_attr(not(feature = "std"), no_std)]

pub mod action;
pub mod align;
pub mod app;
pub mod axis;
pub mod block;
pub mod color;
pub mod el;
pub mod event;
pub mod focus;
pub mod font;
pub mod helpers;
pub mod icons;
mod layer;
pub mod layout;
mod log;
pub mod padding;
pub mod palette;
pub mod render;
pub mod size;
pub mod state;
pub mod style;
pub mod theme;
pub mod ui;
pub mod widget;
pub mod widgets;

// TODO: Feature to switch to fixed-sized heapless
#[macro_use]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;
