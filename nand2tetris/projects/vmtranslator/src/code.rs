use crate::command::{ArithmeticOp, Command, Segment};
use crate::instruct::Instruction;
use std::io::{BufWriter, Write};

fn instruct_vec_str(instructs: &[Instruction]) -> String {
    instructs
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("\n")
}

fn a_const(val: &i16) -> Instruction {
    Instruction::AConst(*val)
}

fn a_sym(sym: &str) -> Instruction {
    Instruction::ASymbolic(sym.to_string())
}

fn addr_assign(lhs: &str, rhs: &str) -> Instruction {
    Instruction::C(Some(lhs.to_string()), rhs.to_string(), None)
}

fn jmp(comp: &str, jump: &str) -> Instruction {
    Instruction::C(None, comp.to_string(), Some(jump.to_string()))
}

fn jmp_no_cond() -> Instruction {
    jmp("0", "JMP")
}

fn label(label: &str) -> Instruction {
    Instruction::Label(label.to_string())
}

fn segment_offset_template(base_reg: &str, offset: &i16) -> Vec<Instruction> {
    vec![a_const(offset), addr_assign("D", "A"), a_sym(base_reg)]
}

fn segment_with_offset(base_reg: &str, offset: &i16) -> Vec<Instruction> {
    let mut instructs = vec![];

    instructs.append(&mut segment_offset_template(base_reg, offset));
    instructs.push(addr_assign("A", "D+A"));

    instructs
}

fn segment_with_offset_ptr(base_reg: &str, offset: &i16) -> Vec<Instruction> {
    let mut instructs = vec![];

    instructs.append(&mut segment_offset_template(base_reg, offset));
    instructs.push(addr_assign("A", "D+M"));

    instructs
}

fn segment_to_hack(segment: &Segment) -> Vec<Instruction> {
    match segment {
        Segment::Argument(offset) => segment_with_offset_ptr("ARG", offset),
        Segment::Local(offset) => segment_with_offset_ptr("LCL", offset),
        Segment::This(offset) => segment_with_offset_ptr("THIS", offset),
        Segment::That(offset) => segment_with_offset_ptr("THAT", offset),
        Segment::Static(file, offset) => vec![a_sym(&format!("{}.{}", file, offset))],
        Segment::Temp(offset) => segment_with_offset("R5", offset),
        Segment::Pointer(val) => vec![match val {
            0 => a_sym("THIS"),
            1 => a_sym("THAT"),
            _ => a_sym(""),
        }],
        Segment::Constant(val) => vec![a_const(val)],
        Segment::Named(name) | Segment::NamedPtr(name) => vec![a_sym(name)],
    }
}

fn assembly_header() -> Vec<Instruction> {
    vec![
        a_const(&256),
        addr_assign("D", "A"),
        a_sym("SP"),
        addr_assign("M", "D"),
        a_sym("Sys.init"),
        jmp_no_cond(),
    ]
}

fn assembly_footer() -> Vec<Instruction> {
    vec![
        label("VM_TRANSLATOR_END_LOOP"),
        a_sym("VM_TRANSLATOR_END_LOOP"),
        jmp_no_cond(),
    ]
}

fn sp() -> Instruction {
    a_sym("SP")
}

fn stack_top() -> Vec<Instruction> {
    vec![sp(), addr_assign("A", "M"), addr_assign("A", "A-1")]
}

fn inc_stack_ptr() -> Vec<Instruction> {
    vec![sp(), addr_assign("M", "M+1")]
}

fn dec_stack_ptr() -> Vec<Instruction> {
    vec![sp(), addr_assign("M", "M-1")]
}

fn push(segment: &Segment) -> Vec<Instruction> {
    let mut instructs = vec![];
    instructs.append(&mut segment_to_hack(segment));
    instructs.push(match segment {
        Segment::Constant(_) | Segment::NamedPtr(_) => addr_assign("D", "A"),
        _ => addr_assign("D", "M"),
    });

    instructs.push(sp());
    instructs.push(addr_assign("A", "M"));
    instructs.push(addr_assign("M", "D"));
    instructs.append(&mut inc_stack_ptr());

    instructs
}

