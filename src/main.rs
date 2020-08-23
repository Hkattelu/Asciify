extern crate image;


use std::io::BufReader;
use std::fs::File;

/** The ascii characters to use in order of dark to bright */
const BASE_CHARS: &str = "@&#%$*/-. ";

/** Index of the alpha channel of a pixel */
const ALPHA_INDEX: usize = 3;

/** Value of the alpha channel indicating that the pixel is transparent */
const TRANSPARENT: u8 = 0;

fn main() {
    let transparent = BASE_CHARS.chars().last().unwrap();
    let opaque = BASE_CHARS.chars().next().unwrap();

    let path = "test-images/SPECIAL(ChocoboA)900.png";
    let file = File::open(path).unwrap();
    let image_reader = BufReader::new(file);
    let loaded = image::load(image_reader, image::ImageFormat::from_path(path).unwrap()).unwrap();
    let rgba_image = loaded.to_rgba();
    let rows = rgba_image.rows();
    let transformed_rows = rows
        .map(|row| row.map(|x| return x[ALPHA_INDEX] == TRANSPARENT).collect::<Vec<bool>>())
        .map(|row| row.into_iter().map(|x| return if x { transparent } else { opaque }).collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let lines = transformed_rows.into_iter()
        .map(|char_vec| char_vec.into_iter().collect::<String>())
        .collect::<Vec<String>>();

    let merged = lines.into_iter().collect::<Vec<String>>().join("\n");
    println!("{}", merged);
}