use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let paths = ResourcePaths::new(&args).unwrap_or_else(|err| {
        eprintln!("Fatal Error: {}", err);
        eprintln!("Usage: ./crossgate-graphic [GraphicInfo.bin] [Graphic.bin] [Palette.cgp]");
        process::exit(1);
    });

    println!("{:?}", paths);
}

#[derive(Debug)]
struct ResourcePaths {
    graphic_info: String,
    graphic: String,
    palette: String,
}

impl ResourcePaths {
    fn new (args: &[String]) -> Result<Self, &'static str> {
        if args.len() != 4 {
            return Err("not enough parameters.");
        }

        let graphic_info = args[1].clone();
        let graphic = args[2].clone();
        let palette = args[3].clone();

        Ok(ResourcePaths{
            graphic_info: graphic_info, graphic: graphic, palette: palette
        })
    }
}
