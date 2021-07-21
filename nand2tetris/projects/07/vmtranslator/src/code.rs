use crate::command::{ArithmeticOp, Command, Segment};
use std::io::{BufWriter, Write};

fn segment_to_hack(segment: &Segment) -> String {
    let word = match segment {
        Segment::Argument(_) => "ARG".to_string(),
        Segment::Local(_) => "LCL".to_string(),
        Segment::This(_) => "THIS".to_string(),
        Segment::That(_) => "THAT".to_string(),
        Segment::Pointer(val) => match val {
            0 => "THIS".to_string(),
            1 => "THAT".to_string(),
            _ => "".to_string(),
        },
        Segment::Constant(val) => val.to_string(),
        _ => "".to_string(),
    };

    format!("@{}", word)
}

fn assembly_header() -> String {
    //["@256", "D=A", "@SP", "M=D"].join("\n")
    "".to_string()
}

fn assembly_footer() -> String {
    [
        "(VM_TRANSLATOR_END_LOOP)",
        "@VM_TRANSLATOR_END_LOOP",
        "0;JMP",
    ]
    .join("\n")
}

fn stack_top() -> String {
    ["@SP", "A=M", "A=A-1"].join("\n")
}

fn inc_stack_ptr() -> String {
    ["@SP", "M=M+1"].join("\n")
}

fn dec_stack_ptr() -> String {
    ["@SP", "M=M-1"].join("\n")
}

fn push(segment: &Segment) -> String {
    let at_load = segment_to_hack(segment);
    let push_load = match segment {
        Segment::Constant(_) => [at_load.as_str(), "D=A"],
        _ => [at_load.as_str(), "D=M"],
    }
    .join("\n");

    [
        push_load.as_str(),
        "@SP",
        "A=M",
        "M=D",
        inc_stack_ptr().as_str(),
    ]
    .join("\n")
}

fn pop(segment: &Segment) -> String {
    let at_load = segment_to_hack(segment);
    let pop_load = match segment {
        Segment::Constant(_) => [at_load.as_str(), "D=A"], // Not allowed. Need to clean-up enums definitions.
        _ => [at_load.as_str(), "M=D"],
    }
    .join("\n");

    [
        stack_top().as_str(),
        "D=M",
        pop_load.as_str(),
        "@SP",
        "M=M-1",
    ]
    .join("\n")
}

fn add() -> String {
    [
        stack_top().as_str(),
        "D=M",
        dec_stack_ptr().as_str(),
        stack_top().as_str(),
        "M=M+D",
    ]
    .join("\n")
}

fn arithmetic(op: &ArithmeticOp) -> String {
    match op {
        ArithmeticOp::Add => add(),
        _ => "".to_string(),
    }
}

pub struct CodeWriter<T: Write> {
    writer: BufWriter<T>,
}

impl<T: Write> CodeWriter<T> {
    pub fn new(writer: BufWriter<T>) -> Self {
        let mut writer = CodeWriter { writer };

        writeln!(writer.writer, "{}", assembly_header()).unwrap();

        writer
    }

    pub fn write(&mut self, cmd: &Command) -> std::io::Result<()> {
        let output = match cmd {
            Command::Push(seg) => push(seg),
            Command::Pop(seg) => pop(seg),
            Command::Arithmetic(op) => arithmetic(op),
            _ => "".to_string(),
        };

        writeln!(self.writer, "{}", output)?;
        self.writer.flush()?;
        Ok(())
    }

    pub fn close(&mut self) -> std::io::Result<()> {
        // Write infinite loop.
        let end_loop = assembly_footer();
        writeln!(self.writer, "{}", end_loop)?;

        self.writer.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_iter(cmds: &[Command], expected_body: Option<&str>) {
        let expected = match expected_body {
            Some(body) => format!("{}\n{}\n{}\n", assembly_header(), body, assembly_footer()),
            None => format!("{}\n{}\n", assembly_header(), assembly_footer()),
        };

        let buf_writer = BufWriter::new(Vec::new());
        let mut writer = CodeWriter::new(buf_writer);

        for cmd in cmds {
            writer.write(cmd).unwrap();
        }

        writer.close().unwrap();

        let bytes = writer.writer.into_inner().unwrap();
        let actual = String::from_utf8(bytes).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_end_loop() {
        test_iter(&[], None);
    }

    #[test]
    fn test_push_constant() {
        let cmd = [Command::Push(Segment::Constant(0))];
        let expected = ["@0", "D=A", "@SP", "A=M", "M=D", "@SP", "M=M+1"].join("\n");

        test_iter(&cmd, Some(&expected));
    }

    #[test]
    fn test_push_local() {
        let cmd = [Command::Push(Segment::Local(0))];
        let expected = ["@LCL", "D=M", "@SP", "A=M", "M=D", "@SP", "M=M+1"].join("\n");

        test_iter(&cmd, Some(&expected));
    }

    #[test]
    fn test_pop() {
        let cmd = [Command::Pop(Segment::Local(0))];
        let expected = ["@SP", "A=M", "A=A-1", "D=M", "@LCL", "M=D", "@SP", "M=M-1"].join("\n");

        test_iter(&cmd, Some(&expected));
    }

    #[test]
    fn test_add() {
        let cmd = [Command::Arithmetic(ArithmeticOp::Add)];
        let expected = [
            "@SP", "A=M", "A=A-1", "D=M", "@SP", "M=M-1", "@SP", "A=M", "A=A-1", "M=M+D",
        ]
        .join("\n");

        test_iter(&cmd, Some(&expected));
    }
}
