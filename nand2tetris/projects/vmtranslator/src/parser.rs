use crate::command::{ArithmeticOp, Command, Segment};
use std::io::{BufRead, Lines};

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

fn parse_seg(seg_str: &str, val: i16, file_name: &str) -> Segment {
    match seg_str {
        "argument" => Segment::Argument(val),
        "local" => Segment::Local(val),
        "static" => Segment::Static(file_name.to_string(), val),
        "this" => Segment::This(val),
        "that" => Segment::That(val),
        "temp" => Segment::Temp(val),
        "pointer" => Segment::Pointer(val),
        _ => Segment::Constant(val),
    }
}

fn parse_arithmetic_op(line: &str) -> ArithmeticOp {
    match line {
        "add" => ArithmeticOp::Add,
        "sub" => ArithmeticOp::Subtract,
        "neg" => ArithmeticOp::Negate,
        "eq" => ArithmeticOp::Equal,
        "gt" => ArithmeticOp::GreaterThan,
        "lt" => ArithmeticOp::LessThan,
        "and" => ArithmeticOp::And,
        "or" => ArithmeticOp::Or,
        "not" => ArithmeticOp::Not,
        _ => ArithmeticOp::Equal,
    }
}

fn parse_cmd(clean_line: &str, file_name: &str) -> Command {
    let tokens = clean_line.split_whitespace().collect::<Vec<&str>>();

    match tokens[0] {
        "push" => Command::Push(parse_seg(tokens[1], tokens[2].parse().unwrap(), file_name)),
        "pop" => Command::Pop(parse_seg(tokens[1], tokens[2].parse().unwrap(), file_name)),
        "label" => Command::Label(tokens[1].to_string()),
        "goto" => Command::Goto(tokens[1].to_string()),
        "if-goto" => Command::IfGoto(tokens[1].to_string()),
        "call" => Command::Call(tokens[1].to_string(), tokens[2].parse().unwrap()),
        "function" => Command::Function(tokens[1].to_string(), tokens[2].parse().unwrap()),
        "return" => Command::Return,
        _ => Command::Arithmetic(parse_arithmetic_op(clean_line)),
    }
}

pub struct Parser<T: BufRead> {
    lines: Lines<T>,
    cur_cmd: Option<Command>,
    more_lines: bool,
    file_name: String,
}

impl<T: BufRead> Parser<T> {
    pub fn new(lines: Lines<T>, file: String) -> Parser<T> {
        let mut parser = Parser {
            lines,
            cur_cmd: None,
            more_lines: true,
            file_name: file,
        };

        parser.advance();

        parser
    }

    pub fn has_more_lines(&self) -> bool {
        self.more_lines
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
                    self.more_lines = false;
                    self.cur_cmd = None;
                    return;
                }
            };
        }

        let clean_line = strip_trailing_comment(&curr_line);
        let temp_cmd = parse_cmd(&clean_line, &self.file_name);

        self.cur_cmd = Some(temp_cmd);
    }

    pub fn command(&self) -> &Option<Command> {
        &self.cur_cmd
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufRead, BufReader};

    struct TestCase {
        input_str: String,
        expected: Command,
    }

    fn test_iter(cases: &[TestCase]) {
        for test_case in cases {
            let reader = BufReader::new(test_case.input_str.as_bytes());
            let parser = Parser::new(reader.lines(), "test".to_string());
            assert_eq!(&test_case.expected, parser.command().as_ref().unwrap())
        }
    }

    #[test]
    fn test_parse_arithmetic_op() {
        let test_cases = vec![
            TestCase {
                input_str: "add".to_string(),
                expected: Command::Arithmetic(ArithmeticOp::Add),
            },
            TestCase {
                input_str: "sub".to_string(),
                expected: Command::Arithmetic(ArithmeticOp::Subtract),
            },
            TestCase {
                input_str: "neg".to_string(),
                expected: Command::Arithmetic(ArithmeticOp::Negate),
            },
            TestCase {
                input_str: "eq".to_string(),
                expected: Command::Arithmetic(ArithmeticOp::Equal),
            },
            TestCase {
                input_str: "gt".to_string(),
                expected: Command::Arithmetic(ArithmeticOp::GreaterThan),
            },
            TestCase {
                input_str: "lt".to_string(),
                expected: Command::Arithmetic(ArithmeticOp::LessThan),
            },
            TestCase {
                input_str: "and".to_string(),
                expected: Command::Arithmetic(ArithmeticOp::And),
            },
            TestCase {
                input_str: "or".to_string(),
                expected: Command::Arithmetic(ArithmeticOp::Or),
            },
            TestCase {
                input_str: "not".to_string(),
                expected: Command::Arithmetic(ArithmeticOp::Not),
            },
        ];

        test_iter(&test_cases);
    }

    #[test]
    fn test_push_pop() {
        let test_cases = vec![
            TestCase {
                input_str: "pop argument 0".to_string(),
                expected: Command::Pop(Segment::Argument(0)),
            },
            TestCase {
                input_str: "push argument 0".to_string(),
                expected: Command::Push(Segment::Argument(0)),
            },
            TestCase {
                input_str: "push local 0".to_string(),
                expected: Command::Push(Segment::Local(0)),
            },
            TestCase {
                input_str: "push constant 0".to_string(),
                expected: Command::Push(Segment::Constant(0)),
            },
        ];

        test_iter(&test_cases);
    }
}
