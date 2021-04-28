#![allow(dead_code)]
#![allow(unused_variables)]
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub fn read_lines<T: AsRef<Path>>(file_path: T) -> io::Result<Vec<String>> {
    //let content = read_to_string(file_path)?;
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|lin| lin.unwrap()).collect();
    Ok(lines)
}
