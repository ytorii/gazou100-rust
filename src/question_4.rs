use image::{DynamicImage, ImageBuffer, Luma};

pub fn answer(im: &DynamicImage) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let mut gray_buf = im.to_luma();

    let (t, _sb_max) = (1..255).fold((0, 0.0), |mut res, i| {
        let sb_i = sb(i, &gray_buf);
        if sb_i > res.1 {
            res.0 = i;
            res.1 = sb_i;
        }
        res
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
    let (class0, class1) = classes_of(t, buf);

    let w0 = (class0.len() as f32) / (buf.pixels().len() as f32);

    let m0 = (class0.iter().sum::<f32>()) / (class0.len() as f32);
    let m1 = (class1.iter().sum::<f32>()) / (class1.len() as f32);

    w0 * (1.0 - w0) * (m0 - m1).powi(2)
}

fn classes_of(t: u8, buf: &ImageBuffer<Luma<u8>, Vec<u8>>) -> (Vec<f32>, Vec<f32>) {
    buf.pixels().fold((vec![], vec![]), |mut res, &pixel| {
        let Luma([p]) = pixel;
        if p < t {
            res.0.push(p as f32)
        } else {
            res.1.push(p as f32)
        }
        res
    })
}
