use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb, RgbImage};

pub fn answer(im: &DynamicImage) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let image_buf = im.to_rgb();

    let hsvs = rgb_to_inverted_hsv(&image_buf);
    hsv_to_rgb(hsvs, im.width(), im.height())
}

fn rgb_to_inverted_hsv(buf: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Vec<(i32, i32, i32)> {
    buf.pixels().fold(vec![], |mut acc, pixel| {
        let Rgb(rgb) = pixel;

        let (max, min) = (rgb.iter().max().unwrap(), rgb.iter().min().unwrap());

        let s = (max - min) as f32;

        let h: i32 = match (max, min) {
            (x, y) if x == y => 180 % 360,
            (_x, y) if rgb[2] == *y => {
                ((60.0 * (rgb[1] as f32 - rgb[0] as f32) / s + 60.0) as i32 + 180) % 360
            }
            (_x, y) if rgb[0] == *y => {
                ((60.0 * (rgb[2] as f32 - rgb[1] as f32) / s + 180.0) as i32 + 180) % 360
            }
            _ => ((60.0 * (rgb[0] as f32 - rgb[2] as f32) / s + 300.0) as i32 + 180) % 360,
        };

        acc.push((h, s as i32, *max as i32));
        acc
    })
}

fn hsv_to_rgb(
    hsvs: Vec<(i32, i32, i32)>,
    width: u32,
    height: u32,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    hsvs.iter()
        .enumerate()
        .fold(RgbImage::new(width, height), |mut res, (i, &(h, s, v))| {
            let c = s as f32;
            let hd = h as f32 / 60.0;
            let x = c * (1.0 - (hd % 2.0 - 1.0).abs());
            let d = v as f32 - c;
            let (r, g, b) = match hd {
                y if 0.0 <= y && y < 1.0 => ((c + d), (x + d), d),
                y if 1.0 <= y && y < 2.0 => ((x + d), (c + d), d),
                y if 2.0 <= y && y < 3.0 => (d, (c + d), (x + d)),
                y if 3.0 <= y && y < 4.0 => (d, (x + d), (c + d)),
                y if 4.0 <= y && y < 5.0 => ((x + d), d, (c + d)),
                y if 5.0 <= y && y < 6.0 => ((c + d), d, (x + d)),
                _ => (d, d, d),
            };
            res.put_pixel(
                i as u32 % width,
                i as u32 / width,
                Rgb([r.round() as u8, g.round() as u8, b.round() as u8]),
            );
            res
        })
}
