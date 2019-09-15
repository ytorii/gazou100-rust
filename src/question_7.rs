use image::{DynamicImage, GenericImage, GenericImageView, ImageBuffer, Rgb};

pub fn answer(im: &DynamicImage) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut image_buf = im.to_rgb();
    let grid = 8;
    for i in 0..(im.width() / grid) {
        for j in 0..(im.height() / grid) {
            let offset_x = grid * i;
            let offset_y = grid * j;

            let sub = image_buf.sub_image(offset_x, offset_y, grid, grid);
            let sum_pixcels = sub.to_image().pixels().fold((0, 0, 0), |mut acc, pixel| {
                acc.0 += pixel[0] as u32;
                acc.1 += pixel[1] as u32;
                acc.2 += pixel[2] as u32;
                acc
            });

            let area = (grid as u32).pow(2);

            let convluted_rgb = Rgb([
                (sum_pixcels.0 / area) as u8,
                (sum_pixcels.1 / area) as u8,
                (sum_pixcels.2 / area) as u8,
            ]);

            sub.to_image()
                .enumerate_pixels()
                .for_each(|(x, y, _pixel)| {
                    image_buf.put_pixel(offset_x + x, offset_y + y, convluted_rgb);
                });
        }
    }

    image_buf
}
