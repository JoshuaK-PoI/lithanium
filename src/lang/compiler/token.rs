use std::iter::Peekable;
use serde::Serialize;

use crate::lang::util::vec::{Unshift, UnshiftExpect };
use super::{CompilerError, CompilerResult};

pub(crate) type Span = std::ops::Range<usize>;
pub(crate) type TokenStream<'a> = Peekable<std::slice::IterMut<'a, Token>>;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize)]
pub enum TokenType {
    // Special tokens
    Unknown,

    // Single char tokens
    Ampersand,
    Asterisk,
    At,
    Colon,
    Comma,
    Equals,
    LeftBrace,
    LeftParen,
    Pipe,
    RightBrace,
    RightParen,
    Semicolon,
    Slash,

    // Multi-char tokens
    AmpersandAmpersand,
    BangEquals,
    EqualsEquals,
    PipePipe,

    // keywords
    Break,
    Continue,
    Else,
    For,
    Function,
    Let,
    If,
    Return,
    While,

    // N-char tokens
    Identifier,
    IntegerLiteral,
}

impl From<TokenType> for String {
    fn from(token: TokenType) -> String {
        format!("{:?}", token)
    }
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(*self))
    }
}

impl<'a> UnshiftExpect<Token, TokenType, CompilerError> for TokenStream<'a>
{
    fn unshift_expect(&mut self, expected: TokenType) -> CompilerResult<&mut Token> {
        match self.peek() {
            Some(token) if token.type_ == expected => Ok(self.unshift().unwrap()),
            Some(token) => Err(CompilerError {
                error_code: crate::lang::compiler::ErrorCode::UnshiftedUnexpectedToken,
                error_message: format!("Expected '{}', got {}", expected, token),
                span_message: String::from(""),
                token: self.unshift().unwrap().clone(),
                help: Some(String::from(
                    "Expected one of: - TODO: List expected tokens",
                )),
                info: None,
            }),
            None => Err(CompilerError {
                error_code: crate::lang::compiler::ErrorCode::NoTokensLeft,
                error_message: format!("Expected token {}, got None", expected),
                span_message: String::from(""),
                token: Token::invalid(),
                help: Some(String::from(
                    "Expected one of: - TODO: List expected tokens",
                )),
                info: None,
            }),
        }
    }

    fn unshift_expect_any(&mut self, expected: &[TokenType]) -> CompilerResult<&mut Token> {
        match self.peek() {
            Some(token) if expected.contains(&token.type_) => Ok(self.unshift().unwrap()),
            Some(token) => Err(CompilerError {
                error_code: crate::lang::compiler::ErrorCode::UnshiftedUnexpectedToken,
                error_message: format!("Expected one of: {:?}, got {}", expected, token),
                span_message: String::from(""),
                token: self.unshift().unwrap().clone(),
                help: Some(String::from(
                    "Expected one of: - TODO: List expected tokens",
                )),
                info: None,
            }),
            None => Err(CompilerError {
                error_code: crate::lang::compiler::ErrorCode::NoTokensLeft,
                error_message: format!("Expected token {:?}, got None", expected),
                span_message: String::from(""),
                token: Token::invalid(),
                help: Some(String::from(
                    "Expected one of: - TODO: List expected tokens",
                )),
                info: None,
            }),
        }
    }

    fn unshift_if(&mut self, token_type: TokenType) -> Option<&mut Token> {
        match self.peek() {
            Some(token) => {
                if token.type_ == token_type {
                    Some(self.unshift().unwrap())
                } else {
                    None
                }
            }
            None => None
        }
    }

    fn next_matches(&mut self, token_type: TokenType) -> bool {
        match self.peek() {
            Some(token) => token.type_ == token_type,
            None => false,
        }
    }

    fn next_matches_any(&mut self, expected: &[TokenType]) -> bool {
        match self.peek() {
            Some(token) => expected.contains(&token.type_),
            None => false,
        }
    }
}

pub(crate) static KEYWORDS: phf::Map<&'static str, TokenType> = phf::phf_map! {
    "let" => TokenType::Let,
    "function" => TokenType::Function,
    "return" => TokenType::Return,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct Token {
    pub(crate) type_: TokenType,
    pub(crate) value: String,
    pub(crate) span: Span,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:16} '{}' @ {:#04}..{:#04} ",
            self.type_, self.value, self.span.start, self.span.end
        )
    }
}

impl Token {
    pub(crate) fn invalid() -> Token {
        Token {
            type_: TokenType::Unknown,
            value: String::from("<invalid>"),
            span: 0..0,
        }
    }
}
