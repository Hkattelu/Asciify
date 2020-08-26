extern crate image;

use image::Rgba;
use image::buffer::Rows;
use std::fs::File;
use std::path::PathBuf;
use image::imageops::FilterType;
use std::io::{BufReader, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

/** The ascii characters to use in order of dark to bright with a 10 character precision */
const SHALLOW_GRAY_SCALE: [char; 10] = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];

/** The ascii characters to use in order of dark to bright with a 65 character precision */
const DEEP_GRAY_SCALE: [char; 65] = [
    ' ', '`', '^', '"', ',', ':', ';', 'I', 'l', '!', 'i', '~', '+', '_', '-', '?', ']', '[', '}',
    '{', '1', ')', '(', '|', '/', 't', 'f', 'j', 'r', 'x', 'n', 'u', 'v', 'c', 'z', 'X', 'Y', 'U',
    'J', 'C', 'L', 'Q', '0', 'O', 'Z', 'm', 'w', 'q', 'p', 'd', 'b', 'k', 'h', 'a', 'o', '*', '#',
    'M', 'W', '&', '8', '%', 'B', '@', '$',
];

/** Index of the alpha channel of a pixel */
const ALPHA_INDEX: usize = 3;

enum ColorType {
    Grayscale,
    Colored(Color),
}

pub struct AsciiPoint {
    color: ColorType,
    brightness: f32,
}

/** Computes the brightness of the pixel as a number between 0 and 1 */
pub fn compute_brightness(pixel: &Rgba<u8>) -> f32 {
    let avg_rgb: f32 = (((pixel[0] as u16) + (pixel[1] as u16) + (pixel[2] as u16)) as f32) / 3.0;
    let opacity: f32 = (pixel[ALPHA_INDEX] as f32) / 255.0;
    // We can cast to f32 since the brightness is between 0 and 1
    (avg_rgb * opacity) / 255.0 as f32
}

/** 
 * Create the information required to construct an ascii representation
 * of the provided pixel in grayscale 
 */
pub fn gray_point_for_pixel(pixel: &Rgba<u8>) -> AsciiPoint {
    AsciiPoint {
        brightness: compute_brightness(pixel),
        color: ColorType::Grayscale,
    }
}

/** 
 * Create the information required to construct an ascii representation
 * of the provided pixel maintaining its color
 */
pub fn colored_point_for_pixel(pixel: &Rgba<u8>) -> AsciiPoint {
    AsciiPoint {
        brightness: compute_brightness(pixel),
        color: ColorType::Colored(Color::Rgb(pixel[0], pixel[1], pixel[2])),
    }
}

/** Fetches the corresponding ascii character to represent the provided brightness */
pub fn ascii_char_for_point(point: AsciiPoint, deep: bool, invert: bool) -> char {
    let epsilon = 0.0001;
    let max_index = if deep {64} else {9};
    let scale = (max_index as f32) + 1.0 - epsilon;
    let scaled = (point.brightness * scale).floor() as usize;
    let index = if invert { max_index - scaled } else { scaled };
    if deep { DEEP_GRAY_SCALE[index] } else { SHALLOW_GRAY_SCALE[index] }
}

pub struct AsciiBuilder {
    path: PathBuf,
    deep: bool,
    invert: bool,
    resize: Option<(u32, u32)>,
}

impl AsciiBuilder {
    pub fn new(path: PathBuf) -> AsciiBuilder {
        AsciiBuilder {
            path: path,
            deep: false,
            invert: false,
            resize: None,
        }
    }

    pub fn set_deep(&self, deep: bool) -> AsciiBuilder {
        let path = PathBuf::from(self.path.as_path());
        let mut builder = AsciiBuilder {
            path: path,
            deep: self.deep,
            invert: self.invert,
            resize: self.resize,
        };
        builder.deep = deep;
        return builder;
    }

    pub fn set_invert(&mut self, invert: bool) -> AsciiBuilder {
        let path = PathBuf::from(self.path.as_path());
        let mut builder = AsciiBuilder {
            path: path,
            deep: self.deep,
            invert: self.invert,
            resize: self.resize,
        };
        builder.invert = invert;
        return builder;
    }

    pub fn set_resize(&mut self, resize: (u32, u32)) -> AsciiBuilder {
        let path = PathBuf::from(self.path.as_path());
        let mut builder = AsciiBuilder {
            path: path,
            deep: self.deep,
            invert: self.invert,
            resize: self.resize,
        };
        builder.resize = Some(resize);
        return builder;
    }

    pub fn clear_resize(&mut self) -> AsciiBuilder {
        let path = PathBuf::from(self.path.as_path());
        let mut builder = AsciiBuilder {
            path: path,
            deep: self.deep,
            invert: self.invert,
            resize: self.resize,
        };
        builder.resize = None;
        return builder;
    }

    pub fn build(&self) -> String {
        let img = self.get_img().to_rgba();
        let lines = img.rows()
            .map(|row| row.map(|pixel| ascii_char_for_point(gray_point_for_pixel(pixel), self.deep, self.invert)))
            .map(|char_vec| char_vec.collect::<String>())
            .collect::<Vec<String>>();

        lines.into_iter().collect::<Vec<String>>().join("\n")
    }

    pub fn to_std_out(&self, use_color: bool) {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        let img = self.get_img().to_rgba();
        img.rows()
           .map(|row| row.map(|pixel| if use_color { colored_point_for_pixel(pixel) } else { gray_point_for_pixel(pixel) }))
           .for_each(|point_row| {
                point_row.for_each(|point| {
                    let color = match point.color {
                        ColorType::Colored(point_color) => point_color,
                        ColorType::Grayscale => Color::White,
                    };
                    stdout.set_color(ColorSpec::new().set_fg(Some(color))).unwrap();
                    write!(&mut stdout, "{}", ascii_char_for_point(point, self.deep, self.invert)).unwrap();
                });
                writeln!(&mut stdout, "").unwrap();
            });
        stdout.flush().unwrap();
    }

    fn get_img(&self) -> image::DynamicImage {
        let file = File::open(&self.path).unwrap();
        let image_reader = BufReader::new(file);
        let mut loaded = image::load(image_reader, image::ImageFormat::from_path(&self.path).unwrap()).unwrap();
        if let Some(dimensions) = self.resize {
            loaded = loaded.resize_exact(dimensions.0, dimensions.1, FilterType::Nearest);
        }

        loaded
    }
}