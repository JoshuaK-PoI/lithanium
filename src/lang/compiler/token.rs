pub(crate) type Span = std::ops::Range<usize>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenType {
    // Special tokens
    Unknown,

    // Single char tokens
    At,
    Colon,
    Equals,
    LeftBrace,
    LeftParen,
    RightBrace,
    RightParen,
    Semicolon,
    Star,

    // Multi char tokens / keywords
    Function,
    Let,
    Return,

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

pub(crate) static KEYWORDS: phf::Map<&'static str, TokenType> = phf::phf_map! {
    "let" => TokenType::Let,
    "function" => TokenType::Function,
    "return" => TokenType::Return,
};

#[derive(Debug, Clone)]
pub(crate) struct Token {
    pub(crate) type_: TokenType,
    pub(crate) value: String,
    pub(crate) span: Span,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#04}..{:#04} {:16} '{}'", self.span.start, self.span.end, self.type_, self.value)
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

