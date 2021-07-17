#[derive(Debug, PartialEq)]
pub enum Instruction {
    AConst(i32),
    AVar(String),
    C {
        dest: Option<String>,
        comp: String,
        jump: Option<String>,
    },
    L(String),
}
