#[derive(Debug, PartialEq)]
pub enum Segment {
    Argument,
    Local,
    Static,
    Constant(usize),
    This,
    That,
    Pointer,
    Temp,
}

#[derive(Debug, PartialEq)]
pub enum Command {
    Arithmetic(String),
    Push(Segment, usize),
    Pop(Segment, usize),
    Label,
    Goto,
    Function(String, usize),
    Return,
    Call(String, usize),
}
