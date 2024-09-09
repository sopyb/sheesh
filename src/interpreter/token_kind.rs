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

pub enum LiteralTokenKind {
    Number,
    String,
    Identifier,
}

pub enum TokenKind {
    Keyword(KeywordTokenKind),
    Operator(OperatorTokenKind),
    Punctuation(PunctuationTokenKind),
    Literal(LiteralTokenKind),
    Comment,
    EOF,
}

pub struct Token {
    pub kind: TokenKind,
    pub value: String,
}