fn pop(segment: &Segment) -> Vec<Instruction> {
    let mut instructs = vec![];
    instructs.append(&mut segment_to_hack(segment));

    let temp_addr = a_sym("R13");

    instructs.push(addr_assign("D", "A"));
    instructs.push(temp_addr.clone());
    instructs.push(addr_assign("M", "D"));
    instructs.append(&mut stack_top());
    instructs.push(addr_assign("D", "M"));
    instructs.push(temp_addr.clone());
    instructs.push(addr_assign("A", "M"));
    instructs.push(addr_assign("M", "D"));
    instructs.push(temp_addr);
    instructs.push(addr_assign("M", "0"));
    instructs.append(&mut dec_stack_ptr());

    instructs
}

fn comparator_template(jump: &str, line_idx: usize) -> Vec<Instruction> {
    let mut instructs = vec![];

    let pos_cond = format!("_pos_cond_{}", line_idx);
    let neg_cond = format!("_neg_cond_{}", line_idx);

    instructs.push(addr_assign("D", "M-D"));
    instructs.push(a_sym(&pos_cond));
    instructs.push(jmp("D", jump));
    instructs.push(a_sym(&neg_cond));
    instructs.push(addr_assign("D", "0"));
    instructs.push(jmp_no_cond());
    instructs.push(label(&pos_cond));
    instructs.push(addr_assign("D", "-1"));
    instructs.push(label(&neg_cond));
    instructs.append(&mut stack_top());
    instructs.push(addr_assign("M", "D"));

    instructs
}

fn arithmetic_two_stack_val(op: &ArithmeticOp, line_idx: usize) -> Vec<Instruction> {
    let mut op = match op {
        ArithmeticOp::Add => vec![addr_assign("M", "M+D")],
        ArithmeticOp::Subtract => vec![addr_assign("M", "M-D")],
        ArithmeticOp::And => vec![addr_assign("M", "M&D")],
        ArithmeticOp::Or => vec![addr_assign("M", "M|D")],
        ArithmeticOp::Equal => comparator_template("JEQ", line_idx),
        ArithmeticOp::GreaterThan => comparator_template("JGT", line_idx),
        ArithmeticOp::LessThan => comparator_template("JLT", line_idx),
        _ => vec![],
    };

    let mut instructs = vec![];

    instructs.append(&mut stack_top());
    instructs.push(addr_assign("D", "M"));
    instructs.append(&mut dec_stack_ptr());
    instructs.append(&mut stack_top());
    instructs.append(&mut op);

    instructs
}

fn arithmetic_one_stack_val(op: &ArithmeticOp) -> Vec<Instruction> {
    let op = match op {
        ArithmeticOp::Negate => addr_assign("M", "-M"),
        ArithmeticOp::Not => addr_assign("M", "!M"),
        _ => addr_assign("M", "M"),
    };

    let mut instructs = vec![];

    instructs.append(&mut stack_top());
    instructs.push(op);

    instructs
}

fn arithmetic(op: &ArithmeticOp, line_idx: usize) -> Vec<Instruction> {
    match op {
        ArithmeticOp::Negate | ArithmeticOp::Not => arithmetic_one_stack_val(op),
        _ => arithmetic_two_stack_val(op, line_idx),
    }
}

fn emit_label(base_label: &str, func: &Option<String>) -> Instruction {
    let lbl = match func {
        Some(func) => format!("{}${}", func, base_label),
        _ => base_label.to_string(),
    };

    label(&lbl)
}

fn emit_goto(base_label: &str, func: &Option<String>) -> Vec<Instruction> {
    let lbl = match func {
        Some(func) => format!("{}${}", func, base_label),
        None => base_label.to_string(),
    };

    vec![a_sym(&lbl), jmp_no_cond()]
}

