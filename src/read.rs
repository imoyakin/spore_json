use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::path::Path;

pub fn ReadJson(filepath: &String) {
    let mut path = Path::new(&filepath);
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open{}:{}",display, Error::description(&why)),
        Ok(file) =>file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't open{}:{}",display, Error::description(&why)),
        Ok(_) => println!("{} contains:\n{}", display, s),
    }
}

pub fn DownloadJson() {}

pub fn IncomingJson() {}