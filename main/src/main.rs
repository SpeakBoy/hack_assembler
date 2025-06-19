use parser::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = args[1].clone();

    let mut file_parser = Parser::new(file_path);
    file_parser.read_file();
}
