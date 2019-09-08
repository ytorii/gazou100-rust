use image::{DynamicImage, ImageBuffer};

pub fn answer(im: &DynamicImage) -> ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let mut imgbuf = im.to_rgb();

    imgbuf.pixels_mut().for_each(|pixel| {
        let image::Rgb(p) = pixel;
        p.swap(0, 2);
    });

    imgbuf
}
