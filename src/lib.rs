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
    extern crate byteorder;
    
    use std::fs::File;
    use std::io::{Read, Error, Cursor, Seek, SeekFrom, ErrorKind};
    use byteorder::{ReadBytesExt, LittleEndian};


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
                match file.read_exact(&mut buffer) {
                    Ok(_) => ret.push(Self::make(&mut buffer)?),
                    Err(_) => break,
                }
            }

            Ok(ret)
        }

        fn make(buf: &mut [u8]) -> Result<Self, Error> {
            let mut rdr = Cursor::new(&buf);

            let id = rdr.read_u32::<LittleEndian>()?;
            let address = rdr.read_u32::<LittleEndian>()?;
            let length = rdr.read_u32::<LittleEndian>()?;
            let offset_x = rdr.read_i32::<LittleEndian>()?;
            let offset_y = rdr.read_i32::<LittleEndian>()?;
            let width = rdr.read_u32::<LittleEndian>()?;
            let height = rdr.read_u32::<LittleEndian>()?;
            let tile_east = rdr.read_i8()?;
            let tile_south = rdr.read_i8()?;
            let access = rdr.read_i8()?;
            let mut unknown = [0; 5];
            rdr.read_i8_into(&mut unknown)?;
            let map_id = rdr.read_u32::<LittleEndian>()?;

            Ok(GraphicInfo {
                id, address, length, offset_x, offset_y, width, height, tile_east, tile_south, 
                access, _unknown: unknown, map_id,
            })
        }
    }

    #[derive(Debug)]
    pub struct Graphic {
        identifier: [i8; 2],
        version: i8,
        _unknown: i8,
        width: u32,
        height: u32,
        length: u32,
        data: Vec<i8>,
    }

    impl Graphic {
        pub fn new (graphic_info: &GraphicInfo, graphic: &mut File) -> Result<Self, Error> {
            graphic.seek(SeekFrom::Start(graphic_info.address.into()))?;

            let mut identifier = [0; 2];
            graphic.read_i8_into(&mut identifier)?;

            if identifier != [82, 68] {
                return Err(Error::new(ErrorKind::InvalidData, "invalid graphic identifier."));
            }

            let version = graphic.read_i8()?;
            let unknown = graphic.read_i8()?;
            let width = graphic.read_u32::<LittleEndian>()?;
            let height = graphic.read_u32::<LittleEndian>()?;
            let length = graphic.read_u32::<LittleEndian>()?;
            let mut data = vec![0; length as usize];
            graphic.read_i8_into(&mut data)?;

            Ok(Graphic {
                identifier, version, _unknown: unknown, width, height, length, data,
            })
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;
        use std::fs::File;
        

        #[test]
        fn it_can_make_graphic_info() {
            let mut chunks: Vec<[u8; 40]> = vec![
                [
                    // First chunk
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xA8, 0x01, 0x00, 0x00, 0xE0, 0xFF, 0xFF, 0xFF,
                    0xE8, 0xFF, 0xFF, 0xFF, 0x40, 0x00, 0x00, 0x00, 0x2F, 0x00, 0x00, 0x00, 0x01, 0x01, 0x01, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0xE7, 0x03, 0x00, 0x00,
                ],
                [
                    // Second chunk
                    0x01, 0x00, 0x00, 0x00, 0xA8, 0x01, 0x00, 0x00, 0x12, 0x00, 0x00, 0x00, 0xE0, 0xFF, 0xFF, 0xFF, 
                    0xE8, 0xFF, 0xFF, 0xFF, 0x40, 0x00, 0x00, 0x00, 0x30, 0x00, 0x00, 0x00, 0x01, 0x01, 0x00, 0x00, 
                    0x00, 0x00, 0x00, 0x00, 0x12, 0x00, 0x00, 0x00,
                ]
            ];

            let graphic_info = vec![
                GraphicInfo::make(&mut chunks[0]).unwrap(),
                GraphicInfo::make(&mut chunks[1]).unwrap(),
            ];
            
            assert_eq!(graphic_info[0].id, 0);
            assert_eq!(graphic_info[0].address, 0);
            assert_eq!(graphic_info[0].length, 424);
            assert_eq!(graphic_info[0].offset_x, -32);
            assert_eq!(graphic_info[0].offset_y, -24);
            assert_eq!(graphic_info[0].width, 64);
            assert_eq!(graphic_info[0].height, 47);
            assert_eq!(graphic_info[0].tile_east, 1);
            assert_eq!(graphic_info[0].tile_south, 1);
            assert_eq!(graphic_info[0].access, 1);
            assert_eq!(graphic_info[0].map_id, 999);

            assert_eq!(graphic_info[1].id, 1);
            assert_eq!(graphic_info[1].address, 424);
            assert_eq!(graphic_info[1].length, 18);
            assert_eq!(graphic_info[1].offset_x, -32);
            assert_eq!(graphic_info[1].offset_y, -24);
            assert_eq!(graphic_info[1].width, 64);
            assert_eq!(graphic_info[1].height, 48);
            assert_eq!(graphic_info[1].tile_east, 1);
            assert_eq!(graphic_info[1].tile_south, 1);
            assert_eq!(graphic_info[1].access, 0);
            assert_eq!(graphic_info[1].map_id, 18);
        }

        #[test]
        fn it_can_new_graphic_info() {
            let mut file = File::open("resources/GraphicInfo.test.bin").unwrap();
            let graphic_info = GraphicInfo::new(&mut file).unwrap();

            assert_eq!(graphic_info.len(), 2);
        }

        #[test]
        fn it_can_new_graphic() {
            let graphic_info = GraphicInfo {
                id: 0, address: 0, length: 424, offset_x: -32, offset_y: -24, width: 64, height: 47,
                tile_east: 1, tile_south: 1, access: 1, _unknown: [0; 5], map_id: 999
            };
            let mut graphic_file = File::open("resources/Graphic.test.bin").unwrap();

            let graphic = Graphic::new(&graphic_info, &mut graphic_file).unwrap();

            assert_eq!(graphic.identifier, [82, 68]);
            assert_eq!(graphic.version, 1);
            assert_eq!(graphic.width, 64);
            assert_eq!(graphic.height, 47);
            assert_eq!(graphic.length, 424);
        }

        #[test]
        fn it_cannot_new_graphic() {
            let graphic_info = GraphicInfo {
                id: 0, address: 0, length: 424, offset_x: -32, offset_y: -24, width: 64, height: 47,
                tile_east: 1, tile_south: 1, access: 1, _unknown: [0; 5], map_id: 999
            };
            let mut graphic_file = File::open("resources/GraphicInfo.test.bin").unwrap(); // Open the wrong file

            assert!(Graphic::new(&graphic_info, &mut graphic_file).is_err());
        }
    }
}