use crate::parse;

use std::{fs::File, io::Result};
use bmp::{px, Image, Pixel};

const WIDTH_LARGE: u32 = 128;
// const WIDTH_SMALL: u32 = 32;
const HEIGHT_LARGE: u32 = 112;
// const HEIGHT_SMALL: u32 = 32;

const PIXELS: [Pixel; 4] = [
    px!(0xff, 0xff, 0xff),
    px!(0xaa, 0xaa, 0xaa),
    px!(0x55, 0x55, 0x55),
    px!(0, 0, 0),
];

type Row = Vec<Pixel>;

#[derive(Debug, Hash)]
pub struct Photo {
    pub large: Vec<u8>,
    pub small: Vec<u8>,
    pub info: Vec<u8>,
}

#[derive(Debug)]
struct Tile {
    idx: u8,
    rows: Vec<Row>,
}

pub fn save_photo(file: &File, idx: usize, out_filename: String) -> Result<()> {
    let photo = parse::read_photo(file, idx)?;
    let img = create_image(photo);
    img.save(out_filename)
}

fn create_image(photo: Photo) -> Image {
    let tiles = get_tiles(photo.large);
    let mut img = Image::new(WIDTH_LARGE, HEIGHT_LARGE);

    for tile in &tiles {
        write_tile(&mut img, tile);
    }

    img
}

fn write_tile(img: &mut Image, tile: &Tile) {
    let tile_col: u32 = u32::from(tile.idx) % 16;
    let tile_row: u32 = u32::from(tile.idx) / 16;
    let x_offset = tile_col * 8;
    let y_offset = tile_row * 8;

    for y in 0..(tile.rows.len()) {
        let row = &tile.rows[y];
        for (x, &pixel) in row.iter().enumerate() {
            img.set_pixel(x_offset + x as u32, y_offset + y as u32, pixel);
        }
    }
}

fn get_tiles(data: Vec<u8>) -> Vec<Tile> {
    // x >> 3 is equivalent to x / 8
    let num_tiles = (WIDTH_LARGE >> 3) * (HEIGHT_LARGE >> 3);
    (0..num_tiles)
        .map(|i| {
            let tile_offset = (0x10 * i) as usize;
            let next_offset = tile_offset + 0x10;
            Tile {
                idx: i as u8,
                rows: get_rows(&data[tile_offset..next_offset]),
            }
        })
        .collect()
}

fn get_rows(data: &[u8]) -> Vec<Row> {
    // 8 rows.. (2 bytes per row)
    (0..16)
        .step_by(2)
        .map(|i| {
            let b1 = data[i];
            let b2 = data[i + 1];
            // ..of 8 pixels
            (0..8).map(|j| get_pixel(b1, b2, j)).collect()
        })
        .collect()
}

fn get_pixel(b1: u8, b2: u8, idx: u8) -> Pixel {
    let mask = 0x80 >> idx;
    let pixel_idx = match (b1 & mask != 0, b2 & mask != 0) {
        (false, false) => 0,
        (true, false) => 1,
        (false, true) => 2,
        (true, true) => 3,
    };
    PIXELS[pixel_idx]
}
