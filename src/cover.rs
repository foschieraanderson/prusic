use image::{DynamicImage, GenericImageView};

#[derive(Clone, Copy)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct CoverFrame {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Pixel>,
}

pub fn decode(data: &[u8]) -> image::ImageResult<DynamicImage> {
    image::load_from_memory(data)
}

pub fn to_ascii(image: &DynamicImage, width: u32, height: u32) -> Vec<String> {
    let image = image.resize_exact(width, height, image::imageops::FilterType::Triangle);

    let image = image.to_luma8();

    let chars = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];

    let mut lines = Vec::new();

    for y in 0..height {
        let mut line = String::new();

        for x in 0..width {
            let pixel = image.get_pixel(x, y);
            let brightness = pixel[0] as usize;

            let index = brightness * (chars.len() - 1) / 255;

            line.push(chars[index]);
        }

        lines.push(line);
    }

    lines
}

pub fn to_terminal(image: &DynamicImage, width: u32, height: u32) -> Vec<String> {
    let image = image.resize_exact(width, height, image::imageops::FilterType::Triangle);

    let rgb = image.to_rgb8();

    let mut lines = Vec::new();

    let mut y = 0;

    while y < height {
        let mut line = String::new();

        for x in 0..width {
            let top = rgb.get_pixel(x, y);

            let bottom = if y + 1 < height {
                rgb.get_pixel(x, y + 1)
            } else {
                top
            };

            line.push_str(&format!(
                "\x1b[38;2;{};{};{}m\x1b[48;2;{};{};{}m▀",
                top[0], top[1], top[2], bottom[0], bottom[1], bottom[2],
            ));
        }

        line.push_str("\x1b[0m");

        lines.push(line);

        y += 2;
    }

    lines
}
