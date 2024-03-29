mod question_9;

use question_9::answer;
use std::env;
use std::path::Path;

fn main() {
    let (input_filename, output_filename) = if env::args().count() == 3 {
        (env::args().nth(1).unwrap(), env::args().nth(2).unwrap())
    } else {
        panic!("Usage: cargo run <input filename> <output filename>")
    };

    let im = image::open(&Path::new(&input_filename)).unwrap();

    let result_imgbuf = answer(&im);

    result_imgbuf.save(Path::new(&output_filename)).unwrap();
}
