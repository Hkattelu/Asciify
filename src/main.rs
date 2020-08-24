extern crate structopt;
extern crate asciify;

use asciify::AsciiBuilder;
use std::path::PathBuf;
use structopt::StructOpt;


#[derive(Debug, StructOpt)]
#[structopt(name = "asciify", about = "Convert images to ascii text")]
struct Opt {
    /// Use a deeper selection of ascii characters
    #[structopt(short, long)]
    deep: bool,

    /// Use the image color when printing to console
    #[structopt(short, long)]
    color: bool,

    /// Invert the opacity of the image
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
    let mut builder: AsciiBuilder = AsciiBuilder::new(opt.input);
    builder.set_deep(opt.deep);
    builder.set_invert(opt.invert);

    if let Some(dimensions) = opt.resize {
        if dimensions.len() != 2 {
            panic!("Must provide exactly two numbers to resize for width and height. Provided {} arguments", dimensions.len());
        }
        builder.set_resize((dimensions[0], dimensions[1]));   
    }

    builder.to_std_out(opt.color);
}
