use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("{file_path}");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let lines = contents.lines();

    for line in lines {
        println!("{line}");
    }
}
