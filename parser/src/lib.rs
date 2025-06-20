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
                    instruction.push_str(&self.decode_a_instruction(line));
                    println!("{instruction}");
                } else {
                    instruction.push_str(&self.decode_c_instruction(line));
                    println!("{instruction}");
                }
            }
        }
    }

    fn decode_a_instruction(&mut self, line: &str) -> String {
        let first_digit = String::from("0");
        let str_len = line.len();
        let the_rest_dec: u16 = line[1..str_len].parse().unwrap();
        let the_rest_bin = self.decimal_to_binary(the_rest_dec);
        first_digit + &the_rest_bin
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
        let first_digits = String::from("111");
        let dest = if line.contains('=') {
            let split_at_equals: Vec<&str> = line.split('=').collect();
            split_at_equals[0]
        } else {
            "null"
        };
        let comp = if line.contains('=') {
            let split_at_equals: Vec<&str> = line.split('=').collect();
            if split_at_equals[1].contains(';') {
                let split_at_semi: Vec<&str> = split_at_equals[1].split(';').collect();
                split_at_semi[0]
            } else {
                split_at_equals[1]
            }
        } else if line.contains(';') {
            let split_at_semi: Vec<&str> = line.split(';').collect();
            if split_at_semi[0].contains('=') {
                let split_at_equals: Vec<&str> = split_at_semi[0].split('=').collect();
                split_at_equals[1]
            } else {
                split_at_semi[0]
            }
        } else {
            "0000000"
        };
        let jump = if line.contains(';') {
            let split_at_semi: Vec<&str> = line.split(';').collect();
            split_at_semi[1]
        } else {
            "null"
        };
        let dest = self.decode_dest(dest);
        let comp = self.decode_comp(comp);
        let jump = self.decode_jump(jump);
        first_digits + &comp + &dest + &jump
    }

    fn decode_dest(&mut self, dest: &str) -> String {
        let mut output = String::new();
        if dest.contains('A') {
            output.push('1');
        } else {
            output.push('0');
        }
        if dest.contains('M') {
            output.push('1');
        } else {
            output.push('0');
        }
        if dest.contains('D') {
            output.push('1');
        } else {
            output.push('0');
        }
        output
    }

    fn decode_comp(&mut self, comp: &str) -> String {
        let output = match comp {
            "0" => "0101010",
            "1" => "0111111",
            "-1" => "0111010",
            "D" => "0001100",
            "A" => "0110000",
            "!D" => "0001101",
            "!A" => "0110001",
            "-D" => "0001111",
            "-A" => "0110011",
            "D+1" => "0011111",
            "A+1" => "0110111",
            "D-1" => "0001110",
            "A-1" => "0110010",
            "D+A" => "0000010",
            "D-A" => "0010011",
            "A-D" => "0000111",
            "D&A" => "0000000",
            "D|A" => "0010101",
            "M" => "1110000",
            "!M" => "1110001",
            "-M" => "1110011",
            "M+1" => "1110111",
            "M-1" => "1110010",
            "D+M" => "1000010",
            "D-M" => "1010011",
            "A-M" => "1000111",
            "D&M" => "1000000",
            "D|M" => "1010101",
            _ => "0000000",
        }
        .to_string();
        output
    }

    fn decode_jump(&mut self, jump: &str) -> String {
        let mut output = String::new();
        if jump == "JMP" || jump.contains('L') || jump.contains("NE") {
            output.push('1');
        } else {
            output.push('0');
        }
        if jump == "JMP" || (jump.contains('E') && !jump.contains("NE")) {
            output.push('1');
        } else {
            output.push('0');
        }
        if jump == "JMP" || jump.contains('G') || jump.contains("NE") {
            output.push('1');
        } else {
            output.push('0');
        }
        output
    }
}
