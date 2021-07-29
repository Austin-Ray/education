use std::fmt;

#[derive(Clone)]
pub enum Instruction {
    AConst(i16),
    ASymbolic(String),
    C(Option<String>, String, Option<String>),
    Label(String),
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = match self {
            Self::AConst(val) => format!("@{}", val),
            Self::ASymbolic(val) => format!("@{}", val),
            Self::Label(val) => format!("({})", val),
            Self::C(dest, comp, jump) => {
                let mut out = String::new();

                if let Some(dest) = dest {
                    out.push_str(dest);
                    out.push('=');
                }

                out.push_str(comp);

                if let Some(jump) = jump {
                    out.push(';');
                    out.push_str(jump);
                }

                out
            }
        };

        write!(f, "{}", out)
    }
}
