use super::{token::TokenStream, CompilerResult, statement::Statement};
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

    pub(crate) fn parse<'a>(&'a mut self, tokens: &mut TokenStream) -> CompilerResult<AST> {
        let mut ast = AST {
            statements: Vec::new(),
        };

        while let Some(token) = tokens.peek() {
            ast.statements.push(Statement::parse(tokens)?);
        }

        Ok(ast)
    }
}