use std::process::exit;

use embedded_graphics::{geometry::Size, pixelcolor::BinaryColor};
use embedded_graphics_simulator::{
    sdl2::{self, MouseButton},
    OutputSettingsBuilder, SimulatorDisplay, Window,
};
use embedded_ui::{
    align::HorizontalAlign,
    col,
    el::ElId,
    event::CommonEvent,
    helpers::{button, checkbox, h_div, select, slider_h, slider_v, text},
    icons::IconKind,
    kit::knob::Knob,
    render::Renderer,
    row,
    size::Length,
    ui::UI,
    value::Value,
};

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
            embedded_graphics_simulator::SimulatorEvent::MouseWheel { scroll_delta, direction } => {
                let dir = match direction {
                    sdl2::MouseWheelDirection::Normal => 1,
                    sdl2::MouseWheelDirection::Flipped => -1,
                    sdl2::MouseWheelDirection::Unknown(_) => {
                        panic!("Unknown mouse direction is not supported")
                    },
                };

                let offset = scroll_delta.y * dir;

                println!("Offset encoder: {offset}");

                Ok(Event::MainEncoderRotation(offset))
            },
            embedded_graphics_simulator::SimulatorEvent::MouseButtonDown { mouse_btn, .. }
                if mouse_btn == MouseButton::Middle =>
            {
                Ok(Event::MainEncoderButtonDown)
            },
            embedded_graphics_simulator::SimulatorEvent::MouseButtonUp { mouse_btn, .. }
                if mouse_btn == MouseButton::Middle =>
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

// struct Sandbox {
//     knob_value: u8,
// }

// impl Sandbox {
//     fn update(&mut self, message: Message) {
//         match message {
//             Message::Focus(id) => ui.focus(id),
//             Message::KnobChange(value) => {
//                 self.knob_value = value;
//             },
//             Message::None => {},
//         }
//     }
// }

fn main() {
    let output_settings = OutputSettingsBuilder::new()
        .theme(embedded_graphics_simulator::BinaryColorTheme::OledWhite)
        .pixel_spacing(0)
        .scale(4)
        .build();

    let mut window = Window::new("TEST", &output_settings);

    let mut display = SimulatorDisplay::<BinaryColor>::new(Size::new(128, 32));

    // I don't certainly know why, but display must be drawn at least once before event fetching. Otherwise SDL2 will panic :(
    window.update(&display);

    // let header_line = h_div().padding(0);

    let knob_value = Value::dynamic(0u8);

    let col = row![
        col![text("OSC1"), button("TYPE"), button("SYNC"), button("EDIT")],
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
        // ],
        col![select(["1", "2", "3"]).cycle(true)],
        col![Knob::new(knob_value.clone()), text(knob_value.clone())]
    ];

    let mut ui = UI::new(col, display.bounds().size).monochrome();

    ui.auto_focus();

    loop {
        ui.tick(window.events().filter_map(|event| Event::try_from(event).ok()));

        while let Some(message) = ui.deque_message() {
            match message {
                Message::Focus(id) => ui.focus(id),
                Message::KnobChange(value) => {},
                Message::None => {},
            }
        }

        ui.draw(&mut display);
        window.update(&display);
    }
}
