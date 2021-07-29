#[derive(Debug, PartialEq)]
pub enum Segment {
    Argument(i16),
    Local(i16),
    Static(String, i16),
    Constant(i16),
    This(i16),
    That(i16),
    Pointer(i16),
    Temp(i16),
    Named(String),
    NamedPtr(String),
}

#[derive(Debug, PartialEq)]
pub enum ArithmeticOp {
    Add,
    Subtract,
    Negate,
    Equal,
    GreaterThan,
    LessThan,
    And,
    Or,
    Not,
}

#[derive(Debug, PartialEq)]
pub enum Command {
    Arithmetic(ArithmeticOp),
    Push(Segment),
    Pop(Segment),
    Label(String),
    Goto(String),
    IfGoto(String),
    Function(String, usize),
    Return,
    Call(String, usize),
}
