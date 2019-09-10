use image::{DynamicImage, ImageBuffer, Luma};

pub fn answer(im: &DynamicImage) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let mut gray_buf = im.to_luma();
    let mut t = 1;
    let mut sb_max = 1.0;

    (1..255).for_each(|i| {
        let sb_i = sb(i as u8, &gray_buf);
        if sb_i > sb_max {
            sb_max = sb_i;
            t = i;
        }
    });

    gray_buf.pixels_mut().for_each(|pixel| {
        if pixel[0] > t {
            pixel[0] = 255;
        } else {
            pixel[0] = 0;
        }
    });

    gray_buf
}

fn sb(t: u8, buf: &ImageBuffer<Luma<u8>, Vec<u8>>) -> f32 {
    let vec0: Vec<f32> = buf
        .pixels()
        .filter(|pixel| pixel[0] < t)
        .map(|luma| luma[0] as f32)
        .collect();
    let vec1: Vec<f32> = buf
        .pixels()
        .filter(|pixel| t <= pixel[0])
        .map(|luma| luma[0] as f32)
        .collect();
    let w0 = (vec0.len() as f32) / (buf.pixels().len() as f32);

    let m0 = (vec0.iter().sum::<f32>()) / (vec0.len() as f32);
    let m1 = (vec1.iter().sum::<f32>()) / (vec1.len() as f32);

    w0 * (1.0 - w0) * (m0 - m1).powi(2)
}
