use image::{DynamicImage, GenericImageView, ImageError, Pixel};
use std::path::Path;
use terminal_size::{terminal_size, Height, Width};

pub struct AsciiChars {
    light: char,
    medium: char,
    dark: char,
    extra_dark: char,
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

pub fn create_ascii(img_path: &Path, char_map: AsciiChars) -> Result<String, ImageError> {
    let img = image::open(img_path)?;
    let grey_img = change_grayscale(img);
    let sized_image = sized_image(grey_img, get_terminal_dimensions());
    Ok(map_image(sized_image, char_map))
}

fn sized_image(img: DynamicImage, window_size: (Width, Height)) -> DynamicImage {
    if img.height() > window_size.1 .0 as u32 || img.width() > window_size.0 .0 as u32 {
        img.resize(
            window_size.0 .0 as u32 - 5,
            img.height(),
            image::imageops::FilterType::Nearest,
        )
    } else {
        img
    }
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
