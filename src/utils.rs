#![allow(dead_code)]
#![allow(unused_variables)]
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

// TODO fix this
pub fn read_lines<T: AsRef<Path>>(file_path: T) -> io::Result<Vec<u32>> {
    //let content = read_to_string(file_path)?;
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let lines: Vec<u32> = reader
        .lines()
        .map(|lin| lin.unwrap().parse().unwrap())
        .collect();
    Ok(lines)
}

pub fn read_lines_true<T: AsRef<Path>>(file_path: T) -> io::Result<Vec<String>> {
    //let content = read_to_string(file_path)?;
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|lin| lin.unwrap()).collect();
    Ok(lines)
}
