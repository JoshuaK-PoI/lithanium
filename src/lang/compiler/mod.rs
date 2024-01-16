use crate::lang::compiler::token::TokenType;
use core::fmt::Display;
use log::trace;

use self::{parser::AST, token::Token};

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
    UnknownToken,
    UnexpectedToken,
    UnshiftedUnexpectedToken,
    NoTokensLeft,
    InvalidParameterType,
    InvalidReturnType,
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
        }
    }
}

#[derive(Clone)]
pub(crate) struct Compiler<'a> {
    pub(crate) input: &'a str,
    pub(crate) filename: &'a str,
    lexer: lexer::Lexer<'a>,
    parser: parser::Parser,
}

impl Compiler<'_> {
    pub(crate) fn new<'a>(input: &'a str, filename: &'a str) -> Compiler<'a> {
        Compiler {
            input,
            filename,
            lexer: lexer::Lexer::new(),
            parser: parser::Parser::new(),
        }
    }

    pub(crate) fn compile(&mut self) -> Result<AST, String> {
        self.lexer.lex(self.input);

        let mut self_clone = self.clone(); // Clone the `self` reference
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

            errors.into_iter().for_each(|error| {
                self_clone.generate_error(&error); // Use the cloned `self` reference
            });
        }

        trace!("{:#04}..{:#04} {}", "Byte", "Rnge", "Token");

        trace!("{:-<1$}", "", 40);
        for token in self.lexer.get_tokens_peekable() {
            trace!("{}", token);
        }
        let mut token_stream = self.lexer.get_tokens_peekable();
        let ast = self.parser.parse(&mut token_stream).map_err(|e| {
            self_clone.generate_error(&e);
            return format!("Error parsing tokens: {}", e.error_message);
        });

        ast
    }

    fn generate_error(&mut self, error_detail: &CompilerError) {
        use ariadne::{ColorGenerator, Label, Report, ReportKind, Source};

        let mut colors = ColorGenerator::new();

        let color_1 = colors.next();

        let mut report = Report::build(ReportKind::Error, self.filename, 0)
            .with_code(&error_detail.error_code)
            .with_message(&error_detail.error_message)
            .with_label(
                Label::new((self.filename, error_detail.token.span.clone()))
                    .with_message(&error_detail.span_message)
                    .with_color(color_1),
            );

        if let Some(help) = &error_detail.help {
            report = report.with_help(help);
        }

        if let Some(info) = &error_detail.info {
            report = report.with_note(info);
        }

        report
            .finish()
            .eprint((self.filename, Source::from(self.input)))
            .unwrap();
    }
}
