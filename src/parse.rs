use crate::img::Photo;

use std::{
    fs::File,
    io::{Read, Result, Seek, SeekFrom},
};

const PHOTO_OFFSET: usize = 0x2000;
const PHOTO_SIZE: usize = 0x1000;

const SIZE_LARGE: usize = 0xe00;
const SIZE_SMALL: usize = 0x100;
const SIZE_INFO: usize = 0x100;

pub fn read_photo(mut file: &File, idx: usize) -> Result<Photo> {
    let pos = PHOTO_OFFSET + (PHOTO_SIZE * idx);

    let mut large = vec![0; SIZE_LARGE];
    let mut small = vec![0; SIZE_SMALL];
    let mut info = vec![0; SIZE_INFO];

    file.seek(SeekFrom::Start(pos as u64))?;
    file.read_exact(&mut large)?;
    file.read_exact(&mut small)?;
    file.read_exact(&mut info)?;

    Ok(Photo { large, small, info })
}
