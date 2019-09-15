use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb, RgbImage};

pub fn answer(im: &DynamicImage) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let image_buf = im.to_rgb();

    let hsvs = rgb_to_hsvs(&image_buf);
    hsv_to_inverted_rgb(hsvs, im.width(), im.height())
}

fn rgb_to_hsvs(buf: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Vec<Hsv> {
    buf.pixels().fold(vec![], |mut acc, rgb_pixel| {
        let hsv = Hsv::from_rgb(rgb_pixel);
        acc.push(hsv);
        acc
    })
}

fn hsv_to_inverted_rgb(hsvs: Vec<Hsv>, width: u32, height: u32) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    hsvs.iter()
        .enumerate()
        .fold(RgbImage::new(width, height), |mut res, (i, hsv)| {
            res.put_pixel(i as u32 % width, i as u32 / width, hsv.invert().to_rgb());
            res
        })
}

struct Hsv {
    h: f32,
    s: f32,
    v: f32,
}

impl Hsv {
    pub fn from_rgb(rgb_pixel: &Rgb<u8>) -> Hsv {
        let Rgb(rgb) = rgb_pixel;

        let (max, min) = (rgb.iter().max().unwrap(), rgb.iter().min().unwrap());
        let v = *max as f32;
        let s = v - *min as f32;
        let h: f32 = match (max, min) {
            (x, y) if x == y => 180.0 % 360.0,
            (_x, y) if rgb[2] == *y => 60.0 * (rgb[1] as f32 - rgb[0] as f32) / s + 60.0,
            (_x, y) if rgb[0] == *y => 60.0 * (rgb[2] as f32 - rgb[1] as f32) / s + 180.0,
            _ => 60.0 * (rgb[0] as f32 - rgb[2] as f32) / s + 300.0,
        };
        Hsv { h: h, s: s, v: v }
    }

    fn invert(&self) -> Hsv {
        Hsv {
            h: (self.h + 180.0) % 360.0,
            s: self.s,
            v: self.v,
        }
    }

    fn to_rgb(&self) -> Rgb<u8> {
        let c = self.s;
        let hd = self.h / 60.0;
        let x = c * (1.0 - (hd % 2.0 - 1.0).abs());
        let d = self.v - c;

        let (r, g, b) = match hd {
            y if 0.0 <= y && y < 1.0 => (c, x, 0.0),
            y if 1.0 <= y && y < 2.0 => (x, c, 0.0),
            y if 2.0 <= y && y < 3.0 => (0.0, c, x),
            y if 3.0 <= y && y < 4.0 => (0.0, x, c),
            y if 4.0 <= y && y < 5.0 => (x, 0.0, c),
            y if 5.0 <= y && y < 6.0 => (c, 0.0, x),
            _ => (0.0, 0.0, 0.0),
        };

        Rgb([(r + d) as u8, (g + d) as u8, (b + d) as u8])
    }
}
