use ascii_lib::{create_ascii, AsciiChars};
use std::path::Path;

fn main() {
    // let img_path =
    //     Path::new(" ");
    let char_map = AsciiChars::new(' ', '.', '*', '#');
    println!("{}", create_ascii(img_path, char_map).unwrap());
    // _ = create_ascii(img_path, char_map).unwrap();
}
