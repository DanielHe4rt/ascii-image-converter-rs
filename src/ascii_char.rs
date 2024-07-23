#[derive(Debug)]
pub struct AsciiChar {
    raw_char: char,
    ascii_value: u8,
    density: f64,
    brightness: u32,
}

impl AsciiChar {

    pub fn new(ascii_byte: char) -> Self {
        let ascii_byte_u32 = ascii_byte as u32;
        let red = (ascii_byte_u32 >> 16) & 0xff;
        let green = (ascii_byte_u32 >> 8) & 0xff;
        let blue = (ascii_byte_u32) & 0xff;
        let brightness = (red + green + blue) / 3;
        let density = brightness as f64 / 255.0;

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
