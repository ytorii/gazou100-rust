use image::{DynamicImage, GenericImage, GenericImageView, ImageBuffer, Rgb};

pub fn answer(im: &DynamicImage) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut image_buf = im.to_rgb();
    let grid = 8;
    let mut sub = image_buf.sub_image(0, 0, grid, grid);

    for i in 0..(im.width() / grid) {
        for j in 0..(im.height() / grid) {
            let offset_x = grid * i;
            let offset_y = grid * j;
            sub.change_bounds(offset_x, offset_y, grid, grid);

            let max_rgb = sub.pixels().fold(Rgb([0, 0, 0]), |mut acc, (_x, _y, rgb)| {
                acc[0] = max_of(acc[0], rgb[0]);
                acc[1] = max_of(acc[1], rgb[1]);
                acc[2] = max_of(acc[2], rgb[2]);
                acc
            });

            for x in 0..grid {
                for y in 0..grid {
                    sub.inner_mut()
                        .put_pixel(offset_x + x, offset_y + y, max_rgb);
                }
            }
        }
    }

    image_buf
}

fn max_of(left: u8, right: u8) -> u8 {
    if left > right {
        left
    } else {
        right
    }
}
