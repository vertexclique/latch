use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::{egcircle, egline, egtext, primitive_style, text_style};
use embedded_graphics::fonts::Font6x8;
use embedded_graphics_simulator::{
    BinaryColorTheme, SimulatorDisplay, SimulatorEvent, Window, OutputSettingsBuilder,
};
use std::thread;
use std::time::Duration;

fn main() {
    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(128, 64));
    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();
    let mut window = Window::new("Example", &output_settings);

    egtext!(text = "Hello World!", top_left = Point::zero(), style = text_style!(font = Font6x8, text_color = BinaryColor::On)).draw(&mut display);

    egcircle!(center = (96, 32), radius = 31, style = primitive_style!(stroke_color = BinaryColor::On)).draw(&mut display);

    egline!(start = (32, 32), end = (1, 32), style = primitive_style!(stroke_color = BinaryColor::On))
        .translate(Point::new(64, 0))
        .draw(&mut display);
    egline!(start = (32, 32), end = (40, 40), style = primitive_style!(stroke_color = BinaryColor::On))
        .translate(Point::new(64, 0))
        .draw(&mut display);

    'running: loop {
        window.update(&display);

        for event in window.events() {
            match event {
                SimulatorEvent::MouseButtonUp { point, .. } => {
                    println!("Click event at ({}, {})", point.x, point.y);
                }
                SimulatorEvent::Quit => break 'running,
                _ => {}
            }

            thread::sleep(Duration::from_millis(200));
        }
    }
}
