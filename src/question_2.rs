use regex::Regex;
use std::fs::File;
use std::path::Path;

pub fn answer(file: &str) {
    let im = image::open(&Path::new(&file)).unwrap();
    let imgbuf = im.to_luma();

    let re = Regex::new(r"\.jpg").unwrap();
    let out_filename = format!("{}_gray.png", re.replace_all(&file, ""));

    let fout = &mut File::create(&Path::new(&out_filename)).unwrap();

    // TODO: ここもグレースケールにする
    image::ImageLuma8(imgbuf)
        .write_to(fout, image::PNG)
        .unwrap();
}
