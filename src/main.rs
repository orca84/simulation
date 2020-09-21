#[macro_use]
extern crate diesel;

#[macro_use]
extern crate serde_derive;

mod errors;
mod iching;
mod schema;
mod settings;
mod wires;

use errors::SimulationResult;
use iching::{Hexagram, Line, Trigram};
use rs_ws281x::Controller;
use settings::Settings;
use std::thread;
use std::time::Duration;
use wires::{build_controller, reset};

fn main() -> SimulationResult<()> {
    println!("iOracle simulation");
    println!("------------------");

    let settings = Settings::read()?;
    let mut top_controller = build_controller(0, 12)?;
    // let mut bottom_controller = build_controller(1, 13)?;

    // run(settings, &mut top_controller, &mut bottom_controller)
    run(settings, &mut top_controller)
}

fn run(
    settings: Settings,
    top_controller: &mut Controller,
    // bottom_controller: &mut Controller,
) -> SimulationResult<()> {
    let line1 = Line::random();
    println!("Line 1: {}", line1);
    line1.render(1, top_controller, &settings.default_colour);
    thread::sleep(Duration::from_secs(1));

    let line2 = Line::random();
    println!("Line 2: {}", line2);
    line2.render(2, top_controller, &settings.default_colour);
    thread::sleep(Duration::from_secs(1));

    let line3 = Line::random();
    println!("Line 3: {}", line3);
    line3.render(3, top_controller, &settings.default_colour);
    thread::sleep(Duration::from_secs(1));

    let top_trigram = Trigram {
        top: line1,
        middle: line2,
        bottom: line3,
    };
    println!("Top Trigram: {}", top_trigram);
    top_trigram.render(&settings, top_controller);
    thread::sleep(Duration::from_secs(1));

    // let line4 = Line::random();
    // println!("Line 4: {}", line4);
    // line4.render(4, top_controller, &settings.default_colour);
    // thread::sleep(Duration::from_secs(1));
    //
    // let line5 = Line::random();
    // println!("Line 5: {}", line5);
    // line5.render(5, top_controller, &settings.default_colour);
    // thread::sleep(Duration::from_secs(1));
    //
    // let line6 = Line::random();
    // println!("Line 6: {}", line6);
    // line6.render(6, top_controller, &settings.default_colour);
    // thread::sleep(Duration::from_secs(1));
    //
    // let bottom_trigram = Trigram {
    //     top: line4,
    //     middle: line5,
    //     bottom: line6,
    // };
    // println!("Bottom Trigram: {}", bottom_trigram);
    // bottom_trigram.render(&settings, bottom_controller);
    // thread::sleep(Duration::from_secs(1));
    //
    // let hexagram = Hexagram {
    //     top: top_trigram,
    //     bottom: bottom_trigram,
    // };
    // println!("Hexagram: {}", hexagram);

    reset(&settings, top_controller);
    // reset(&settings, bottom_controller);

    Ok(())
}
