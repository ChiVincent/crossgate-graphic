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
    use std::io::{Read, Error};

    #[derive(Debug)]
    pub struct GraphicInfo {
        id: u32,
        address: u32,
        length: u32,
        offset_x: i32,
        offset_y: i32,
        width: u32,
        height: u32,
        tile_east: i8,
        tile_south: i8,
        access: i8,
        _unknown: [i8; 5],
        map_id: u32,
    }

    impl GraphicInfo {
        pub fn new(file: &mut File) -> Result<Vec<Self>, Error> {
            let mut ret = vec![];

            loop {
                let mut buffer = [0; 40];
                if file.read(&mut buffer)? == 0 {
                    break;
                }
                ret.push(Self::make(&mut buffer));
            }

            Ok(ret)
        }

        fn make(buf: &mut [u8]) -> Self {
            GraphicInfo {
                id: 0, address: 0, length: 0, offset_x: 0, offset_y: 0, width: 0, height: 0,
                tile_east: 0, tile_south: 0, access: 0, _unknown: [0; 5], map_id: 0
            }
        }
    }
}