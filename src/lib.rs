pub mod resource {
    #[derive(Debug)]
    pub struct Paths {
        graphic_info: String,
        graphic: String,
        palette: String,
    }
    
    impl Paths {
        pub fn new (args: &[String]) -> Result<Self, &'static str> {
            if args.len() != 4 {
                return Err("not enough parameters.");
            }
    
            let graphic_info = args[1].clone();
            let graphic = args[2].clone();
            let palette = args[3].clone();
    
            Ok(Paths {
                graphic_info: graphic_info, graphic: graphic, palette: palette
            })
        }
    }
}

