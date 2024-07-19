use std::{process::exit, thread, time::Duration};

use embedded_graphics::{
    geometry::{Dimensions, Point, Size},
    pixelcolor::{Rgb888, RgbColor},
    primitives::{
        CornerRadii, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, RoundedRectangle,
        StyledDrawable,
    },
};
use embedded_graphics_simulator::{
    sdl2::{self, Keycode, MouseButton},
    OutputSettingsBuilder, SimulatorDisplay, Window,
};
use embedded_ui::{
    col,
    el::ElId,
    event::CommonEvent,
    helpers::{bar_h, button, checkbox, select_h, select_v, text},
    icons::IconKind,
    kit::knob::Knob,
    row,
    ui::UI,
    value::Value,
};
use embedded_ui::{helpers::bar_v, theme::Theme};

#[derive(Clone, Copy, Debug)]
enum Event {
    MainEncoderRotation(i32),
    MainEncoderButtonDown,
    MainEncoderButtonUp,
}

impl From<CommonEvent> for Event {
    fn from(value: CommonEvent) -> Self {
        match value {
            CommonEvent::FocusMove(offset) => Self::MainEncoderRotation(offset),
            CommonEvent::FocusClickDown => Self::MainEncoderButtonDown,
            CommonEvent::FocusClickUp => Self::MainEncoderButtonUp,
        }
    }
}

impl TryFrom<embedded_graphics_simulator::SimulatorEvent> for Event {
    type Error = ();

    fn try_from(value: embedded_graphics_simulator::SimulatorEvent) -> Result<Self, Self::Error> {
        match value {
            embedded_graphics_simulator::SimulatorEvent::KeyDown { keycode, .. } => match keycode {
                Keycode::Space | Keycode::Return | Keycode::Return2 => {
                    Ok(Event::MainEncoderButtonDown)
                },
                _ => Err(()),
            },
            embedded_graphics_simulator::SimulatorEvent::KeyUp { keycode, .. } => match keycode {
                Keycode::Left | Keycode::Up => Ok(Event::MainEncoderRotation(-1)),
                Keycode::Right | Keycode::Down => Ok(Event::MainEncoderRotation(1)),
                Keycode::Space | Keycode::Return | Keycode::Return2 => {
                    Ok(Event::MainEncoderButtonUp)
                },
                _ => Err(()),
            },
            embedded_graphics_simulator::SimulatorEvent::MouseWheel { scroll_delta, direction } => {
                let dir = match direction {
                    sdl2::MouseWheelDirection::Normal => 1,
                    sdl2::MouseWheelDirection::Flipped => -1,
                    sdl2::MouseWheelDirection::Unknown(_) => {
                        panic!("Unknown mouse direction is not supported")
                    },
                };

                let offset = scroll_delta.y * dir;

                Ok(Event::MainEncoderRotation(offset))
            },
            embedded_graphics_simulator::SimulatorEvent::MouseButtonDown { mouse_btn, .. }
                if mouse_btn == MouseButton::Middle || mouse_btn == MouseButton::Left =>
            {
                Ok(Event::MainEncoderButtonDown)
            },
            embedded_graphics_simulator::SimulatorEvent::MouseButtonUp { mouse_btn, .. }
                if mouse_btn == MouseButton::Middle || mouse_btn == MouseButton::Left =>
            {
                Ok(Event::MainEncoderButtonUp)
            },
            embedded_graphics_simulator::SimulatorEvent::Quit => exit(0),
            _ => Err(()),
        }
    }
}

impl embedded_ui::event::Event for Event {
    fn as_common(&self) -> Option<CommonEvent> {
        match self {
            Event::MainEncoderRotation(offset) => Some(CommonEvent::FocusMove(*offset)),
            Event::MainEncoderButtonDown => Some(CommonEvent::FocusClickDown),
            Event::MainEncoderButtonUp => Some(CommonEvent::FocusClickUp),
        }
    }

