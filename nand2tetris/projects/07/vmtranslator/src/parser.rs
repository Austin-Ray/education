use crate::command::{Command, Segment};
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

fn parse_seg(seg_str: &str) -> Segment {
    match seg_str {
        "argument" => Segment::Argument,
        "local" => Segment::Local,
        "static" => Segment::Static,
        _ => Segment::Constant(seg_str.parse().unwrap()),
    }
}

fn parse_cmd(clean_line: &str) -> Command {
    let tokens = clean_line.split_whitespace().collect::<Vec<&str>>();

    match tokens[0] {
        "push" => Command::Push(parse_seg(tokens[1]), tokens[2].parse().unwrap()),
        "pop" => Command::Pop(parse_seg(tokens[1]), tokens[2].parse().unwrap()),
        _ => Command::Arithmetic(clean_line.to_string()),
    }
}

struct Parser<T: BufRead> {
    lines: Lines<T>,
    cur_cmd: Option<Command>,
    more_lines: bool,
}

impl<T: BufRead> Parser<T> {
    fn new(lines: Lines<T>) -> Parser<T> {
        let mut parser = Parser {
            lines,
            cur_cmd: None,
            more_lines: true,
        };

        parser.advance();

        parser
    }

    fn has_more_lines(&self) -> bool {
        self.more_lines
    }

    fn advance(&mut self) {
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
        let temp_cmd = parse_cmd(&clean_line);

        self.cur_cmd = Some(temp_cmd);
    }

    fn command(&self) -> &Option<Command> {
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
            let parser = Parser::new(reader.lines());
            assert_eq!(&test_case.expected, parser.command().as_ref().unwrap())
        }
    }

    #[test]
    fn test_parse_arithmetic_op() {
        let test_cases = vec![
            TestCase {
                input_str: "add".to_string(),
                expected: Command::Arithmetic("add".to_string()),
            },
            TestCase {
                input_str: "sub".to_string(),
                expected: Command::Arithmetic("sub".to_string()),
            },
            TestCase {
                input_str: "neg".to_string(),
                expected: Command::Arithmetic("neg".to_string()),
            },
            TestCase {
                input_str: "eq".to_string(),
                expected: Command::Arithmetic("eq".to_string()),
            },
            TestCase {
                input_str: "gt".to_string(),
                expected: Command::Arithmetic("gt".to_string()),
            },
            TestCase {
                input_str: "lt".to_string(),
                expected: Command::Arithmetic("lt".to_string()),
            },
            TestCase {
                input_str: "and".to_string(),
                expected: Command::Arithmetic("and".to_string()),
            },
            TestCase {
                input_str: "or".to_string(),
                expected: Command::Arithmetic("or".to_string()),
            },
            TestCase {
                input_str: "not".to_string(),
                expected: Command::Arithmetic("not".to_string()),
            },
        ];

        test_iter(&test_cases);
    }

    #[test]
    fn test_push_pop() {
        let test_cases = vec![
            TestCase {
                input_str: "pop argument 0".to_string(),
                expected: Command::Pop(Segment::Argument, 0),
            },
            TestCase {
                input_str: "push argument 0".to_string(),
                expected: Command::Push(Segment::Argument, 0),
            },
            TestCase {
                input_str: "push local 0".to_string(),
                expected: Command::Push(Segment::Local, 0),
            },
            TestCase {
                input_str: "push 0 0".to_string(),
                expected: Command::Push(Segment::Constant(0), 0),
            },
        ];

        test_iter(&test_cases);
    }
}
