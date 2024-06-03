use clap::ValueEnum;
use image::{DynamicImage, GenericImageView, ImageError, Pixel};
use std::path::PathBuf;
use terminal_size::{terminal_size, Height, Width};

pub struct AsciiChars {
    light: char,
    medium: char,
    dark: char,
    extra_dark: char,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum ImageRatio {
    /// The height of the image will stay same, width will change according to terminal width
    /// [Defualt]
    Fill,
    /// Both height and width will change according to the terminal image will maintain the aspect
    /// ratio
    Fit,
    /// Keep the original size of the image
    Original,
}

struct ImageSize {
    width: u32,
    height: u32,
}

impl ImageSize {
    fn new(width: u32, height: u32) -> Self {
        ImageSize { width, height }
    }
}

impl AsciiChars {
    pub fn new(light: char, medium: char, dark: char, extra_dark: char) -> Self {
        AsciiChars {
            light,
            medium,
            dark,
            extra_dark,
        }
    }
}

pub fn create_ascii(
    img_path: &PathBuf,
    char_map: AsciiChars,
    size: ImageRatio,
) -> Result<String, ImageError> {
    let img = image::open(img_path)?;
    let grey_img = change_grayscale(img);
    let image_size = get_image_size(size, &grey_img);
    let sized_image = sized_image(grey_img, image_size);
    Ok(map_image(sized_image, char_map))
}

fn get_image_size(options: ImageRatio, img: &DynamicImage) -> ImageSize {
    match options {
        ImageRatio::Fit => {
            let window_size = get_terminal_dimensions();
            ImageSize::new(window_size.0 .0 as u32, window_size.1 .0 as u32)
        }
        ImageRatio::Fill => {
            let window_size = get_terminal_dimensions();
            let image_height = img.height();
            ImageSize::new(window_size.0 .0 as u32 - 5, image_height)
        }
        ImageRatio::Original => ImageSize::new(img.width(), img.height()),
    }
}

fn sized_image(img: DynamicImage, window_size: ImageSize) -> DynamicImage {
    img.resize(
        window_size.width,
        window_size.height,
        image::imageops::FilterType::Nearest,
    )
}

fn change_grayscale(img: DynamicImage) -> DynamicImage {
    img.grayscale()
}

fn get_char(intensity: u8, chars: &AsciiChars) -> char {
    match intensity {
        0..=100 => chars.light,
        101..=150 => chars.medium,
        151..=200 => chars.dark,
        _ => chars.extra_dark,
    }
}

fn map_image(img: DynamicImage, char_map: AsciiChars) -> String {
    let mut char_img: Vec<char> = Vec::with_capacity((img.width() * img.height()) as usize);

    for (x, y, pixel) in img.pixels() {
        if x == 0 && y != 0 {
            char_img.push('\n');
        }
        let intensity = pixel.to_luma().0[0];
        char_img.push(get_char(intensity, &char_map));
    }

    char_img.iter().collect()
}

fn get_terminal_dimensions() -> (Width, Height) {
    let dimensions = terminal_size();
    if let Some((Width(w), Height(h))) = dimensions {
        (Width(w), Height(h))
    } else {
        (Width(200), Height(60))
    }
}
