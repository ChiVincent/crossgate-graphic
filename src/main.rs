extern crate crossgate;

use std::env;
use std::process;
use crossgate::resource::{Paths, Files};

fn main() {
    let args: Vec<String> = env::args().collect();
    let paths = Paths::new(&args).unwrap_or_else(|err| {
        eprintln!("Fatal Error: {}", err);
        eprintln!("Usage: ./crossgate-graphic [GraphicInfo.bin] [Graphic.bin] [Palette.cgp]");
        process::exit(1);
    });
    let files = Files::new(&paths).unwrap_or_else(|err| {
        eprintln!("Fatal Error: {}", err);
        eprintln!("Please check [GraphicInfo.bin] [Graphic.bin] and [Palette.cgp] exist and readable.");
        process::exit(2);
    });

    println!("{:?}", files);
}
