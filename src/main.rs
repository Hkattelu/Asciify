extern crate image;
extern crate structopt;

use image::Rgba;
use image::imageops::FilterType;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use structopt::StructOpt;

/** The ascii characters to use in order of dark to bright with a 10 character precision */
const SHALLOW_GREY_SCALE: [char; 10] = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];

/** The ascii characters to use in order of dark to bright with a 65 character precision */
const DEEP_GREY_SCALE: [char; 65] = [
    ' ', '`', '^', '"', ',', ':', ';', 'I', 'l', '!', 'i', '~', '+', '_', '-', '?', ']', '[', '}',
    '{', '1', ')', '(', '|', '/', 't', 'f', 'j', 'r', 'x', 'n', 'u', 'v', 'c', 'z', 'X', 'Y', 'U',
    'J', 'C', 'L', 'Q', '0', 'O', 'Z', 'm', 'w', 'q', 'p', 'd', 'b', 'k', 'h', 'a', 'o', '*', '#',
    'M', 'W', '&', '8', '%', 'B', '@', '$',
];

/** Index of the alpha channel of a pixel */
const ALPHA_INDEX: usize = 3;

#[derive(Debug, StructOpt)]
#[structopt(name = "asciify", about = "Convert images to ascii images")]
struct Opt {
    /// Use deep grey scale
    #[structopt(short, long)]
    deep: bool,

    /// Use inverted colors
    #[structopt(short, long)]
    invert: bool,

    /// The comma-separated width and height to resize the image to
    #[structopt(short, long)]
    resize: Option<Vec<u32>>,

    /// Input image to print
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

/**
 * Computes the brightness of the pixel as a number between 0 and 1
 */
fn brightness(pixel: &Rgba<u8>) -> f32 {
    let avg_rgb: f32 = (((pixel[0] as u16) + (pixel[1] as u16) + (pixel[2] as u16)) as f32) / 3.0;
    let opacity: f32 = (pixel[ALPHA_INDEX] as f32) / 255.0;
    // We can cast to f32 since the brightness is between 0 and 1
    (avg_rgb * opacity) / 255.0 as f32
}

fn ascii_char_for_brightness(brightness: f32, deep: bool, invert: bool) -> char {
    let epsilon = 0.000001;
    let max_index = if deep {64} else {9};
    let scale = (max_index as f32) + 1.0 - epsilon;
    let scaled = (brightness * scale) as usize;
    let index = if invert { max_index - scaled } else { scaled };
    if deep { DEEP_GREY_SCALE[index] } else { SHALLOW_GREY_SCALE[index] }
}

fn pixel_to_ascii_char(pixel: &Rgba<u8>, deep: bool, invert: bool) -> char {
    ascii_char_for_brightness(brightness(pixel), deep, invert)
}

fn main() {
    let opt = Opt::from_args();
    let path = opt.input;
    let deep = opt.deep;
    let invert = opt.invert;
    let file = File::open(&path).unwrap();
    let image_reader = BufReader::new(file);
    let mut loaded = image::load(image_reader, image::ImageFormat::from_path(&path).unwrap()).unwrap();
    
    // This makes the image more crisp in ascii form by making the borders darker
    loaded.invert();    

    if let Some(dimensions) = opt.resize {
        if dimensions.len() != 2 {
            panic!("Must provide exactly two numbers to resize for width and height. Provided {} arguments", dimensions.len());
        }
        loaded = loaded.resize(dimensions[0], dimensions[1], FilterType::Nearest);
    }

    let rgba_image = loaded.to_rgba();
    let rows = rgba_image.rows();
    let lines = rows
        .map(|row| row.map(|pixel| pixel_to_ascii_char(pixel, deep, invert)))
        .map(|char_vec| char_vec.collect::<String>())
        .collect::<Vec<String>>();

    let merged = lines.into_iter().collect::<Vec<String>>().join("\n");
    println!("{}", merged);
}
