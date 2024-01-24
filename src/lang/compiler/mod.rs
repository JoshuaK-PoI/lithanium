use crate::lang::compiler::token::TokenType;
use core::fmt::Display;
use log::trace;

use self::{parser::{AST, Parser}, token::Token};

use super::util::error_logger::ErrorLogger;

pub(crate) mod expression;
pub(crate) mod lexer;
pub(crate) mod parser;
pub(crate) mod statement;
pub(crate) mod token;

pub(crate) type CompilerResult<T> = Result<T, CompilerError>;
pub(crate) struct CompilerError {
    pub(crate) error_code: ErrorCode,
    pub(crate) error_message: String,
    pub(crate) span_message: String,
    pub(crate) token: Token,
    pub(crate) help: Option<String>,
    pub(crate) info: Option<String>,
}

pub(crate) enum ErrorCode {
    InvalidExpression,
    InvalidParameterType,
    InvalidReturnType,
    NoTokensLeft,
    UnexpectedToken,
    UnknownToken,
    UnshiftedUnexpectedToken,
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorCode::UnknownToken => write!(f, "Unknown token"),
            ErrorCode::UnexpectedToken => write!(f, "Unexpected token"),
            ErrorCode::NoTokensLeft => write!(f, "No tokens left"),
            ErrorCode::UnshiftedUnexpectedToken => write!(f, "Unshifted unexpected token"),
            ErrorCode::InvalidParameterType => write!(f, "Invalid parameter type"),
            ErrorCode::InvalidReturnType => write!(f, "Invalid return type"),
            ErrorCode::InvalidExpression => write!(f, "Invalid expression"),
        }
    }
}

#[derive(Clone)]
pub(crate) struct Compiler<'a> {
    pub(crate) input: &'a str,
    pub(crate) filename: &'a str,
    lexer: lexer::Lexer<'a>,
    error_logger: ErrorLogger<'a>,
}

impl Compiler<'_> {
    pub(crate) fn new<'a>(input: &'a str, filename: &'a str) -> Compiler<'a> {
        let error_logger = ErrorLogger::new(filename, input);
        Compiler {
            input,
            filename,
            lexer: lexer::Lexer::new(),
            error_logger,
        }
    }

    pub(crate) fn compile(&mut self) -> Result<AST, String> {
        self.lexer.lex(self.input);

        let unknown_tokens: Vec<&mut Token> = self
            .lexer
            .get_tokens_peekable()
            .filter(|token| token.type_ == TokenType::Unknown)
            .collect();

        if unknown_tokens.len() > 0 {
            let errors: Vec<CompilerError> = unknown_tokens
                .iter()
                .map(|token| CompilerError {
                    error_code: ErrorCode::UnknownToken,
                    error_message: format!("Unknown token: {:?}", token.type_), // Clone the token object
                    token: (**token).clone(),
                    span_message: format!("This token is unknown to the compiler"),
                    help: None,
                    info: None,
                })
                .collect();

            self.error_logger.report_many(&errors);
        }

        if log::max_level() >= log::LevelFilter::Trace {
            trace!("{:#04}..{:#04} {}", "Byte", "Rnge", "Token");

            trace!("{:-<1$}", "", 40);
            for token in self.lexer.get_tokens_peekable() {
                trace!("{}", token);
            }
        }

        let mut token_stream = self.lexer.get_tokens_peekable();
        let ast = Parser::parse(&mut token_stream).map_err(|e| {
            self.error_logger.report(&e);
            return format!("Error parsing tokens: {}", e.error_message);
        });

        ast
    }
}
