use std::fs::File;
use regex::Regex;
use std::path::Path;

pub fn answer(file: &str) {
    let im = image::open(&Path::new(&file)).unwrap();
    let mut imgbuf = im.to_rgba();

    imgbuf.pixels_mut().for_each(|pixel| {
        let image::Rgba(p) = pixel;
        p.swap(0, 2);
    });

    let re = Regex::new(r"\.jpg").unwrap();
    let out_filename = format!("{}.png", re.replace_all(&file, ""));

    let fout = &mut File::create(&Path::new(&out_filename)).unwrap();
    image::ImageRgba8(imgbuf).write_to(fout, image::PNG).unwrap();

}