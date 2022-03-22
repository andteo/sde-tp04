use std::{fs, env};
use std::os::unix::fs::MetadataExt;
use std::os::unix::io::AsRawFd;
use std::io::Read;


fn main() {
    let args: Vec<String> = env::args().collect();

    let folder = &args[1];
    
    let paths = fs::read_dir(folder).unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
}
