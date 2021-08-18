use crate::syntax::Token;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::VecDeque;
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

fn clean_line(line: &str) -> String {
    let trimmed = line.trim();
    strip_trailing_comment(trimmed)
}

fn grab_next_line<T: BufRead>(lines: &mut Lines<T>) -> Option<String> {
    match lines.next() {
        Some(res) => match res {
            Ok(line) => {
                if line.is_empty() {
                    grab_next_line(lines)
                } else {
                    Some(line)
                }
            }
            Err(_) => None,
        },
        None => None,
    }
}

fn char_slice_to_token(chars: &[char]) -> Result<Token, String> {
    chars.iter().collect::<String>().parse()
}

fn nested_tokenize(nested_string: &str) -> Vec<Token> {
    let mut tokens = vec![];
    let chars = nested_string.chars().collect::<Vec<char>>();

    if !chars.is_empty() {
        let mut next_slice_idx = 0;
        let mut next_token = None;

        for i in 0..chars.len() {
            next_token = match char_slice_to_token(&chars[0..i + 1]) {
                Ok(token) => Some(token),
                Err(_) => {
                    if next_token.is_some() {
                        break;
                    }
                    None
                }
            };

            next_slice_idx = i;
        }

        if let Some(token) = next_token {
            tokens.push(token);
            tokens.append(&mut nested_tokenize(
                &chars[next_slice_idx + 1..].iter().collect::<String>(),
            ));
        }
    }

    tokens
}

fn take_until_valid_line<T: BufRead>(lines: &mut Lines<T>) -> Option<String> {
    let mut cleaned_line = None;

    while cleaned_line.is_none() {
        let cur_line = grab_next_line(lines)?;
        let clean_line = clean_line(&cur_line);

        if !clean_line.is_empty() {
            cleaned_line = Some(clean_line);
        }
    }

    cleaned_line
}

fn tokenize<T: BufRead>(lines: &mut Lines<T>) -> Option<Vec<Token>> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"('.*?'|".*?"|\S+)"#).unwrap();
    }

    let cur_line = take_until_valid_line(lines)?;

    let tokens = RE
        .find_iter(&cur_line)
        .map(|x| x.as_str())
        .map(|x| x.parse::<Token>())
        .map(|x| match x {
            Ok(x) => vec![x],
            Err(error_string) => nested_tokenize(&error_string),
        })
        .flatten()
        .collect::<Vec<Token>>();

    Some(tokens)
}

struct Tokenizer<T: BufRead> {
    lines: Lines<T>,
    cur_tokens: VecDeque<Token>,
}

impl<T: BufRead> Tokenizer<T> {
    fn new(lines: Lines<T>) -> Self {
        Tokenizer {
            lines,
            cur_tokens: VecDeque::new(),
        }
    }

