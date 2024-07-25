use std::process::exit;

use embedded_graphics::{
    geometry::{Dimensions, Size},
    pixelcolor::Rgb888,
};
use embedded_graphics_simulator::{
    sdl2::{self, Keycode, MouseButton},
    OutputSettingsBuilder, SimulatorDisplay, Window,
};
use embedded_ui::{
    col,
    el::ElId,
    event::CommonEvent,
    helpers::{bar_h, button, container, scrollable_h, select_v},
    icons::IconKind,
    row,
    simulator::single_encoder::Event,
    ui::UI,
    widgets::container::InsideContainerExt,
};
use embedded_ui::{helpers::bar_v, theme::Theme};

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

    let col = row![
        col!["This is a text inside a container", button("Button").height(50), button("EDIT")].gap(1).padding(2),
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
            // text("This is a checkbox and some text also blah-blah-blah yeah yeah").wrap(),
            // checkbox(|value| {
            //     println!("{value}");
            //     Message::None
            // })
            // .wrap()
            scrollable_h("Super long text Super long text Super long text Super long text Super long text Super long text Super long text Super long text Super long text Super long text Super long text Super long text Super long text Super long text Super long text Super long text Super long text Super long text Super long text Super long text Super long text Super long text Super long text Super long text Super long text Super long text Super long text Super long text Super long text Super long text Super long text Super long text Super long text Super long text Super long text Super long text Super long text Super long text Super long text Super long text Super long text Super long text Super long text Super long text Super long text"),
            "kek"
        ]
    ]
    .gap(1);

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

        // RoundedRectangle::new(
        //     Rectangle::new(Point::new(100, 100), Size::new(100, 100)),
        //     CornerRadii::new(Size::new_equal(2)),
        // )
        // .draw_styled(&PrimitiveStyle::with_fill(Rgb888::BLACK), &mut display)
        // .unwrap();

        window.update(&display);
    }
}
