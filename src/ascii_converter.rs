use std::{
    io::Write,

    fs::{
        self,
        File
    },

    fmt::{
        Display, 
        Formatter
    }
};

use crate::ascii_char::AsciiChar;

#[derive(Debug)]
pub struct AsciiConverter {
    width: i32,
    height: i32,
    payload: String,
}

impl AsciiConverter {
    
    pub fn new(payload: String) -> Self {
        let mut read_file = payload.split("\n").collect::<Vec<&str>>();
        let mut file_specs = read_file.remove(0).split(" ");

        let payload = read_file.join("\n");
        let width = file_specs.next().unwrap().parse::<i32>().unwrap();
        let height = file_specs.next().unwrap().parse::<i32>().unwrap();

        Self {
            width,
            height,
            payload,
        }
    }

    pub fn convert(&self) {
        let char_map = fs::read("src/fixtures/char_map.fodase").unwrap();
        let char_map = char_map;

        for (i, char) in char_map.iter().enumerate() {
            println!("Index: {:?} | {:?}", i, char);
        }

        let foo_payload = self.payload
            .chars()
            .collect::<Vec<char>>()
            .chunks(self.width as usize)
            .map(|v| v.to_vec())
            .collect::<Vec<Vec<char>>>()
            .iter().map(|chunk| {

                chunk.iter()
                    .map(|&ascii_byte| {
                        AsciiChar::new(ascii_byte)
                    }).collect::<Vec<AsciiChar>>()

            }).collect::<Vec<Vec<AsciiChar>>>();

        let mut output = File::create("fodase.foo2").expect("deu ruim");
        for chunk in foo_payload {
            chunk.iter().for_each(|char| {
                write!(output, "{}", char.convert(&char_map, 4)).unwrap()
            });
            write!(output, "{}", "\n").unwrap();
        }
    }
}

impl Display for AsciiConverter {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Width: {} | Height: {} | Payload Size: {}", self.width, self.height, self.payload.len())
    }

}