    fn advance(&mut self) -> Option<Token> {
        if self.cur_tokens.is_empty() {
            self.cur_tokens = VecDeque::from(tokenize(&mut self.lines)?);
        }

        self.cur_tokens.pop_front()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::{KeywordType, Symbol};
    use std::io::{BufRead, BufReader};

    fn test_iter(input: &str, expected: &[Token]) {
        let reader = BufReader::new(input.as_bytes());
        let mut tokenizer = Tokenizer::new(reader.lines());

        let mut actual = vec![];
        while let Some(token) = tokenizer.advance() {
            actual.push(token)
        }

        assert_eq!(expected, actual);
    }

    /// While invalid to neglect a semi-colon, it's easier to parse.
    /// This allows building simple token-based parsing without a look ahead.
    #[test]
    fn test_assignment_no_semi() {
        let input = "let x = 100";
        let expected = [
            Token::Keyword(KeywordType::Let),
            Token::Identifier("x".to_string()),
            Token::Symbol("=".parse().unwrap()),
            Token::IntConst(100),
        ];

        test_iter(input, &expected);
    }

    #[test]
    fn test_assignment() {
        let input = "let x = 100;";
        let expected = [
            Token::Keyword(KeywordType::Let),
            Token::Identifier("x".to_string()),
            Token::Symbol("=".parse().unwrap()),
            Token::IntConst(100),
            Token::Symbol(";".parse().unwrap()),
        ];

        test_iter(input, &expected);
    }

    #[test]
    fn test_multiline_tokenize() {
        let input = r#"
            let x = 100;
            let y = 100;
        "#;

        let expected = [
            Token::Keyword(KeywordType::Let),
            Token::Identifier("x".to_string()),
            Token::Symbol("=".parse().unwrap()),
            Token::IntConst(100),
            Token::Symbol(";".parse().unwrap()),
            Token::Keyword(KeywordType::Let),
            Token::Identifier("y".to_string()),
            Token::Symbol("=".parse().unwrap()),
            Token::IntConst(100),
            Token::Symbol(";".parse().unwrap()),
        ];

        test_iter(input, &expected);
    }

    #[test]
    fn test_while_no_comment() {
        let input = r#"
        if (x < 0) {
            // handles the sign
            let sign = "negative";
        }
        "#;

        let expected = [
            Token::Keyword(KeywordType::If),
            Token::Symbol("(".parse().unwrap()),
            Token::Identifier("x".to_string()),
            Token::Symbol("<".parse().unwrap()),
            Token::IntConst(0),
            Token::Symbol(")".parse().unwrap()),
            Token::Symbol("{".parse().unwrap()),
            Token::Keyword(KeywordType::Let),
            Token::Identifier("sign".to_string()),
            Token::Symbol("=".parse().unwrap()),
            Token::StringConst("negative".to_string()),
            Token::Symbol(";".parse().unwrap()),
            Token::Symbol("}".parse().unwrap()),
        ];

        test_iter(input, &expected);
    }

    #[test]
    fn test_while_comment() {
        let input = r#"
        if (x < 0) {
            // handles the sign
            let sign = "negative";
        }
        "#;

        let expected = [
            Token::Keyword(KeywordType::If),
            Token::Symbol("(".parse().unwrap()),
            Token::Identifier("x".to_string()),
            Token::Symbol("<".parse().unwrap()),
            Token::IntConst(0),
            Token::Symbol(")".parse().unwrap()),
            Token::Symbol("{".parse().unwrap()),
            Token::Keyword(KeywordType::Let),
            Token::Identifier("sign".to_string()),
            Token::Symbol("=".parse().unwrap()),
            Token::StringConst("negative".to_string()),
            Token::Symbol(";".parse().unwrap()),
            Token::Symbol("}".parse().unwrap()),
        ];

        test_iter(input, &expected);
    }

    /// Test that string constants won't be split.
    /// No semi-colon as that complicates the parsing.
    #[test]
    fn test_space_in_str_const_no_semi() {
        let input = "let sign = \"hello world\"";
        let expected = [
            Token::Keyword(KeywordType::Let),
            Token::Identifier("sign".to_string()),
            Token::Symbol("=".parse().unwrap()),
            Token::StringConst("hello world".to_string()),
        ];

        test_iter(input, &expected);
    }

    #[test]
    fn test_nested_tokenize_num_semi() {
        let input = "42;";
        let expected = vec![Token::IntConst(42), Token::Symbol(Symbol::Semicolon)];

        let actual = nested_tokenize(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_nested_tokenize_curly_brace_str_const() {
        let input = "{\"hello\"}";
        let expected = vec![
            Token::Symbol(Symbol::LCurlyBrace),
            Token::StringConst("hello".to_string()),
            Token::Symbol(Symbol::RCurlyBrace),
        ];

        let actual = nested_tokenize(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_nested_tokenize_curly_brace_str_const_twice() {
        let input = "{\"hello\"}{\"hello\"}";
        let expected = vec![
            Token::Symbol(Symbol::LCurlyBrace),
            Token::StringConst("hello".to_string()),
            Token::Symbol(Symbol::RCurlyBrace),
            Token::Symbol(Symbol::LCurlyBrace),
            Token::StringConst("hello".to_string()),
            Token::Symbol(Symbol::RCurlyBrace),
        ];

        let actual = nested_tokenize(input);

        assert_eq!(expected, actual);
    }
}
