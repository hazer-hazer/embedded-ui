use core::{marker::PhantomData, ops::ControlFlow};

use alloc::vec::Vec;

use crate::el::ElId;

#[derive(Clone)]
pub enum Capture {
    /// Event is captured by element and should not be accepted by its parents
    Captured,
}

impl<E: Event> Into<EventResponse<E>> for Capture {
    #[inline]
    fn into(self) -> EventResponse<E> {
        EventResponse::Break(self)
    }
}

#[derive(Clone)]
pub enum Propagate<E: Event> {
    /// Event is ignored by element and can be accepted by parents
    Ignored,
    /// Event is accepted by element and does not belongs to it logic but its parent.
    /// For example FocusMove on focused button is captured by button but bubbles up to its container which already moves the focus to next children. Check source of Linear container as an example of how to handle bubble up and why it doesn't need to store any state or identifier of element started the bubble up.
    BubbleUp(ElId, E),
}

impl<E: Event> Into<EventResponse<E>> for Propagate<E> {
    #[inline]
    fn into(self) -> EventResponse<E> {
        EventResponse::Continue(self)
    }
}

pub type EventResponse<E> = ControlFlow<Capture, Propagate<E>>;

#[derive(Clone, Copy)]
pub enum CommonEvent {
    /// Moves focus to current ±offset
    FocusMove(i32),
    /// Moves focus starting from back (internal usage only)
    // FocusMoveRev(i32),
    /// Focus click button (e.g. enter key) is down
    FocusClickDown,
    // Focus click button is up
    FocusClickUp,
}

// impl CommonEvent {
//     pub fn bubble_up<E: Event>(self) -> EventResponse<E> {
//         Propagate::BubbleUp(self).into()
//     }
// }

impl Event for CommonEvent {
    fn as_common(&self) -> Option<CommonEvent> {
        Some(*self)
    }
}

pub trait Event: Clone + From<CommonEvent> {
    // fn is_focus_move(&self) -> Option<i32>;

    // fn is_focus_click(&self) -> bool;

    fn as_common(&self) -> Option<CommonEvent>;
}

#[derive(Clone)]
pub struct EventStub;

impl Event for EventStub {
    fn as_common(&self) -> Option<CommonEvent> {
        None
    }
}

impl From<CommonEvent> for EventStub {
    fn from(_: CommonEvent) -> Self {
        Self
    }
}

// pub struct EventHandler<E: Event, H: FnOnce(E) -> EventResponse> {
//     handler: H,
//     // TODO: Can I get rid of this marker?
//     marker: PhantomData<E>,
// }

// impl<E: Event, H> From<H> for EventHandler<E, H>
// where
//     H: FnOnce(E) -> EventResponse,
// {
//     fn from(value: H) -> Self {
//         EventHandler {
//             handler: value,
//             marker: PhantomData,
//         }
//     }
// }

pub trait Controls<E: Event> {
    // TODO: Pass state to event collector of platform. Is should include:
    //  - Focus target (widget id). For example, encoder click in common case is FocusClick, but on other page its logic differs
    fn events(&mut self) -> Vec<E>;
}

impl<F, E: Event> Controls<E> for F
where
    F: FnMut() -> Vec<E>,
{
    fn events(&mut self) -> Vec<E> {
        self()
    }
}

pub struct NullControls<E: Event> {
    marker: PhantomData<E>,
}

impl<E: Event> Controls<E> for NullControls<E> {
    fn events(&mut self) -> Vec<E> {
        vec![]
    }
}

impl<E: Event> Default for NullControls<E> {
    fn default() -> Self {
        Self { marker: PhantomData }
    }
}
