use std::fs::File;
use std::io::prelude::*;
use std::io;

pub fn read_input(path: &str) -> io::Result<String> {
    let mut file = File::open(format!("examples/input/{}.txt", path))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
