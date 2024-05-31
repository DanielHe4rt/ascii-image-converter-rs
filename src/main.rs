use std::fmt::{Display, Formatter};
use std::fs;
use std::fs::File;
use std::io::Write;

fn main() {
    let payload = std::fs::read("src/fixtures/img0.foo")
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
    index: usize,
    converted_char: char,
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
        let ascii_list_length = char_map.len() as i32;

        for (i, char) in char_map.iter().enumerate() {
            println!("Index: {:?} | {:?}", i, char);
        }


        let foo_payload = self.payload
            .as_bytes()
            .chunks(3)
            .map(|v| v.to_vec())
            .collect::<Vec<Vec<u8>>>()
            .iter().map(|chunk| {
            chunk.iter()
                .map(|&ascii_byte| {
                    let blue = ((ascii_byte) & 0xff) as u32;
                    let red = (ascii_byte as u32 >> 16u32) & 0xff;
                    let green = (ascii_byte as u32 >> 8u32) & 0xff;
                    let brightness = ((red + green + blue) / 3);
                    let density = (brightness as f64 / 255.0) as f64;
                    let ascii_index = ((ascii_list_length - 1) as f64 * density) as usize;
                    let converted_char = char_map.get(ascii_index).unwrap().clone() as char;
                    AsciiChar {
                        raw_char: ascii_byte as char,
                        ascii_value: ascii_byte,
                        brightness,
                        density,
                        index: ascii_index,
                        converted_char
                    }
                }).collect::<Vec<AsciiChar>>()
        }).collect::<Vec<Vec<AsciiChar>>>();


        let mut output = File::create("fodase.foo2").expect("deu ruim");
        for chunk in foo_payload {
            chunk.iter().for_each(|char| {
                println!("{:?}", char);
                write!(output, "{}", char.converted_char).unwrap()
            });
            write!(output, "{}", "\n").unwrap();

        }


        // for (i, char) in foo_filtered.into_iter().enumerate() {
        //     println!("{i}, {char}");
        // }
    }
}

impl Display for AsciiConverter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Width: {} | Height: {} | Payload Size: {}", self.width, self.height, self.payload.len())
    }
}
