extern crate regex;

use std::env;
use std::fs::File;
use std::path::Path;
use regex::Regex;

fn main() {
    let file = if env::args().count() == 2 {
        env::args().nth(1).unwrap()
    } else {
        panic!("Please enter a file")
    };

    let im = image::open(&Path::new(&file)).unwrap();

    let mut imgbuf = im.to_rgba();

    for (_, _, pixel) in imgbuf.enumerate_pixels_mut() {
        pixel[2] = pixel[0];
    }

    let re = Regex::new(r"\.jpg").unwrap();
    let out_filename = format!("{}.png", re.replace_all(&file, ""));

    let fout = &mut File::create(&Path::new(&out_filename)).unwrap();
    image::ImageRgba8(imgbuf).write_to(fout, image::PNG).unwrap();

}