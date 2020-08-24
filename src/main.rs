extern crate structopt;
extern crate asciify;

use asciify::AsciiBuilder;
use std::path::PathBuf;
use structopt::StructOpt;


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

fn main() {
    let opt = Opt::from_args();
    let path = opt.input;
    let deep = opt.deep;
    let invert = opt.invert;

    let mut builder: AsciiBuilder = AsciiBuilder::new(path);
    builder.set_deep(deep);
    builder.set_invert(invert);

    if let Some(dimensions) = opt.resize {
        if dimensions.len() != 2 {
            panic!("Must provide exactly two numbers to resize for width and height. Provided {} arguments", dimensions.len());
        }
        builder.set_resize((dimensions[0], dimensions[1]));   
    }

    println!("{}", builder.build());
}
