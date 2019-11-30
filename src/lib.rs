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
        pub graphic_info: File,
        pub graphic: File,
        pub palette: File,
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

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn it_can_make_paths() {
            let args: Vec<String> = vec![
                String::from("./crossgate-graphic"),
                String::from("GraphicInfo.bin"),
                String::from("Graphic.bin"),
                String::from("Palette.cgp"),
            ];
            let paths = Paths::new(&args).unwrap();

            assert_eq!(paths.graphic_info, "GraphicInfo.bin");
            assert_eq!(paths.graphic, "Graphic.bin");
            assert_eq!(paths.palette, "Palette.cgp");
        }

        #[test]
        fn it_cannot_make_paths() {
            let args: Vec<String> = vec![
                String::from("./crossgate-graphic"),
                String::from("GraphicInfo.bin"),
                String::from("Graphic.bin"),
                // Lake of Palette.cgp.
            ];
            
            assert!(Paths::new(&args).is_err());
        }

        #[test]
        fn it_can_open_files() {
            let args: Vec<String> = vec![
                String::from("./crossgate-graphic"),
                String::from("resources/GraphicInfo_66.bin"),
                String::from("resources/Graphic_66.bin"),
                String::from("resources/palet_00.cgp"),
            ];
            let paths = Paths::new(&args).unwrap();
            
            assert!(Files::new(&paths).is_ok());
        }

        #[test]
        fn it_cannot_open_files() {
            let args: Vec<String> = vec![
                String::from("./crossgate-graphic"),
                String::from("resources/GraphicInfo_66.bin"),
                String::from("resources/Graphic_66.bin"),
                String::from("resources/palet_00.cgp.d"), // Not found
            ];
            let paths = Paths::new(&args).unwrap();
            
            assert!(Files::new(&paths).is_err());
        }
    }
}

pub mod structure {
    use std::fs::File;

    #[derive(Debug)]
    pub struct GraphicInfo {

    }

    impl GraphicInfo {
        pub fn new(file: &File) -> Result<Vec<GraphicInfo>, &'static str> {
            Ok(vec!(GraphicInfo{}))
        }
    }
}