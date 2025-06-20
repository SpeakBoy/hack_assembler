use parser::*;
use std::env;
use std::fs::File;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = args[1].clone();
    let file_split: Vec<&str> = file_path.split('.').collect();
    let file_name = file_split[0].to_string();

    let mut file_parser = Parser::new(file_path);
    let binary_code = file_parser.assemble();

    let hack_file_path = file_name + ".hack";
    let mut hack_file = File::create(hack_file_path).expect("creation failed");
    hack_file
        .write(&binary_code.as_bytes())
        .expect("write failed");
}
