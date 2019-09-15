use std::fs;
use std::io;
use std::io::{SeekFrom};
use std::io::prelude::*;

fn main() {
    let mut fp = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open("test.txt")
        .expect("cannot create file");
    
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line);
    fp.write_all(b"1");
    stdin.lock().read_line(&mut line);
    fp.write_all(b"2");
    stdin.lock().read_line(&mut line);
    fp.write_all(b"3");
    stdin.lock().read_line(&mut line);
    fp.seek(SeekFrom::Start(0));
    fp.write_all(b"4");
    stdin.lock().read_line(&mut line);
    fp.write_all(b"5");
    stdin.lock().read_line(&mut line);
}
