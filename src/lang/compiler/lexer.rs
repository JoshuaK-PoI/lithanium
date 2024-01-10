use log::debug;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenType {
    // Special tokens
    Unknown,

    // Single char tokens
    /// =
    Equals,

    /// ;
    Semicolon,

    // Multi char tokens / keywords

    /// let
    Let,

    
    // N-char tokens


    /// Identifier
    Identifier,

    /// Integer literals
    IntegerLiteral,

}

static KEYWORDS: phf::Map<&'static str, TokenType> = phf::phf_map! {
    "let" => TokenType::Let,
};

#[derive(Debug)]
pub(crate) struct Token {
    pub(crate) type_: TokenType,
    pub(crate) value: String,
    pub(crate) span: std::ops::Range<usize>
}

#[derive(Debug)]
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
        
        loop {
            let start_position = self.position;

            match self.read_char() {
                Some('=') => return Some(make_token!(Equals, String::from("="), start_position..self.position)),
                Some(';') => return Some(make_token!(Semicolon, String::from(";"), start_position..self.position)),
                Some(c) if c.is_ascii_alphabetic() => {
                    let str = self.continue_while(c, |c| c.is_ascii_alphanumeric() || *c == '_');
                    let ident = str.into_iter().collect::<String>();

                    if let Some(keyword) = KEYWORDS.get(ident.as_str()) {
                        return Some(Token {
                            type_: *keyword,
                            value: ident,
                            span: start_position..self.position,
                        });
                    }

                    return Some(make_token!(Identifier, ident, start_position..self.position));
                }

                Some(c) if c.is_ascii_digit() => {
                    let str = self.continue_while(c, |c| c.is_ascii_digit());
                    let ident = str.into_iter().collect::<String>();
                    return Some(make_token!(IntegerLiteral, ident, start_position..self.position));
                }

                Some(c) => return Some(make_token!(Unknown, String::from(c), start_position..self.position)),

                None => return None,
            }
        }
    }
}

impl Lexer<'_> {
    pub(crate) fn new(input: &str) -> Lexer {
        let mut lexer = Lexer {
            chars: input.chars().peekable(),
            tokens: Vec::new(),
            position: 0,
        };

        lexer
    }

    pub(crate) fn lex(&mut self) -> () {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::debug;

    fn before_each() {
        let _ = env_logger::builder()
            .filter_level(log::LevelFilter::max())
            .is_test(true)
            .try_init()
            .unwrap();
        // Logger should be set to debug level for tests
        debug!("Logger initialized");
    }
    
    #[test]
    fn test_lexer() {
        before_each();
        let input = "let x = 5;";
        let mut lexer = Lexer::new(input);
        lexer.lex();
        debug!("{:#?}", lexer.tokens);
        assert_eq!(lexer.tokens.len(), 5);

        /*
         * Expected output:
         *
         * Token {
         *    type_: TokenType::Let,
         *    value: "let",
         *    span: 0..3,
         * },
         * 
         * Token {
         *    type_: TokenType::Identifier,
         *    value: "x",
         *    span: 4..5,
         * },
         * 
         * Token {
         *    type_: TokenType::Equals,
         *    value: "=",
         *    span: 6..7,
         * },
         *
         * Token {
         *    type_: TokenType::IntegerLiteral,
         *    value: "5",
         *    span: 8..9,
         * },
         *
         * Token {
         *   type_: TokenType::Semicolon,
         *   value: ";",
         *   span: 9..10,
         * },
         */

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
