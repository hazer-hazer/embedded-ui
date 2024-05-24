use std::process::exit;

use embedded_graphics::{geometry::Size, pixelcolor::BinaryColor};
use embedded_graphics_simulator::{sdl2, OutputSettingsBuilder, SimulatorDisplay, Window};
use embedded_ui::{
    align::HorizontalAlign,
    col,
    el::ElId,
    event::CommonEvent,
    helpers::{button, checkbox, h_div, select, text},
    icons::IconKind,
    render::Renderer,
    row,
    size::Length,
    ui::UI,
};

#[derive(Clone, Copy)]
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
}

#[derive(Clone, Copy)]
enum Message {
    Focus(ElId),
}

fn main() {
    let output_settings = OutputSettingsBuilder::new()
        .theme(embedded_graphics_simulator::BinaryColorTheme::OledWhite)
        .pixel_spacing(1)
        .scale(3)
        .build();

    let mut window = Window::new("TEST", &output_settings);

    let mut display = SimulatorDisplay::<BinaryColor>::new(Size::new(128, 32));

    // I don't certainly know why, but display must be drawn at least once before event fetching. Otherwise SDL2 will panic :(
    window.update(&display);

    let header_line = h_div().padding(0);

    let col = row![
        col![text("OSC1"), header_line, button("TYPE"), button("SYNC"), button("EDIT")],
        col![text("OSC2"), header_line, button("TYPE"), button("SYNC"), button("EDIT")],
        // col![text("OSC3"), header_line, button("TYPE"), button("SYNC"), button("EDIT")],
        col![select(["1", "2", "3"])],
    ];

    let mut ui = UI::new(col, display.bounds().size).no_controls().monochrome();

    loop {
        ui.feed_events(window.events().filter_map(|event| Event::try_from(event).ok()));
        ui.tick();

        while let Some(message) = ui.deque_message() {
            match message {
                Message::Focus(id) => ui.focus(id),
            }
        }

        ui.draw(&mut display);
        window.update(&display);
    }
}
