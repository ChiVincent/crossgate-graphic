extern crate crossgate;

use std::env;
use std::process;
use crossgate::resource::Paths;

fn main() {
    let args: Vec<String> = env::args().collect();
    let paths = Paths::new(&args).unwrap_or_else(|err| {
        eprintln!("Fatal Error: {}", err);
        eprintln!("Usage: ./crossgate-graphic [GraphicInfo.bin] [Graphic.bin] [Palette.cgp]");
        process::exit(1);
    });

    println!("{:?}", paths);
}
