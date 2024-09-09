use crate::interpreter::token_kind::*;
use crate::utils::IteratorExt;

#[derive(Clone, Debug)]
pub struct Tokenizer<'a> {
    input: &'a str,
    position: usize,
    peeked: Option<char>,
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(c) = self.peeked.take() {
            self.position += c.len_utf8();
            Some(c)
        } else {
            self.input[self.position..].chars().next().map(|c| {
                self.position += c.len_utf8();
                c
            })
        }
    }
}

impl<'a> Tokenizer<'a> {
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(c) = self.peek() {
            match c {
                // Skip whitespace
                ' ' | '\t' | '\n' | '\r' => {
                    self.advance();
                }
                // Handle keywords and identifiers
                'a'..='z' | 'A'..='Z' | '_' => {
                    let start = self.position;
                    while let Some(c) = self.peek() {
                        if c.is_alphanumeric() || c == '_' {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    let value = &self.input[start..self.position];
                    let kind = match value {
                        "let" => TokenKind::Keyword(KeywordTokenKind::Let),
                        "const" => TokenKind::Keyword(KeywordTokenKind::Const),
                        "if" => TokenKind::Keyword(KeywordTokenKind::If),
                        "else" => TokenKind::Keyword(KeywordTokenKind::Else),
                        "while" => TokenKind::Keyword(KeywordTokenKind::While),
                        "do" => TokenKind::Keyword(KeywordTokenKind::Do),
                        "for" => TokenKind::Keyword(KeywordTokenKind::For),
                        "function" => TokenKind::Keyword(KeywordTokenKind::Function),
                        "return" => TokenKind::Keyword(KeywordTokenKind::Return),
                        "break" => TokenKind::Keyword(KeywordTokenKind::Break),
                        "continue" => TokenKind::Keyword(KeywordTokenKind::Continue),
                        _ => TokenKind::Literal(LiteralTokenKind::Identifier),
                    };
                    tokens.push(Token { kind, value: value.to_string() });
                }
                // Handle numbers
                '0'..='9' => {
                    let start = self.position;
                    while let Some(c) = self.peek() {
                        if c.is_numeric() {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    let value = &self.input[start..self.position];
                    tokens.push(Token { kind: TokenKind::Literal(LiteralTokenKind::Number), value: value.to_string() });
                }
                // Handle strings
                '"' => {
                    self.advance();
                    let start = self.position;
                    while let Some(c) = self.peek() {
                        if c == '"' {
                            break;
                        }
                        self.advance();
                    }
                    let value = &self.input[start..self.position];
                    self.advance();
                    tokens.push(Token { kind: TokenKind::Literal(LiteralTokenKind::String), value: value.to_string() });
                }
                // Handle operators
                '+' | '-' | '*' | '/' | '%' | '^' | '&' | '|' | '!' | '=' | '<' | '>' => {
                    let start = self.position;
                    let start_c = c;
                    while let Some(c) = self.peek() {
                        match c {
                            '<' | '>' | '/' => {
                                if start_c == start_c {
                                    self.advance();
                                    // break if // or <<< or >>>
                                    if start_c == '/' || (matches!(start_c, '<' | '>') && start == self.position - 2) {
                                        break;
                                    }
                                }

                                break;
                            }
                            '=' => {
                                self.advance();
                                break;
                            }
                            _ => {
                                break;
                            }
                        }
                    }
                    let value = &self.input[start..self.position];
                    let kind = match value {
                        "+" => TokenKind::Operator(OperatorTokenKind::Plus),
                        "-" => TokenKind::Operator(OperatorTokenKind::Minus),
                        "*" => TokenKind::Operator(OperatorTokenKind::Star),
                        "/" => TokenKind::Operator(OperatorTokenKind::Slash),
                        "%" => TokenKind::Operator(OperatorTokenKind::Percent),
                        "^" => TokenKind::Operator(OperatorTokenKind::Caret),
                        "&" => TokenKind::Operator(OperatorTokenKind::And),
                        "|" => TokenKind::Operator(OperatorTokenKind::Or),
                        "!" => TokenKind::Operator(OperatorTokenKind::Not),
                        "=" => TokenKind::Operator(OperatorTokenKind::Equal),
                        "!=" => TokenKind::Operator(OperatorTokenKind::NotEqual),
                        "<" => TokenKind::Operator(OperatorTokenKind::Less),
                        "<<" => TokenKind::Operator(OperatorTokenKind::DoubleLess),
                        "<<<" => TokenKind::Operator(OperatorTokenKind::TripleLess),
                        ">" => TokenKind::Operator(OperatorTokenKind::Greater),
                        ">>" => TokenKind::Operator(OperatorTokenKind::DoubleGreater),
                        ">>>" => TokenKind::Operator(OperatorTokenKind::TripleGreater),
                        "<=" => TokenKind::Operator(OperatorTokenKind::LessEqual),
                        ">=" => TokenKind::Operator(OperatorTokenKind::GreaterEqual),
                        ":=" => TokenKind::Operator(OperatorTokenKind::Assign),
                        "+=" => TokenKind::Operator(OperatorTokenKind::PlusAssign),
                        "-=" => TokenKind::Operator(OperatorTokenKind::MinusAssign),
                        "*=" => TokenKind::Operator(OperatorTokenKind::StarAssign),
                        "/=" => TokenKind::Operator(OperatorTokenKind::SlashAssign),
                        "%=" => TokenKind::Operator(OperatorTokenKind::PercentAssign),
                        "//" => TokenKind::Comment,
                        _ => unreachable!(),
                    };
                    tokens.push(Token { kind, value: value.to_string() });
                }
                // Handle punctuation
                '(' => {
                    self.advance();
                    tokens.push(Token { kind: TokenKind::Punctuation(PunctuationTokenKind::LParen), value: "(".to_string() });
                }
                ')' => {
                    self.advance();
                    tokens.push(Token { kind: TokenKind::Punctuation(PunctuationTokenKind::RParen), value: ")".to_string() });
                }
                '{' => {
                    self.advance();
                    tokens.push(Token { kind: TokenKind::Punctuation(PunctuationTokenKind::LBrace), value: "{".to_string() });
                }
                '}' => {
                    self.advance();
                    tokens.push(Token { kind: TokenKind::Punctuation(PunctuationTokenKind::RBrace), value: "}".to_string() });
                }
                '[' => {
                    self.advance();
                    tokens.push(Token { kind: TokenKind::Punctuation(PunctuationTokenKind::LBracket), value: "[".to_string() });
                }
                ']' => {
                    self.advance();
                    tokens.push(Token { kind: TokenKind::Punctuation(PunctuationTokenKind::RBracket), value: "]".to_string() });
                }
                ';' => {
                    self.advance();
                    tokens.push(Token { kind: TokenKind::Punctuation(PunctuationTokenKind::Semicolon), value: ";".to_string() });
                }
                ',' => {
                    self.advance();
                    tokens.push(Token { kind: TokenKind::Punctuation(PunctuationTokenKind::Comma), value: ",".to_string() });
                }
                '.' => {
                    self.advance();
                    tokens.push(Token { kind: TokenKind::Punctuation(PunctuationTokenKind::Dot), value: ".".to_string() });
                }

                _ => {
                    self.advance();
                }
            }
        }

        tokens.push(Token { kind: TokenKind::EOF, value: "".to_string() });

        tokens
    }
}