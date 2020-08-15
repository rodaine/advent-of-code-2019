#![allow(dead_code)]

use std::io::BufReader;
use std::fs::File;
use std::path::PathBuf;

mod computer;

mod dec01;
mod dec02;
mod dec03;
mod dec04;
mod dec05;
mod dec06;
mod dec07;

fn input_path(name: &str) -> PathBuf {
    let mut path = PathBuf::from("inputs");
    path.push(name);
    path
}

fn read_input(name: &str) -> BufReader<File> {
    let path = input_path(name);
    File::open(&path)
        .map(BufReader::new)
        .unwrap_or_else(|err| panic!("unable to open input file {:?}: {}", path, err))
}

fn read_to_string(name: &str) -> String {
    let path = input_path(name);
    ::std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("unable to read input file {:?}: {}", path, err))
}