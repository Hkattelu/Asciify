extern crate image;

use image::{Rgba, DynamicImage};
use std::{fs::File, io::Cursor};
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
    image: DynamicImage,
    deep: bool,
    invert: bool,
}

//This is a macro and not a function due to closure types
macro_rules! impl_stream {
    ($map:expr, $print:expr, $image:expr) => {{
        let img = $image.to_rgba();
        img.rows()
           .map(|row| row.map($map))
           .for_each($print);
    }};
}


impl AsciiBuilder {
    pub fn new_from_path(path: PathBuf) -> Self {
        let file = File::open(&path).unwrap();
        let image_reader = BufReader::new(file);
        let loaded = image::load(image_reader, image::ImageFormat::from_path(&path).unwrap()).unwrap();

        AsciiBuilder {
            image: loaded,
            deep: false,
            invert: false,
        }
    }

    #[deprecated(since="0.1.5", note="please use `new_from_path` instead")]
    pub fn new(path: PathBuf) -> Self {
        Self::new_from_path(path)
    }

    pub fn new_from_image(image: DynamicImage) -> Self {
        AsciiBuilder {
            image,
            deep: false,
            invert: false
        }
    }

    pub fn set_deep(mut self, deep: bool) -> Self {
        self.deep = deep;
        self
    }

    pub fn set_invert(mut self, invert: bool) -> Self {
        self.invert = invert;
        self
    }

    pub fn set_resize(mut self, resize: (u32, u32)) -> Self {
        self.image = self.image.resize_exact(resize.0, resize.1, FilterType::Nearest);
        self
    }

    pub fn build(&self) -> String {
        let mut buf = Vec::new();
        let mut cursor = Cursor::new(&mut buf);
        self.to_stream(&mut cursor); 
        String::from_utf8(buf).unwrap()
    }

    /// Writes the image to standard output
    /// the image is formatted with ANSI colors if `use_color` is true
    pub fn to_std_out(&self, use_color: bool) {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        match use_color {
            true => self.to_stream_colored(&mut stdout),
            false => self.to_stream(&mut stdout)
        }
    }

    /// Writes a colored version of the image to `stream` using ANSI code
    pub fn to_stream_colored(&self, mut stream: &mut dyn WriteColor) {
        impl_stream!(
            |pixel| colored_point_for_pixel(pixel),
            |point_row| {
                point_row.for_each(|point| {
                    let color = match point.color {
                        ColorType::Colored(point_color) => point_color,
                        ColorType::Grayscale => Color::White, //This should be unreachable, but i'll still keep it here to be sure
                    };
                    stream.set_color(ColorSpec::new().set_fg(Some(color))).unwrap();
                    write!(&mut stream, "{}", ascii_char_for_point(point, self.deep, self.invert)).unwrap();
                });
                writeln!(&mut stream, "").unwrap();
            },
            self.image
        );
        stream.flush().unwrap();
    }

    /// Writes the image to `stream`
    pub fn to_stream(&self, mut stream: &mut dyn Write) {
        impl_stream!(
            |pixel| gray_point_for_pixel(pixel),
            |point_row| {
                point_row.for_each(|point| {
                    write!(&mut stream, "{}", ascii_char_for_point(point, self.deep, self.invert)).unwrap();
                });
                writeln!(&mut stream, "").unwrap();
            },
            self.image
        );
        stream.flush().unwrap();
    }
}
