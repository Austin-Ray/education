use crate::instruction::Instruction;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::{BufReader, Lines};

fn is_comment(line: &str) -> bool {
    line.starts_with("//")
}

/// Determines if a line has no effect on the program.
fn superficial(line: &str) -> bool {
    line.is_empty() || is_comment(line)
}

fn strip_trailing_comment(line: &str) -> String {
    match line.find("//") {
        Some(offset) => {
            let mut new_line = line.to_string();
            new_line.replace_range(offset.., "");
            new_line.trim().to_string()
        }
        None => line.to_string(),
    }
}

fn parse_c_instruction(raw_str: &str) -> Instruction {
    lazy_static! {
        static ref RE: Regex = Regex::new("((.*)=)?([^;]*)(;(.*))?").unwrap();
    }

    let captures = RE.captures(raw_str).unwrap();

    Instruction::C {
        dest: captures.get(2).map(|cap| cap.as_str().to_string()),
        comp: match captures.get(3) {
            Some(cap) => cap.as_str().to_string(),
            None => "".to_string(),
        },
        jump: captures.get(5).map(|cap| cap.as_str().to_string()),
    }
}

fn parse_instruction(line: &str) -> Instruction {
    let chars = line.chars().collect::<Vec<char>>();

    match chars[0] {
        '@' => {
            let symbol = chars[1..].iter().collect::<String>();
            match symbol.parse::<i32>() {
                Ok(num) => Instruction::AConst(num),
                Err(_) => Instruction::AVar(symbol.to_string()),
            }
        }
        '(' => Instruction::L(chars[1..chars.len() - 1].iter().collect::<String>()),
        _ => parse_c_instruction(line),
    }
}

pub struct Parser {
    lines: Lines<BufReader<File>>,
    curr_line_idx: usize,
    curr_inst: Option<Instruction>,
    has_more_lines: bool,
}

impl Parser {
    pub fn new(lines: Lines<BufReader<File>>) -> Parser {
        Parser {
            lines,
            curr_line_idx: 0,
            curr_inst: None,
            has_more_lines: true,
        }
    }

    pub fn has_more_lines(&self) -> bool {
        self.has_more_lines
    }

    pub fn advance(&mut self) {
        let mut invalid = true;
        let mut curr_line = String::new();

        while invalid {
            match self.lines.next() {
                Some(line) => {
                    curr_line = line.unwrap();
                    invalid = superficial(&curr_line);
                }
                None => {
                    self.has_more_lines = false;
                    self.curr_inst = None;
                    return;
                }
            };
        }

        let clean_line = strip_trailing_comment(&curr_line);
        let temp_inst = parse_instruction(&clean_line);

        match temp_inst {
            Instruction::L(_) => {}
            _ => self.curr_line_idx += 1,
        };

        self.curr_inst = Some(temp_inst);
    }

    pub fn get_current_instruction(&self) -> &Option<Instruction> {
        &self.curr_inst
    }

    pub fn current_line_number(&self) -> usize {
        self.curr_line_idx
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c_instruct() {
        let input = "D=M";
        let expected = Instruction::C {
            dest: Some("D".to_string()),
            comp: "M".to_string(),
            jump: None,
        };

        let actual = parse_instruction(input);

        assert_eq!(expected, actual);

        let input = "0;JMP";
        let expected = Instruction::C {
            dest: None,
            comp: "0".to_string(),
            jump: Some("JMP".to_string()),
        };

        let actual = parse_instruction(input);

        assert_eq!(expected, actual);
    }
}
