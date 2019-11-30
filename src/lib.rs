pub mod resource {
    use std::io;
    use std::fs::File;

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

    #[derive(Debug)]
    pub struct Files {
        graphic_info: File,
        graphic: File,
        palette: File,
    }

    impl Files {
        pub fn new (paths: &Paths) -> Result<Self, io::Error> {
            let graphic_info = File::open(&paths.graphic_info)?;
            let graphic = File::open(&paths.graphic)?;
            let palette = File::open(&paths.palette)?;

            Ok(Files {
                graphic_info: graphic_info, graphic: graphic, palette: palette
            })
        }
    }
}

