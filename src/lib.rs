use std::io::{self, BufReader, Read};

use data_formats::{read_scout_file, ScoutFile};

mod data_formats;

pub fn read(mut input: impl Read) -> Result<ScoutFile, io::Error> {
    read_scout_file(&mut input)
}

pub fn read_from_file(file_name: &str) -> Result<ScoutFile, io::Error> {
    let file = std::fs::File::open(file_name)?;
    let mut buffer = BufReader::new(file);
    read(&mut buffer)
}
