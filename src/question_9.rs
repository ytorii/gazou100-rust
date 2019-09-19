use image::{DynamicImage, GenericImage, GenericImageView, ImageBuffer, Rgb};

pub fn answer(im: &DynamicImage) -> DynamicImage {
    let kernel = [1.0, 2.0, 1.0, 2.0, 8.0, 2.0, 1.0, 2.0, 1.0];
    let k = kernel.iter().map(|val| val / 16.0f32).collect::<Vec<f32>>();
    im.filter3x3(&k)
}
