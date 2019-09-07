use std::env;
use std::fs::File;
use std::path::Path;

fn main() {
    let file = if env::args().count() == 2 {
        env::args().nth(1).unwrap()
    } else {
        panic!("Please enter a file")
    };

    let im = image::open(&Path::new(&file)).unwrap();

    let mut imgbuf = im.to_rgba();

    for (_, _, pixel) in imgbuf.enumerate_pixels_mut() {
        let mut res_pixel = pixel.clone();
        res_pixel[2] = res_pixel[0];
        *pixel = res_pixel;
    }

    let fout = &mut File::create(&Path::new(&format!("{}.png", file))).unwrap();
    image::ImageRgba8(imgbuf).write_to(fout, image::PNG).unwrap();

}