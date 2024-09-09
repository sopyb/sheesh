#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
pub enum LiteralTokenKind {
    Number,
    String,
    Identifier,
}

#[derive(Copy, Clone)]
pub enum TokenKind {
    Keyword(KeywordTokenKind),
    Operator(OperatorTokenKind),
    Punctuation(PunctuationTokenKind),
    Literal(LiteralTokenKind),
    Comment,
    EOF,
}

#[derive(Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
}