fn emit_if_goto(label: &str, cur_func: &Option<String>) -> Vec<Instruction> {
    let lbl = match cur_func {
        Some(func) => format!("{}${}", func, label),
        None => label.to_string(),
    };

    let mut instructs = vec![];

    instructs.append(&mut stack_top());
    instructs.push(addr_assign("D", "M"));
    instructs.append(&mut dec_stack_ptr());
    instructs.push(a_sym(&lbl));
    instructs.push(jmp("D", "JNE"));

    instructs
}

fn emit_func(func: &str, arg_cnt: &usize) -> Vec<Instruction> {
    let mut args = (0..*arg_cnt)
        .map(|_| push(&Segment::Constant(0)))
        .flatten()
        .collect::<Vec<Instruction>>();

    let mut instructs = vec![label(func)];
    instructs.append(&mut args);

    instructs
}

fn emit_call(
    func: &str,
    arg_cnt: &usize,
    ret_cnt: &usize,
    parent_func: &Option<String>,
) -> Vec<Instruction> {
    let lbl = match parent_func {
        Some(func) => format!("{}$ret.{}", func, ret_cnt),
        _ => format!("{}$ret.{}", func, ret_cnt),
    };

    let mut instructs = vec![];

    instructs.append(&mut push(&Segment::NamedPtr(lbl.clone())));
    instructs.append(&mut push(&Segment::Named("LCL".to_string())));
    instructs.append(&mut push(&Segment::Named("ARG".to_string())));
    instructs.append(&mut push(&Segment::Named("THIS".to_string())));
    instructs.append(&mut push(&Segment::Named("THAT".to_string())));

    // ARG = SP - 5 - nArgs
    instructs.push(sp());
    instructs.push(addr_assign("D", "M"));
    instructs.push(a_const(&((5 + arg_cnt) as i16))); // stack_top() decrements by 1
    instructs.push(addr_assign("D", "D-A"));
    instructs.push(a_sym("ARG"));
    instructs.push(addr_assign("M", "D"));

    // LCL = SP
    instructs.push(sp());
    instructs.push(addr_assign("D", "M"));
    instructs.push(a_sym("LCL"));
    instructs.push(addr_assign("M", "D"));

    instructs.append(&mut emit_goto(func, &None));
    instructs.push(label(&lbl));

    instructs
}

fn frame_sub(dest: &str, offset: usize) -> Vec<Instruction> {
    vec![
        a_sym("R14"),
        addr_assign("D", "M"),
        a_const(&(offset as i16)),
        addr_assign("A", "D-A"),
        addr_assign("D", "M"),
        a_sym(dest),
        addr_assign("M", "D"),
    ]
}

fn emit_return() -> Vec<Instruction> {
    let frame = "R14";
    let ret_addr = "R15";

    let mut instructs = vec![
        a_sym("LCL"),
        addr_assign("D", "M"),
        a_sym(frame),
        addr_assign("M", "D"),
    ];

    // retAddr = *(frame - 5)
    instructs.append(&mut frame_sub(ret_addr, 5));

    // *ARG = pop()
    instructs.append(&mut pop(&Segment::Argument(0)));

    // SP = ARG + 1
    instructs.push(a_sym("ARG"));
    instructs.push(addr_assign("D", "M"));
    instructs.push(sp());
    instructs.push(addr_assign("M", "D+1"));

    // THAT = *(frame - 1)
    instructs.append(&mut frame_sub("THAT", 1));
    // THIS = *(frame - 2)
    instructs.append(&mut frame_sub("THIS", 2));
    // ARG = *(frame - 3)
    instructs.append(&mut frame_sub("ARG", 3));
    // LCL = *(frame - 4)
    instructs.append(&mut frame_sub("LCL", 4));

    // goto retAddr
    instructs.push(a_sym(ret_addr));
    instructs.push(addr_assign("A", "M"));
    instructs.push(jmp_no_cond());

    instructs
}

