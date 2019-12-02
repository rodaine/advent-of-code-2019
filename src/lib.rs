#![allow(dead_code)]

use std::io::BufReader;
use std::fs::File;
use std::path::PathBuf;

mod dec01;

fn read_input(name: &str) -> BufReader<File> {
    let mut path = PathBuf::from("inputs");
    path.push(name);

    let file = File::open(&path)
        .unwrap_or_else(|err| panic!("unable to open input file {:?}: {}", path, err));

    BufReader::new(file)
}