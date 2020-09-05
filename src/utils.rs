use std::fs;
use std::fs::File;
use std::io::Read;
use std::path;

pub fn get_file_as_byte_vec(file_path: &path::Path) -> Vec<u8> {
    let mut file = File::open(file_path).expect("No file found");
    let metadata = fs::metadata(file_path).expect("Unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    file.read_exact(&mut buffer).expect("Buffer overflow");

    buffer
}

macro_rules! zip {
    ($x: expr) => ($x);
    ($x: expr, $($y: expr), +) => (
        $x.zip(zip!($($y), +))
    )
}
