#[derive(Debug, PartialEq)]
pub enum Segment {
    Argument(i16),
    Local(i16),
    Static(i16),
    Constant(i16),
    This(i16),
    That(i16),
    Pointer(i16),
    Temp(i16),
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
    Label,
    Goto,
    Function(String, usize),
    Return,
    Call(String, usize),
}
