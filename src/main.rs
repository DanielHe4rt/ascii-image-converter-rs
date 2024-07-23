mod ascii_char;
mod ascii_converter;

use ascii_converter::AsciiConverter;

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