pub struct CodeWriter<T: Write> {
    writer: BufWriter<T>,
    cur_line_idx: usize,
    cur_ret_count: usize,
    cur_func: Option<String>,
}

impl<T: Write> CodeWriter<T> {
    pub fn new(writer: BufWriter<T>, no_sys_init: bool) -> Self {
        let mut writer = CodeWriter {
            writer,
            cur_line_idx: 0,
            cur_ret_count: 0,
            cur_func: None,
        };

        if !no_sys_init {
            let out = instruct_vec_str(&assembly_header());
            writeln!(writer.writer, "{}", out).unwrap();
        }

        writer
    }

    pub fn write(&mut self, cmd: &Command) -> std::io::Result<()> {
        let output = match cmd {
            Command::Push(seg) => push(seg),
            Command::Pop(seg) => pop(seg),
            Command::Arithmetic(op) => arithmetic(op, self.cur_line_idx),
            Command::Label(label) => vec![emit_label(label, &self.cur_func)],
            Command::Goto(label) => emit_goto(label, &self.cur_func),
            Command::IfGoto(label) => emit_if_goto(label, &self.cur_func),
            Command::Call(func, arg_cnt) => {
                let out = emit_call(func, arg_cnt, &self.cur_ret_count, &self.cur_func);
                self.cur_ret_count += 1;
                out
            }
            Command::Function(func, arg_cnt) => {
                self.cur_func = Some(func.to_string());
                self.cur_ret_count = 0;
                emit_func(func, arg_cnt)
            }
            Command::Return => emit_return(),
        };

        writeln!(self.writer, "{}", instruct_vec_str(&output))?;
        self.writer.flush()?;
        self.cur_line_idx += 1;
        Ok(())
    }

    pub fn on_new_file(&mut self) {
        self.cur_func = None;
        self.cur_ret_count = 0;
    }

