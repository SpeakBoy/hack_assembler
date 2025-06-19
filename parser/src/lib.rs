use std::collections::HashMap;
use std::fs;

pub struct Parser {
    file_path: String,
    symbol_table: HashMap<String, String>,
}

impl Parser {
    pub fn new(file_path: String) -> Parser {
        Parser {
            file_path,
            symbol_table: HashMap::new(),
        }
    }

    pub fn read_file(&mut self) {
        let contents =
            fs::read_to_string(&self.file_path).expect("Should have been able to read the file");

        let lines = contents.lines();

        for line in lines {
            let blank = line.trim().is_empty();
            let comment = line.contains('/');
            let mut instruction = String::new();
            if !blank && !comment {
                let first_char = line.chars().nth(0).expect("Out of range");
                if first_char == '@' {
                    instruction.push_str(&(self.decode_a_instruction(line)));
                    println!("{instruction}");
                } else {
                    println!("{line}");
                }
            }
        }
    }

    fn decode_a_instruction(&mut self, line: &str) -> String {
        let first_digit = String::from("0");
        let str_len = line.len();
        let the_rest: u16 = line[1..str_len].parse().unwrap();
        let the_rest = self.decimal_to_binary(the_rest);
        first_digit + &the_rest
    }

    fn decimal_to_binary(&mut self, mut num: u16) -> String {
        let mut output = String::new();
        loop {
            if num % 2 == 1 {
                output = String::from("1") + &output;
            } else {
                output = String::from("0") + &output;
            }
            num /= 2;
            if num == 0 {
                break;
            }
        }

        let diff = 15 - output.len();
        let zero_padding = "0".repeat(diff);
        let output = zero_padding + &output;
        output
    }

    fn decode_c_instruction(&mut self, line: &str) -> String {
        let mut first_digits = String::from("111");

        first_digits
    }
}
