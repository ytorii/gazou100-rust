use image::{DynamicImage, GenericImageView, GrayImage, ImageBuffer};

pub fn answer(im: &DynamicImage) -> ImageBuffer<image::Luma<u8>, Vec<u8>> {
    im.to_rgb().enumerate_pixels().fold(
        GrayImage::new(im.width(), im.height()),
        |mut res, (x, y, pixel)| {
            res.put_pixel(x, y, gray_pixel_of(pixel));
            res
        },
    )
}

fn gray_pixel_of(rgb_pixel: &image::Rgb<u8>) -> image::Luma<u8> {
    let gray_pixel = (0.2126 * (rgb_pixel[0] as f32)
        + 0.7152 * (rgb_pixel[1] as f32)
        + 0.0722 * (rgb_pixel[2] as f32)) as u8;
    image::Luma([gray_pixel])
}
