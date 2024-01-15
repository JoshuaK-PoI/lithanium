use super::{token::{Token, TokenType}, CompilerResult, CompilerError, ErrorCode, statement::{Statement, StatementType, LetStatement}};
use crate::lang::util::vec::Unshift;

#[derive(Debug, Clone)]
pub(crate) struct AST {
    statements: Vec<Statement>
}

#[derive(Debug, Clone)]
pub(crate) struct Parser {}

impl Parser {
    pub(crate) fn new() -> Self {
        Self {}
    }

    pub(crate) fn parse<'a>(&'a mut self, tokens: &'a mut Vec<Token>) -> CompilerResult<AST> {
        let mut ast = AST {
            statements: Vec::new(),
        };

        while let Some(token) = tokens.unshift() {
            ast.statements.push(Statement::parse(token, tokens)?);
        }

        Ok(ast)
    }
}