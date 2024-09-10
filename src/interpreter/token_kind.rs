#[derive(Copy, Clone, Debug, PartialEq)]
pub enum KeywordTokenKind {
    Let,
    Const,
    If,
    Else,
    While,
    Do,
    For,
    Function,
    Return,
    Break,
    Continue,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum OperatorTokenKind {
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    And,
    Or,
    Not,
    Equal,
    NotEqual,
    Less,
    DoubleLess,
    TripleLess,
    Greater,
    DoubleGreater,
    TripleGreater,
    LessEqual,
    GreaterEqual,
    Assign,
    PlusAssign,
    MinusAssign,
    StarAssign,
    SlashAssign,
    PercentAssign,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PunctuationTokenKind {
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Semicolon,
    Comma,
    Dot,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LiteralTokenKind {
    Number,
    String,
    Identifier,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TokenKind {
    Keyword(KeywordTokenKind),
    Operator(OperatorTokenKind),
    Punctuation(PunctuationTokenKind),
    Literal(LiteralTokenKind),
    Comment,
    EOF,
}


#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
}

impl Token {
    pub fn new(kind: TokenKind, value: &str) -> Token {
        Token {
            kind,
            value: value.to_string(),
        }
    }
}