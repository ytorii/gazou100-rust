use image::{DynamicImage, ImageBuffer, Rgb};

pub fn answer(im: &DynamicImage) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut image_buf = im.to_rgb();

    image_buf.pixels_mut().for_each(|rgb_pixel| {
        let Rgb([r, g, b]) = *rgb_pixel;
        *rgb_pixel = Rgb([decrease(r), decrease(g), decrease(b)]);
    });

    image_buf
}

fn decrease(val: u8) -> u8 {
    match val {
        y if y < 64 => 32,
        y if 64 <= y && y < 128 => 96,
        y if 128 <= y && y < 192 => 160,
        _ => 224,
    }
}
