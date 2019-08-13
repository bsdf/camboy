mod img;
mod parse;

use std::{fs::File, io::Result, path::PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "CAM BOY", author = "")]
/// CAM BOY takes a GAME BOY camera save file dump and extracts the images from it.
struct Opt {
    #[structopt(name = "picture number", short = "i", long = "index")]
    /// dump a single picture, indexed by 0.
    picture_index: Option<usize>,

    #[structopt(
        name = "format",
        short = "o",
        long = "out",
        default_value = "CAMBOY%"
    )]
    /// filename output format. the '%' character will be replaced with the picture number.
    output_format: String,

    #[structopt(name = "INPUT", parse(from_os_str))]
    /// the GAME BOY camera save file dump.
    input_file: PathBuf,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let file = File::open(&opt.input_file).expect("could not open file.");

    match opt.picture_index {
        Some(idx) => {
            // user provided a picture index
            img::save_photo(&file, idx, get_filename(&opt.output_format, idx))
        }
        None => {
            // no picture index, dump all pictures
            (0..30)
                .map(|i| img::save_photo(&file, i, get_filename(&opt.output_format, i)))
                .collect()
        }
    }
}

fn get_filename(format: &str, idx: usize) -> String {
    format.replace("%", &format!("{:02}", idx)) + ".png"
}
