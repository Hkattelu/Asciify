extern crate image;

use image::{Rgba};
use std::io::BufReader;
use std::fs::File;
/** The ascii characters to use in order of dark to bright */
const BASE_CHARS: [char; 10] = [' ', '.', '-', '/', '*', '$', '%', '#', '&', '@'];

/** Index of the alpha channel of a pixel */
const ALPHA_INDEX: usize = 3;

/** 
 * Computes the brightness of the pixel as a number between 0 and 1
 */
fn brightness(pixel: &Rgba<u8>) -> f32 {
    let avg_rgb: f32 = (((pixel[0] as u16) + (pixel[1] as u16) + (pixel[2] as u16)) as f32)/3.0;
    let opacity: f32 = (pixel[ALPHA_INDEX] as f32)/255.0;
    // We can cast to f32 since the opacity is between 0 and 1
    (avg_rgb * opacity)/255.0 as f32
}

fn ascii_char_for_brightness(brightness: f32) -> char {
    // We can cast to f32 since the brightness is between 0 and 1
    let index = (brightness * 9.9) as usize;
    BASE_CHARS[index]
}

fn pixel_to_ascii_char(pixel: &Rgba<u8>) -> char {
    ascii_char_for_brightness(brightness(pixel))
}

fn main() {
    let path = "test-images/SPECIAL(ChocoboA)900.png";
    let file = File::open(path).unwrap();
    let image_reader = BufReader::new(file);
    let loaded = image::load(image_reader, image::ImageFormat::from_path(path).unwrap()).unwrap();
    let rgba_image = loaded.to_rgba();
    let rows = rgba_image.rows();
    let transformed_rows = rows
        .map(|row| row.map(|x| pixel_to_ascii_char(x)).collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let lines = transformed_rows.into_iter()
        .map(|char_vec| char_vec.into_iter().collect::<String>())
        .collect::<Vec<String>>();

    let merged = lines.into_iter().collect::<Vec<String>>().join("\n");
    println!("{}", merged);
}