    pub fn close(&mut self) -> std::io::Result<()> {
        // Write infinite loop.
        let end_loop = assembly_footer();
        writeln!(self.writer, "{}", instruct_vec_str(&end_loop))?;

        self.writer.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_iter(cmds: &[Command], expected_body: Option<&str>) {
        let expected = match expected_body {
            Some(body) => format!(
                "{}\n{}\n{}\n",
                instruct_vec_str(&assembly_header()),
                body,
                instruct_vec_str(&assembly_footer())
            ),
            None => format!(
                "{}\n{}\n",
                instruct_vec_str(&assembly_header()),
                instruct_vec_str(&assembly_footer())
            ),
        };

        let buf_writer = BufWriter::new(Vec::new());
        let mut writer = CodeWriter::new(buf_writer, false);

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
        let expected = [
            "@0", "D=A", "@LCL", "A=D+M", "D=M", "@SP", "A=M", "M=D", "@SP", "M=M+1",
        ]
        .join("\n");

        test_iter(&cmd, Some(&expected));
    }

    #[test]
    fn test_pop() {
        let cmd = [Command::Pop(Segment::Local(0))];
        let expected = [
            "@0", "D=A", "@LCL", "A=D+M", "D=A", "@R13", "M=D", "@SP", "A=M", "A=A-1", "D=M",
            "@R13", "A=M", "M=D", "@R13", "M=0", "@SP", "M=M-1",
        ]
        .join("\n");

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

    #[test]
    fn test_sub() {
        let cmd = [Command::Arithmetic(ArithmeticOp::Subtract)];
        let expected = [
            "@SP", "A=M", "A=A-1", "D=M", "@SP", "M=M-1", "@SP", "A=M", "A=A-1", "M=M-D",
        ]
        .join("\n");

        test_iter(&cmd, Some(&expected));
    }

    #[test]
    fn test_and() {
        let cmd = [Command::Arithmetic(ArithmeticOp::And)];
        let expected = [
            "@SP", "A=M", "A=A-1", "D=M", "@SP", "M=M-1", "@SP", "A=M", "A=A-1", "M=M&D",
        ]
        .join("\n");

        test_iter(&cmd, Some(&expected));
    }

    #[test]
    fn test_or() {
        let cmd = [Command::Arithmetic(ArithmeticOp::Or)];
        let expected = [
            "@SP", "A=M", "A=A-1", "D=M", "@SP", "M=M-1", "@SP", "A=M", "A=A-1", "M=M|D",
        ]
        .join("\n");

        test_iter(&cmd, Some(&expected));
    }

    #[test]
    fn test_neg() {
        let cmd = [Command::Arithmetic(ArithmeticOp::Negate)];
        let expected = ["@SP", "A=M", "A=A-1", "M=-M"].join("\n");

        test_iter(&cmd, Some(&expected));
    }

    #[test]
    fn test_not() {
        let cmd = [Command::Arithmetic(ArithmeticOp::Not)];
        let expected = ["@SP", "A=M", "A=A-1", "M=!M"].join("\n");

        test_iter(&cmd, Some(&expected));
    }

    #[test]
    fn test_eq() {
        let cmd = [Command::Arithmetic(ArithmeticOp::Equal)];
        let expected = [
            "@SP",
            "A=M",
            "A=A-1",
            "D=M",
            "@SP",
            "M=M-1",
            "@SP",
            "A=M",
            "A=A-1",
            "D=M-D",
            "@_pos_cond_0",
            "D;JEQ",
            "@_neg_cond_0",
            "D=0",
            "0;JMP",
            "(_pos_cond_0)",
            "D=-1",
            "(_neg_cond_0)",
            "@SP",
            "A=M",
            "A=A-1",
            "M=D",
        ]
        .join("\n");

        test_iter(&cmd, Some(&expected));
    }

    #[test]
    fn test_gt() {
        let cmd = [Command::Arithmetic(ArithmeticOp::GreaterThan)];
        let expected = [
            "@SP",
            "A=M",
            "A=A-1",
            "D=M",
            "@SP",
            "M=M-1",
            "@SP",
            "A=M",
            "A=A-1",
            "D=M-D",
            "@_pos_cond_0",
            "D;JGT",
            "@_neg_cond_0",
            "D=0",
            "0;JMP",
            "(_pos_cond_0)",
            "D=-1",
            "(_neg_cond_0)",
            "@SP",
            "A=M",
            "A=A-1",
            "M=D",
        ]
        .join("\n");

        test_iter(&cmd, Some(&expected));
    }

    #[test]
    fn test_lt() {
        let cmd = [Command::Arithmetic(ArithmeticOp::LessThan)];
        let expected = [
            "@SP",
            "A=M",
            "A=A-1",
            "D=M",
            "@SP",
            "M=M-1",
            "@SP",
            "A=M",
            "A=A-1",
            "D=M-D",
            "@_pos_cond_0",
            "D;JLT",
            "@_neg_cond_0",
            "D=0",
            "0;JMP",
            "(_pos_cond_0)",
            "D=-1",
            "(_neg_cond_0)",
            "@SP",
            "A=M",
            "A=A-1",
            "M=D",
        ]
        .join("\n");

        test_iter(&cmd, Some(&expected));
    }

    #[test]
    fn test_this_segment() {
        let input = Segment::This(6);
        let expected = ["@6", "D=A", "@THIS", "A=D+M"].join("\n");

        assert_eq!(expected, instruct_vec_str(&segment_to_hack(&input)));
    }

    #[test]
    fn test_func() {
        let cmds = [Command::Function("Xxx.foo".to_string(), 0)];
        let expected = ["(Xxx.foo)"].join("\n");

        test_iter(&cmds, Some(&expected));
    }

    #[test]
    fn test_func_label() {
        let cmds = [
            Command::Function("Xxx.foo".to_string(), 0),
            Command::Label("bar".to_string()),
        ];
        let expected = ["(Xxx.foo)", "(Xxx.foo$bar)"].join("\n");

        test_iter(&cmds, Some(&expected));
    }
}
