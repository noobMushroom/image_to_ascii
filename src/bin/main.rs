use ascii_lib::{create_ascii, AsciiChars, ImageRatio};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Path of the image that you want to convert
    #[arg(short, long, value_name = "IMAGE")]
    image: PathBuf,

    /// Size of the output image
    #[arg(value_enum)]
    size: ImageRatio,
}

fn main() {
    let cli = Cli::parse();
    let char_map = AsciiChars::new(' ', '.', '*', '#');

    match create_ascii(&cli.image, char_map, cli.size) {
        Ok(image) => println!("{}", image),
        Err(err) => eprintln!(
            "Failed to process image: encountered the following error \n\t{}",
            err
        ),
    }
}
