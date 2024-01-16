use super::token::{Token, TokenStream, TokenType, KEYWORDS};

#[derive(Debug, Clone)]
pub(crate) struct Lexer<'a> {
    pub(crate) chars: std::iter::Peekable<std::str::Chars<'a>>,
    pub(crate) tokens: Vec<Token>,
    position: usize,
}

macro_rules! make_token {
    ($type_:ident, $value:expr, $span:expr) => {
        Token {
            type_: TokenType::$type_,
            value: $value,
            span: $span,
        }
    };
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        let start_position = self.position;

        match self.read_char() {
            Some('@') => Some(make_token!(
                At,
                String::from("@"),
                start_position..self.position
            )),
            Some(',') => Some(make_token!(
                Comma,
                String::from(","),
                start_position..self.position
            )),
            Some(':') => Some(make_token!(
                Colon,
                String::from(":"),
                start_position..self.position
            )),
            Some('=') => Some(make_token!(
                Equals,
                String::from("="),
                start_position..self.position
            )),
            Some('{') => Some(make_token!(
                LeftBrace,
                String::from("{"),
                start_position..self.position
            )),
            Some('(') => Some(make_token!(
                LeftParen,
                String::from("("),
                start_position..self.position
            )),
            Some(')') => Some(make_token!(
                RightParen,
                String::from(")"),
                start_position..self.position
            )),
            Some('}') => Some(make_token!(
                RightBrace,
                String::from("}"),
                start_position..self.position
            )),
            Some(';') => Some(make_token!(
                Semicolon,
                String::from(";"),
                start_position..self.position
            )),
            Some('*') => Some(make_token!(
                Star,
                String::from("*"),
                start_position..self.position
            )),
            Some(c) if c.is_ascii_alphabetic() => {
                let str = self.continue_while(c, |c| c.is_ascii_alphanumeric() || *c == '_');
                let ident = str.into_iter().collect::<String>();

                if let Some(keyword) = KEYWORDS.get(ident.as_str()) {
                    Some(Token {
                        type_: *keyword,
                        value: ident,
                        span: start_position..self.position,
                    })
                } else {
                    Some(make_token!(
                        Identifier,
                        ident,
                        start_position..self.position
                    ))
                }
            }

            Some(c) if c.is_ascii_digit() => {
                let str = self.continue_while(c, |c| c.is_ascii_digit());
                let ident = str.into_iter().collect::<String>();
                Some(make_token!(
                    IntegerLiteral,
                    ident,
                    start_position..self.position
                ))
            }

            Some(c) => Some(make_token!(
                Unknown,
                String::from(c),
                start_position..self.position
            )),
            None => return None,
        }
    }
}

impl<'a> Lexer<'a> {
    pub(crate) fn new() -> Lexer<'a> {
        Lexer {
            chars: "".chars().peekable(),
            tokens: Vec::new(),
            position: 0,
        }
    }

    pub(crate) fn lex(&mut self, input: &'a str) -> () {
        self.tokens.clear();
        self.chars = input.chars().peekable();
        while let Some(token) = self.next() {
            self.tokens.push(token);
        }
    }

    fn skip_whitespace(&mut self) -> () {
        while let Some(_) = self.chars.next_if(|c| c.is_whitespace()) {
            self.position += 1;
        }
    }

    fn read_char(&mut self) -> Option<char> {
        let c = self.chars.next();
        if c.is_some() {
            self.position += 1;
        }
        c
    }

    fn continue_while(&mut self, c: char, f: impl Fn(&char) -> bool) -> Vec<char> {
        let mut v = vec![c];
        while let Some(c) = self.chars.next_if(&f) {
            self.position += 1;
            v.push(c);
        }

        v
    }

    pub(crate) fn get_tokens_peekable(&mut self) -> TokenStream {
        self.tokens
            .iter_mut()
            .peekable()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::before_each;

    #[test]
    fn test_lexer() {
        before_each();
        let input = "let x = 5;";
        let mut lexer = Lexer::new();
        lexer.lex(input);
        assert_eq!(lexer.tokens.len(), 5);

        assert_eq!(lexer.tokens[0].type_, TokenType::Let);
        assert_eq!(lexer.tokens[0].value, "let");
        assert_eq!(lexer.tokens[0].span, 0..3);

        assert_eq!(lexer.tokens[1].type_, TokenType::Identifier);
        assert_eq!(lexer.tokens[1].value, "x");
        assert_eq!(lexer.tokens[1].span, 4..5);

        assert_eq!(lexer.tokens[2].type_, TokenType::Equals);
        assert_eq!(lexer.tokens[2].value, "=");
        assert_eq!(lexer.tokens[2].span, 6..7);

        assert_eq!(lexer.tokens[3].type_, TokenType::IntegerLiteral);
        assert_eq!(lexer.tokens[3].value, "5");
        assert_eq!(lexer.tokens[3].span, 8..9);

        assert_eq!(lexer.tokens[4].type_, TokenType::Semicolon);
        assert_eq!(lexer.tokens[4].value, ";");
        assert_eq!(lexer.tokens[4].span, 9..10);
    }
}
