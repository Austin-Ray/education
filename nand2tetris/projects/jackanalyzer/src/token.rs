use std::io::Lines;

#[derive(Debug, PartialEq, Clone)]
enum KeywordType {
    Class,
    Method,
    Function,
    Constructor,
    Int,
    Boolean,
    Char,
    Void,
    Var,
    Static,
    Field,
    Let,
    Do,
    If,
    Else,
    While,
    Return,
    True,
    False,
    Null,
    This,
}

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Keyword(KeywordType),
    Symbol(char),
    Identifier(String),
    IntConst(i32),
    StringConst(String),
}

struct Tokenizer<T> {
    lines: Lines<T>,
}

impl<T> Tokenizer<T> {
    fn new(lines: Lines<T>) -> Self {
        Tokenizer { lines }
    }

    fn advance(&mut self) -> Option<Token> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufRead, BufReader};

    fn test_iter(input: &str, expected: &[Token]) {
        let reader = BufReader::new(input.as_bytes());
        let mut tokenizer = Tokenizer::new(reader.lines());

        for item in expected {
            let actual = tokenizer.advance();

            assert_eq!(Some(item.clone()), actual);
        }
    }

    #[test]
    fn test_assignment() {
        let input = "let x = 100;";
        let expected = [
            Token::Keyword(KeywordType::Let),
            Token::Identifier("x".to_string()),
            Token::Symbol('='),
            Token::IntConst(100),
            Token::Symbol(';'),
        ];

        test_iter(input, &expected);
    }

    #[test]
    fn test_while() {
        let input = r#"
        if (x < 0) {
            // handles the sign
            let sign = "negative";
        }
        "#;

        let expected = [
            Token::Keyword(KeywordType::If),
            Token::Symbol('('),
            Token::Identifier("x".to_string()),
            Token::Symbol('<'),
            Token::IntConst(0),
            Token::Symbol(')'),
            Token::Symbol('{'),
            Token::Keyword(KeywordType::Let),
            Token::Identifier("sign".to_string()),
            Token::Symbol('='),
            Token::StringConst("negative".to_string()),
            Token::Symbol(';'),
            Token::Symbol('}'),
        ];

        test_iter(input, &expected);
    }
}
