use std::fs::*;
use std::path::PathBuf;

use std::io;
use std::io::Read;

type IOError = io::Error;

pub fn read(path: &str) -> Result<String, IOError> {
    let abs_path = PathBuf::from(path);

    let real_path = try!(canonicalize(&abs_path));
    let mut file = try!(File::open(real_path));

    let mut raw = String::new();
    file.read_to_string(&mut raw).and(Ok(raw))
}
