use regex::Regex;
use std::path::Path;

use image::GenericImageView;

pub fn answer(file: &str) {
    let im = image::open(&Path::new(&file)).unwrap();
    let mut gray_imgbuf = image::GrayImage::new(im.width(), im.height());

    gray_imgbuf
        .enumerate_pixels_mut()
        .for_each(|(x, y, pixel)| {
            let rgb = im.get_pixel(x, y);
            pixel[0] = (0.2126 * (rgb[0] as f32)
                + 0.7152 * (rgb[1] as f32)
                + 0.0722 * (rgb[2] as f32)) as u8;
        });

    let re = Regex::new(r"\.jpg").unwrap();
    let out_filename = format!("{}_gray.jpg", re.replace_all(&file, ""));

    gray_imgbuf.save(Path::new(&out_filename)).unwrap();
}
