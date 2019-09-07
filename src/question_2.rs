use regex::Regex;
use std::path::Path;

use image::{GenericImageView, GrayImage};

pub fn answer(file: &str) {
    let im = image::open(&Path::new(&file)).unwrap();

    let gray_imgbuf = im.to_rgb().enumerate_pixels().fold(
        GrayImage::new(im.width(), im.height()),
        |mut res, (x, y, pixel)| {
            res.put_pixel(x, y, gray_pixel_of(pixel));
            res
        },
    );

    let re = Regex::new(r"\.jpg").unwrap();
    let out_filename = format!("{}", re.replace_all(&file, "_gray.jpg"));

    gray_imgbuf.save(Path::new(&out_filename)).unwrap();
}

fn gray_pixel_of(rgb_pixel: &image::Rgb<u8>) -> image::Luma<u8> {
    let gray_pixel = (0.2126 * (rgb_pixel[0] as f32)
        + 0.7152 * (rgb_pixel[1] as f32)
        + 0.0722 * (rgb_pixel[2] as f32)) as u8;
    image::Luma([gray_pixel])
}
