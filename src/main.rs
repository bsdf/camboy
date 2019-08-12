mod parse;
mod image;

use std::{fs::File, io::Result};
use clap::{App, Arg, ArgMatches};

const ARG_SAVE_FILE: &'static str = "save_file";
const ARG_OUTPUT_FMT: &'static str = "output";
const ARG_PICTURE_IDX: &'static str = "picture_index";

fn main() -> Result<()> {
    let args = parse_args();
    let filename = args.value_of(ARG_SAVE_FILE).unwrap();
    let out_fmt = args.value_of(ARG_OUTPUT_FMT).unwrap();
    let picture_idx = args.value_of(ARG_PICTURE_IDX);

    let file = File::open(filename).expect("could not open file.");

    match picture_idx {
        Some(idx) => {
            // user provided a picture index
            let idx: usize = usize::from_str_radix(idx, 10).unwrap();
            image::save_photo(&file, idx, get_filename(&out_fmt, idx))
        }
        None => {
            // no picture index, dump all pictures
            (0..30)
                .map(|i| image::save_photo(&file, i, get_filename(&out_fmt, i)))
                .collect()
        }
    }
}

fn parse_args<'a>() -> ArgMatches<'a> {
    App::new("gbcam parser")
        .version("1.0")
        .author("me")
        .about("parses gbc camera save file")
        .arg(
            Arg::with_name(ARG_SAVE_FILE)
                .value_name("INPUT")
                .help("the gameboy camera save file dump")
                .required(true)
                .index(1))
        .arg(
            Arg::with_name(ARG_PICTURE_IDX)
                .short("i")
                .long("index")
                .value_name("picture #")
                .help("dump a single picture, indexed by 0.")
                .takes_value(true))
        .arg(
            Arg::with_name(ARG_OUTPUT_FMT)
                .short("o")
                .long("out")
                .default_value("picture-%"))
        .get_matches()
}

fn get_filename(format: &str, idx: usize) -> String {
    format.replace("%", &format!("{:02}", idx)) + ".bmp"
}
