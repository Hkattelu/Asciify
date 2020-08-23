extern crate image;


use std::io::BufReader;
use std::fs::File;

/** The ascii characters to use in order of dark to bright */
const BASE_CHARS: &str = "@&#%$*/-. ";

fn main() {
    let path = "test-images/SPECIAL(ChocoboA)900.png";
    let file = File::open(path).unwrap();
    let image_reader = BufReader::new(file);
    let image = image::load(image_reader, image::ImageFormat::from_path(path).unwrap()).unwrap();
    let dims = image.to_rgba().dimensions();
    println!("width: {}, height: {}", dims.0, dims.1);
}