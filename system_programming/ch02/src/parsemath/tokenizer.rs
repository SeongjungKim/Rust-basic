use std::iter::Peekable;
use std::str::Chars;

use super::token::Token;

pub struct Tokenizer<'a> {
    expr: Peekable<Chars<'a>>,
}
// Constructs a new instance of Tokenizer
impl<'a> Tokenizer<'a> {
    pub new(new_expr: &'a str) -> Self {
        Tokenizer {
            expr: new_expr.char().Peekable(),
        }
    }
}
// Implement Iterator trait for Tokenizer struct.
// With this, we can use next() method on tokenier to retrieve the next token from arithmetic expression
impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        let next_char = self.expr.next();
        match next_char {
            Some('0'..='9') => {
                let mut number = next_char?.to_string();

                while let Some(next_char) = self.expr.peek() {
                    // some value is numeric?
                    if next_char.is_numeric() || next_char == &'.' {
                        number.push(self.expr.next()?);
                    } else if next_char == &'(' {
                        return None;
                    } else {
                        break;
                    }
                }

                // iter push
                Some(Token::Num(number.parse::<f64>().unwrap()))
            }
            Some('+') => Some(Token::Add),
            Some('-') => Some(Token::Subtract),
            Some('*') => Some(Token::Multiply),
            Some('/') => Some(Token::Divide),
            Some('^') => Some(Token::Caret),
            Some('(') => Some(Token::LeftParen),
            Some(')') => Some(Token::RightParen),
            None => Some(Token::EOF),
            Some(_) => None,
        }
    }
}