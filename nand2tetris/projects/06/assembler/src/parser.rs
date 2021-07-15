use lazy_static::lazy_static;
use regex::Regex;

#[derive(Copy, Clone, PartialEq)]
pub enum InstructionType {
    A,
    C,
    L,
}

fn is_comment(line: &str) -> bool {
    line.starts_with("//")
}

/// Determines if a line has an effect on the program.
fn functional(line: &str) -> bool {
    !line.is_empty() && !is_comment(line)
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

fn strip_lines(lines: &[String]) -> Vec<String> {
    lines
        .iter()
        .map(|x| x.trim())
        .filter(|x| functional(x))
        .map(|x| strip_trailing_comment(x))
        .collect()
}

#[derive(Debug, PartialEq)]
struct CInstruction {
    dest: String,
    comp: String,
    jump: String,
}

impl CInstruction {
    fn new(raw_str: &str) -> CInstruction {
        lazy_static! {
            static ref RE: Regex = Regex::new("((.*)=)?([^;]*)(;(.*))?").unwrap();
        }

        let captures = RE.captures(raw_str).unwrap();

        CInstruction {
            dest: match captures.get(2) {
                Some(cap) => cap.as_str().to_string(),
                None => "".to_string(),
            },
            comp: match captures.get(3) {
                Some(cap) => cap.as_str().to_string(),
                None => "".to_string(),
            },
            jump: match captures.get(5) {
                Some(cap) => cap.as_str().to_string(),
                None => "".to_string(),
            },
        }
    }
}

pub struct Parser {
    lines: Vec<String>,
    current_line_index: usize,
    instruct_type: InstructionType,
    curr_c_instruct: Option<CInstruction>,
}

impl Parser {
    pub fn new(lines: Vec<String>) -> Parser {
        let lines = strip_lines(&lines);
        let mut parser = Parser {
            lines,
            current_line_index: 0,
            instruct_type: InstructionType::A,
            curr_c_instruct: None,
        };

        parser.parse_current_line();

        parser
    }

    pub fn reset(&mut self) {
        self.current_line_index = 0;
        self.parse_current_line();
    }

    pub fn has_more_lines(&self) -> bool {
        self.current_line_index < self.lines.len()
    }

    fn parse_current_line(&mut self) {
        if self.lines.len() - 1 < self.current_line_index {
            return;
        }
        let line = self.get_current_line();
        match line.chars().next().unwrap() {
            '@' => {
                self.instruct_type = InstructionType::A;
                self.curr_c_instruct = None;
            }
            '(' => {
                self.instruct_type = InstructionType::L;
                self.curr_c_instruct = None;
            }
            _ => {
                self.instruct_type = InstructionType::C;
                self.curr_c_instruct = Some(CInstruction::new(&line));
            }
        }
    }

    pub fn advance(&mut self) {
        self.current_line_index += 1;
        self.parse_current_line();
    }

    pub fn instruction_type(&self) -> InstructionType {
        self.instruct_type
    }

    fn get_current_line(&self) -> String {
        self.lines[self.current_line_index].to_string()
    }

    pub fn symbol(&self) -> String {
        let line = self.get_current_line();
        let chars = line.chars().collect::<Vec<char>>();
        match self.instruct_type {
            InstructionType::A => chars[1..].iter().collect::<String>(),
            InstructionType::L => chars[1..chars.len() - 1].iter().collect::<String>(),
            _ => "".to_string(),
        }
    }

    pub fn dest(&self) -> String {
        match &self.curr_c_instruct {
            Some(instruct) => instruct.dest.clone(),
            None => "".to_string(),
        }
    }

    pub fn comp(&self) -> String {
        match &self.curr_c_instruct {
            Some(instruct) => instruct.comp.clone(),
            None => "".to_string(),
        }
    }

    pub fn jump(&self) -> String {
        match &self.curr_c_instruct {
            Some(instruct) => instruct.jump.clone(),
            None => "".to_string(),
        }
    }

    pub fn current_line(&self) -> usize {
        self.current_line_index
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c_instruct() {
        let input = "D=M";
        let expected = CInstruction {
            dest: "D".to_string(),
            comp: "M".to_string(),
            jump: "".to_string(),
        };

        let actual = CInstruction::new(input);

        assert_eq!(expected, actual);

        let input = "0;JMP";
        let expected = CInstruction {
            dest: "".to_string(),
            comp: "0".to_string(),
            jump: "JMP".to_string(),
        };

        let actual = CInstruction::new(input);

        assert_eq!(expected, actual);
    }
}