    fn as_select_shift(&self) -> Option<i32> {
        match self {
            &Event::MainEncoderRotation(offset) => Some(offset),
            _ => None,
        }
    }

    fn as_slider_shift(&self) -> Option<i32> {
        match self {
            &Event::MainEncoderRotation(offset) => Some(offset),
            _ => None,
        }
    }

    fn as_knob_rotation(&self) -> Option<i32> {
        match self {
            &Event::MainEncoderRotation(offset) => Some(offset),
            _ => None,
        }
    }

    fn as_input_letter_scroll(&self) -> Option<i32> {
        match self {
            &Event::MainEncoderRotation(offset) => Some(offset),
            _ => None,
        }
    }
}

#[derive(Clone, Copy)]
enum Message {
    None,
    Focus(ElId),
    KnobChange(u8),
}

fn main() {
    let output_settings = OutputSettingsBuilder::new().scale(2).build();

    let mut window = Window::new("TEST", &output_settings);

    let mut display = SimulatorDisplay::<Rgb888>::new(Size::new(480, 270));

    // I don't certainly know why, but display must be drawn at least once before
    // event fetching. Otherwise SDL2 will panic :(
    window.update(&display);

    let knob_value = Value::dynamic(0u8);

    let col = row![
        col![text("OSC1"), button("TYPE"), button("SYNC"), button("EDIT")].gap(1),
        // col![text("OSC2"), button("TYPE"), button("SYNC"), button("EDIT")],
        // col![text("OSC3"), header_line, button("TYPE"), button("SYNC"), button("EDIT")],
        // col![
        //     select(["1", "2", "3"]).cycle(true),
        //     select(["1", "2", "3"]).cycle(true),
        //     select(["1", "2", "3"]).cycle(true)
        // ],
        // col![
        //     slider_h(|pos| {
        //         println!("pos: {pos}");
        //         Message::None
        //     })
        //     .step(8),
        //     slider_h(|pos| {
        //         println!("pos: {pos}");
        //         Message::None
        //     }),
        //     row![
        //         checkbox(|state| {
        //             println!("Checkbox state: {state}");
        //             Message::None
        //         }),
        //         checkbox(|state| {
        //             println!("Checkbox state: {state}");
        //             Message::None
        //         }),
        //         checkbox(|state| {
        //             println!("Checkbox state: {state}");
        //             Message::None
        //         })
        //     ],
        col![
            select_v(["one", "two", "three", "four", "five"]).circular(true),
            row![bar_v().value(0.8), bar_h()].padding(5).gap(5),
            row![IconKind::SnakeCw]
        ],
        col![
            Knob::new(knob_value.clone()),
            text(knob_value.clone()),
            checkbox(|value| {
                println!("{value}");
                Message::None
            }),
        ]
    ];

    let mut ui = UI::new(col, display.bounding_box().size.into()).rgb888().theme(Theme::AyuLight);

    ui.auto_focus();

    loop {
        ui.tick(window.events().filter_map(|event| Event::try_from(event).ok()));

        while let Some(message) = ui.deque_message() {
            match message {
                Message::Focus(id) => ui.focus(id),
                Message::KnobChange(_value) => {},
                Message::None => {},
            }
        }

        ui.draw(&mut display);

        // display
        //     .bounding_box()
        //     .draw_styled(&PrimitiveStyle::with_fill(Rgb888::WHITE), &mut display)
        //     .unwrap();
        // RoundedRectangle::new(
        //     Rectangle::new(Point::new(300, 50), Size::new(100, 50)),
        //     CornerRadii::new(Size::new_equal(5)),
        // )
        // .draw_styled(
        //     &PrimitiveStyleBuilder::new()
        //         .stroke_color(RgbColor::BLACK)
        //         .stroke_width(1)
        //         .fill_color(Rgb888::BLACK)
        //         .build(),
        //     &mut display,
        // )
        // .unwrap();

        window.update(&display);
    }
}
