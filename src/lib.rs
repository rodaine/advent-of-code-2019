use std::io::BufReader;
use std::fs::File;
use std::path::PathBuf;

mod dec1;

fn read_input(name: &str) -> BufReader<File> {
    let mut path = PathBuf::from("inputs");
    path.push(name);

    let file = File::open(&path)
        .expect(&format!("unable to open input file: {:?}", path));

    BufReader::new(file)
}