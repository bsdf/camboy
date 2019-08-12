mod image;
mod parse;

use std::{fs::File, io::Result, path::PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "CAMBOY")]
/// camboy's about string maybbe
struct Opt {
    #[structopt(name = "picture number", short = "i", long = "index")]
    /// dump a single picture, indexed by 0.
    picture_index: Option<usize>,

    #[structopt(
        name = "format",
        short = "o",
        long = "out",
        default_value = "picture-%"
    )]
    /// filename output format. the '%' character will be replaced with the picture number.
    output_format: String,

    #[structopt(name = "INPUT", parse(from_os_str))]
    /// the gameboy camera save file dump.
    input_file: PathBuf,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let file = File::open(&opt.input_file).expect("could not open file.");

    match opt.picture_index {
        Some(idx) => {
            // user provided a picture index
            image::save_photo(&file, idx, get_filename(&opt.output_format, idx))
        }
        None => {
            // no picture index, dump all pictures
            (0..30)
                .map(|i| image::save_photo(&file, i, get_filename(&opt.output_format, i)))
                .collect()
        }
    }
}

fn get_filename(format: &str, idx: usize) -> String {
    format.replace("%", &format!("{:02}", idx)) + ".bmp"
}
