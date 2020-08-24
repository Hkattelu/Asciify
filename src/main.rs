extern crate image;
extern crate structopt;

use image::Rgba;
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

    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

/**
 * Computes the brightness of the pixel as a number between 0 and 1
 */
fn brightness(pixel: &Rgba<u8>) -> f32 {
    let avg_rgb: f32 = (((pixel[0] as u16) + (pixel[1] as u16) + (pixel[2] as u16)) as f32) / 3.0;
    let opacity: f32 = (pixel[ALPHA_INDEX] as f32) / 255.0;
    // We can cast to f32 since the opacity is between 0 and 1
    (avg_rgb * opacity) / 255.0 as f32
}

fn ascii_char_for_brightness(brightness: f32, deep: bool, invert: bool) -> char {
    // We can cast to f32 since the brightness is between 0 and 1
    if deep {
        let index = (brightness * 64.9) as usize;
        if invert {
            DEEP_GREY_SCALE[64 - index]
        } else {
            DEEP_GREY_SCALE[index]
        }
    } else {
        let index = (brightness * 9.9) as usize;
        if invert {
            SHALLOW_GREY_SCALE[9 - index]
        } else {
            SHALLOW_GREY_SCALE[index]
        }
    }
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
    let loaded = image::load(image_reader, image::ImageFormat::from_path(&path).unwrap()).unwrap();
    let rgba_image = loaded.to_rgba();
    let rows = rgba_image.rows();
    let transformed_rows = rows
        .map(|row| {
            row.map(|x| pixel_to_ascii_char(x, deep, invert))
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    let lines = transformed_rows
        .into_iter()
        .map(|char_vec| char_vec.into_iter().collect::<String>())
        .collect::<Vec<String>>();

    let merged = lines.into_iter().collect::<Vec<String>>().join("\n");
    println!("{}", merged);
}
