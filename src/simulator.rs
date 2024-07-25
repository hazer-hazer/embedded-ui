pub mod single_encoder {
    use crate::event::CommonEvent;
    use embedded_graphics_simulator::sdl2::{self, Keycode, MouseButton};

    #[derive(Clone, Copy, Debug)]
    pub enum Event {
        EncoderRotation(i32),
        EncoderButtonDown,
        EncoderButtonUp,
        Exit,
    }

    impl From<CommonEvent> for Event {
        fn from(value: CommonEvent) -> Self {
            match value {
                CommonEvent::FocusMove(offset) => Self::EncoderRotation(offset),
                CommonEvent::FocusButtonDown => Self::EncoderButtonDown,
                CommonEvent::FocusButtonUp => Self::EncoderButtonUp,
                CommonEvent::Exit => Self::Exit,
            }
        }
    }

    impl TryFrom<embedded_graphics_simulator::SimulatorEvent> for Event {
        type Error = ();

        fn try_from(
            value: embedded_graphics_simulator::SimulatorEvent,
        ) -> Result<Self, Self::Error> {
            match value {
                embedded_graphics_simulator::SimulatorEvent::KeyDown { keycode, .. } => {
                    match keycode {
                        Keycode::Space | Keycode::Return | Keycode::Return2 => {
                            Ok(Event::EncoderButtonDown)
                        },
                        _ => Err(()),
                    }
                },
                embedded_graphics_simulator::SimulatorEvent::KeyUp { keycode, .. } => match keycode
                {
                    Keycode::Left | Keycode::Up => Ok(Event::EncoderRotation(-1)),
                    Keycode::Right | Keycode::Down => Ok(Event::EncoderRotation(1)),
                    Keycode::Space | Keycode::Return | Keycode::Return2 => {
                        Ok(Event::EncoderButtonUp)
                    },
                    _ => Err(()),
                },
                embedded_graphics_simulator::SimulatorEvent::MouseWheel {
                    scroll_delta,
                    direction,
                } => {
                    let dir = match direction {
                        sdl2::MouseWheelDirection::Normal => 1,
                        sdl2::MouseWheelDirection::Flipped => -1,
                        sdl2::MouseWheelDirection::Unknown(_) => {
                            panic!("Unknown mouse direction is not supported")
                        },
                    };

                    let offset = scroll_delta.y * dir;

                    Ok(Event::EncoderRotation(offset))
                },
                embedded_graphics_simulator::SimulatorEvent::MouseButtonDown {
                    mouse_btn, ..
                } if mouse_btn == MouseButton::Middle || mouse_btn == MouseButton::Left => {
                    Ok(Event::EncoderButtonDown)
                },
                embedded_graphics_simulator::SimulatorEvent::MouseButtonUp {
                    mouse_btn, ..
                } if mouse_btn == MouseButton::Middle || mouse_btn == MouseButton::Left => {
                    Ok(Event::EncoderButtonUp)
                },
                embedded_graphics_simulator::SimulatorEvent::Quit => Ok(Event::Exit),
                _ => Err(()),
            }
        }
    }

    impl crate::event::Event for Event {
        fn as_common(&self) -> Option<CommonEvent> {
            match self {
                Event::EncoderRotation(offset) => Some(CommonEvent::FocusMove(*offset)),
                Event::EncoderButtonDown => Some(CommonEvent::FocusButtonDown),
                Event::EncoderButtonUp => Some(CommonEvent::FocusButtonUp),
                Event::Exit => Some(CommonEvent::Exit),
            }
        }

        fn as_select_shift(&self) -> Option<i32> {
            match self {
                &Event::EncoderRotation(offset) => Some(offset),
                _ => None,
            }
        }

        fn as_slider_shift(&self) -> Option<i32> {
            match self {
                &Event::EncoderRotation(offset) => Some(offset),
                _ => None,
            }
        }

        fn as_knob_rotation(&self) -> Option<i32> {
            match self {
                &Event::EncoderRotation(offset) => Some(offset),
                _ => None,
            }
        }

        fn as_input_letter_scroll(&self) -> Option<i32> {
            match self {
                &Event::EncoderRotation(offset) => Some(offset),
                _ => None,
            }
        }

        fn as_scroll_offset(&self) -> Option<i32> {
            match self {
                &Event::EncoderRotation(offset) => Some(offset * 5),
                _ => None,
            }
        }
    }
}
