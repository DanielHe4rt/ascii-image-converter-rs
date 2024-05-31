use std::fmt::{Display, Formatter};
use std::fs;
use std::fs::File;
use std::io::Write;

fn main() {
    let payload = std::fs::read("src/fixtures/img2.foo")
        .unwrap()
        .iter()
        .map(|&x| x as char)
        .collect::<String>();

    let converter = AsciiConverter::new(payload);

    converter.convert();

    println!("{}", converter);
}

struct AsciiGrayScale {}

#[derive(Debug)]
struct AsciiChar {
    raw_char: char,
    ascii_value: u8,
    density: f64,
    brightness: u32,
}

impl AsciiChar {
    pub fn new(ascii_byte: char) -> Self {
        let ascii_byte_u32 = ascii_byte as u32;
        let red = ((ascii_byte_u32 >> 16) & 0xff);
        let green = ((ascii_byte_u32 >> 8) & 0xff);
        let blue = (ascii_byte_u32) & 0xff;
        let brightness = (red + green + blue) / 3;
        let density = (brightness as f64 / 255.0);
        Self {
            raw_char: ascii_byte,
            ascii_value: ascii_byte as u8,
            density,
            brightness,
        }
    }

    pub fn convert(&self, table: &Vec<u8>, density_level: usize) -> char {
        let ascii_index = ((table.len() / density_level) as f64) * self.density;

        table.get(ascii_index as usize).unwrap().clone() as char
    }
}

#[derive(Debug)]
struct AsciiConverter {
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
        let mut char_map = fs::read("src/fixtures/char_map.fodase").unwrap();
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
