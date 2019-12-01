extern crate crossgate;

use std::env;
use std::process;
use crossgate::resource::{Paths, Files};
use crossgate::structure::{GraphicInfo, Graphic};

fn main() {
    let args: Vec<String> = env::args().collect();
    let paths = Paths::new(&args).unwrap_or_else(|err| {
        eprintln!("Fatal Error: {}", err);
        eprintln!("Usage: ./crossgate-graphic [GraphicInfo.bin] [Graphic.bin] [Palette.cgp]");
        process::exit(1);
    });
    let mut files = Files::new(&paths).unwrap_or_else(|err| {
        eprintln!("Fatal Error: {}", err);
        eprintln!("Please check [GraphicInfo.bin] [Graphic.bin] and [Palette.cgp] exist and readable.");
        process::exit(1);
    });
    let graphic_info = GraphicInfo::new(&mut files.graphic_info).unwrap();
    let graphic = Graphic::new(&graphic_info[0], &mut files.graphic).unwrap_or_else(|err| {
        eprintln!("Fatal Error: {}", err);
        eprintln!("Cannot parse [Graphic.bin], it might be broken.");
        process::exit(1);
    });

    println!("{:?}, {:?}", graphic_info[3], graphic);
}
