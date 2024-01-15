use crate::lang::compiler::{token::{Token, TokenType}, CompilerResult, CompilerError};



/// Allows vector elements to be taken from the front of the vector.
/// It is a safe wrapper for: 
/// ``` 
/// if vec.is_empty() { 
///     None 
/// } else { 
///     Some(vec.remove(0)) 
/// } 
/// ```
/// that can be used as an expression, similar to `vec.pop()`.
pub(crate) trait Unshift<T> {
    /// Takes the first element of the vector and returns it.
    /// 
    /// Returns `None` if the vector is empty.
    fn unshift(&mut self) -> Option<T>;
}

impl<T> Unshift<T> for Vec<T> {
    fn unshift(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            Some(self.remove(0))
        }
    }
}

pub(crate) trait UnshiftExpect<T, K, E> {
    fn unshift_expect(&mut self, expected: K) -> Result<T, E>;
}

impl UnshiftExpect<Token, TokenType, CompilerError> for Vec<Token> 
{
    fn unshift_expect(&mut self, expected: TokenType) -> CompilerResult<Token>
    {
        match self.unshift() {
            Some(token) => {
                if token.type_ == expected {
                    Ok(token)
                } else {
                    Err(CompilerError {
                        error_code: crate::lang::compiler::ErrorCode::UnexpectedToken,
                        error_message: format!("Expected '{}', got {}", expected, token),
                        span_message: String::from(""),
                        token,
                        help: Some(String::from("Expected one of: - TODO: List expected tokens")),
                        info: None,
                    })
                }
            }
            None => Err(CompilerError {
                error_code: crate::lang::compiler::ErrorCode::UnexpectedToken,
                error_message: format!("Expected token {}, got None", expected),
                span_message: String::from(""),
                token: Token::invalid(),
                help: Some(String::from("Expected one of: - TODO: List expected tokens")),
                info: None,
            }),
        }
    }
}