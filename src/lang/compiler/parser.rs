use serde::Serialize;

use super::{token::TokenStream, CompilerResult, statement::{Statement, SpannedStatement}};

#[derive(Debug, Clone, Serialize)]
pub(crate) struct AST {
    statements: Vec<SpannedStatement>
}

#[derive(Debug, Clone)]
pub(crate) struct Parser {}

impl Parser {
    pub(crate) fn parse<'a>(tokens: &mut TokenStream) -> CompilerResult<AST> {
        let mut ast = AST {
            statements: Vec::new(),
        };

        while let Some(_) = tokens.peek() {
            ast.statements.push(Statement::parse(tokens)?);
        }

        Ok(ast)
    }
}