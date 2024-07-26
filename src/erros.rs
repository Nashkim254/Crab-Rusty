use std::{fs::File, io::ErrorKind};

pub fn run() {
    // panic!("Program crash");

    //handle errors gracefully
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    let f = File::open("hello.txt").unwrap();
    let f = File::open("hello.txt").expect("Failed to open file");

    let f = match f {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(er) => panic!("Error {:?}", er),
            },
            other_error => panic!("Error {:?}", other_error),
        },
    };
}
