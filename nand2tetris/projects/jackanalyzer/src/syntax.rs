use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub enum KeywordType {
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

impl FromStr for KeywordType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "class" => Ok(Self::Class),
            "method" => Ok(Self::Method),
            "function" => Ok(Self::Function),
            "constructor" => Ok(Self::Constructor),
            "int" => Ok(Self::Int),
            "boolean" => Ok(Self::Boolean),
            "char" => Ok(Self::Char),
            "void" => Ok(Self::Void),
            "var" => Ok(Self::Var),
            "static" => Ok(Self::Static),
            "field" => Ok(Self::Field),
            "let" => Ok(Self::Let),
            "do" => Ok(Self::Do),
            "if" => Ok(Self::If),
            "else" => Ok(Self::Else),
            "while" => Ok(Self::While),
            "return" => Ok(Self::Return),
            "true" => Ok(Self::True),
            "false" => Ok(Self::False),
            "null" => Ok(Self::Null),
            "this" => Ok(Self::This),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Symbol {
    LCurlyBrace,
    RCurlyBrace,
    LParen,
    RParen,
    LBracket,
    RBracket,
    Period,
    Comma,
    Semicolon,
    Plus,
    Minus,
    Asterick,
    ForwardSlash,
    Ampersand,
    Pipe,
    LAngleBracket,
    RAngleBracket,
    Equal,
    Tilde,
}

impl FromStr for Symbol {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "{" => Ok(Self::LCurlyBrace),
            "}" => Ok(Self::RCurlyBrace),
            "(" => Ok(Self::LParen),
            ")" => Ok(Self::RParen),
            "[" => Ok(Self::LBracket),
            "]" => Ok(Self::RBracket),
            "." => Ok(Self::Period),
            "," => Ok(Self::Comma),
            ";" => Ok(Self::Semicolon),
            "+" => Ok(Self::Plus),
            "-" => Ok(Self::Minus),
            "*" => Ok(Self::Asterick),
            "/" => Ok(Self::ForwardSlash),
            "&" => Ok(Self::Ampersand),
            "|" => Ok(Self::Pipe),
            "<" => Ok(Self::LAngleBracket),
            ">" => Ok(Self::RAngleBracket),
            "=" => Ok(Self::Equal),
            "~" => Ok(Self::Tilde),
            _ => Err(s.to_string()),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Keyword(KeywordType),
    Symbol(Symbol),
    Identifier(String),
    IntConst(i16),
    StringConst(String),
}

impl FromStr for Token {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref STRING_CONST_RE: Regex = Regex::new(r#"^"[^"]*"$"#).unwrap();
            static ref IDENTIFIER_RE: Regex = Regex::new("^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap();
        }

        if let Ok(keyword) = s.parse::<KeywordType>() {
            return Ok(Self::Keyword(keyword));
        }
        if let Ok(symbol) = s.parse::<Symbol>() {
            return Ok(Self::Symbol(symbol));
        }
        if let Ok(num) = s.parse::<i16>() {
            return Ok(Self::IntConst(num));
        }
        if STRING_CONST_RE.is_match(s) {
            return Ok(Self::StringConst(s.replace("\"", "")));
        }
        if IDENTIFIER_RE.is_match(s) {
            return Ok(Self::Identifier(s.to_string()));
        }

        Err(s.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::KeywordType::*;
    use super::Symbol::*;
    use super::Token;
    use super::Token::*;
    use std::fmt::Debug;
    use std::str::FromStr;

    fn parse_tester<T>(input: &str, expected: T)
    where
        T: FromStr + Debug + PartialEq,
        <T as FromStr>::Err: Debug + PartialEq,
    {
        let expected = Ok(expected);
        let actual = input.parse::<T>();

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_keyword_type() {
        parse_tester("class", Class);
    }

    #[test]
    fn parse_token_keyword() {
        parse_tester("class", Keyword(Class));
    }

    #[test]
    fn parse_token_symbol() {
        parse_tester("{", Symbol(LCurlyBrace));
    }

    #[test]
    fn parse_token_int_const() {
        parse_tester("42", IntConst(42));
    }

    #[test]
    fn parse_token_string_const() {
        parse_tester("\"my_const\"", StringConst("my_const".to_string()));
        assert_eq!(Err("\"hello\"}".to_string()), "\"hello\"}".parse::<Token>());
        assert_eq!(
            Err("\"hello\"world\"".to_string()),
            "\"hello\"world\"".parse::<Token>()
        );
    }

    #[test]
    fn parse_token_identifier() {
        parse_tester("_hello", Identifier("_hello".to_string()));
        parse_tester("_hello0", Identifier("_hello0".to_string()));
        assert_eq!(Err("0hello".to_string()), "0hello".parse::<Token>());
        assert_eq!(Err("_hello;".to_string()), "_hello;".parse::<Token>());
    }
}
