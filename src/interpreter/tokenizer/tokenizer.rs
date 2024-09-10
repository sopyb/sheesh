use crate::interpreter::token_kind::*;
use crate::utils::IteratorExt;
use std::collections::HashMap;

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
    pub fn new(input: &'a str) -> Self {
        Tokenizer {
            input,
            position: 0,
            peeked: None,
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn read_identifier_or_keyword(&mut self) -> Token {
        let start = self.position;
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }
        let value = &self.input[start..self.position];
        let lowercase_value = value.to_lowercase();
        let kind = match lowercase_value.as_str() {
            "let" => TokenKind::Keyword(KeywordTokenKind::Let),
            "const" => TokenKind::Keyword(KeywordTokenKind::Const),
            "if" => TokenKind::Keyword(KeywordTokenKind::If),
            "else" => TokenKind::Keyword(KeywordTokenKind::Else),
            "while" => TokenKind::Keyword(KeywordTokenKind::While),
            "do" => TokenKind::Keyword(KeywordTokenKind::Do),
            "for" => TokenKind::Keyword(KeywordTokenKind::For),
            "fun" => TokenKind::Keyword(KeywordTokenKind::Function),
            "return" => TokenKind::Keyword(KeywordTokenKind::Return),
            "break" => TokenKind::Keyword(KeywordTokenKind::Break),
            "continue" => TokenKind::Keyword(KeywordTokenKind::Continue),
            _ => TokenKind::Literal(LiteralTokenKind::Identifier),
        };
        Token {
            kind,
            value: value.to_string(),
        }
    }

    fn read_number(&mut self) -> Token {
        let start = self.position;
        let mut has_decimal = false;

        while let Some(c) = self.peek() {
            if c.is_numeric() {
                self.advance();
            } else if c == '.' && !has_decimal {
                has_decimal = true;
                self.advance();
            } else {
                break;
            }
        }

        if let Some(c) = self.peek() {
            if c == 'e' || c =='E' {
                self.advance(); // Skip 'e'
                if let Some(c) = self.peek() {
                    if c == '+' || c == '-' {
                        self.advance(); // Skip the sign
                    }
                }
                while let Some(c) = self.peek() {
                    if c.is_numeric() {
                        self.advance();
                    } else {
                        break;
                    }
                }
            }
        }

        let value = &self.input[start..self.position];
        Token {
            kind: TokenKind::Literal(LiteralTokenKind::Number),
            value: value.to_string(),
        }
    }

    fn read_string(&mut self) -> Token {
        self.advance(); // Skip the opening quote
        let start = self.position;
        while let Some(c) = self.peek() {
            if c == '"' {
                break;
            }
            self.advance();
        }
        let value = &self.input[start..self.position];
        self.advance(); // Skip the closing quote
        Token {
            kind: TokenKind::Literal(LiteralTokenKind::String),
            value: value.to_string(),
        }
    }

    fn read_operator(&mut self) -> Token {
        let start = self.position;
        let start_c = self.peek().unwrap();

        // advance past the first character
        self.advance();

        while let Some(c) = self.peek() {
            match c {
                '<' | '>' | '/' => {
                    if start_c == start_c {
                        self.advance();
                        // break if // or <<< or >>>
                        if start_c == '/'
                            || (matches!(start_c, '<' | '>') && start == self.position - 2)
                        {
                            break;
                        }
                    }
                    break;
                }
                '=' => {
                    self.advance();
                    if start_c == '=' {
                        break;
                    }
                    break;
                }
                _ => {
                    break;
                }
            }
        }
        let mut value = &self.input[start..self.position];
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
            "==" => TokenKind::Operator(OperatorTokenKind::Equal),
            "!=" => TokenKind::Operator(OperatorTokenKind::NotEqual),
            "<" => TokenKind::Operator(OperatorTokenKind::Less),
            "<<" => TokenKind::Operator(OperatorTokenKind::DoubleLess),
            "<<<" => TokenKind::Operator(OperatorTokenKind::TripleLess),
            ">" => TokenKind::Operator(OperatorTokenKind::Greater),
            ">>" => TokenKind::Operator(OperatorTokenKind::DoubleGreater),
            ">>>" => TokenKind::Operator(OperatorTokenKind::TripleGreater),
            "<=" => TokenKind::Operator(OperatorTokenKind::LessEqual),
            ">=" => TokenKind::Operator(OperatorTokenKind::GreaterEqual),
            "=" => TokenKind::Operator(OperatorTokenKind::Assign),
            "+=" => TokenKind::Operator(OperatorTokenKind::PlusAssign),
            "-=" => TokenKind::Operator(OperatorTokenKind::MinusAssign),
            "*=" => TokenKind::Operator(OperatorTokenKind::StarAssign),
            "/=" => TokenKind::Operator(OperatorTokenKind::SlashAssign),
            "%=" => TokenKind::Operator(OperatorTokenKind::PercentAssign),
            "//" => TokenKind::Comment,
            _ => unreachable!(),
        };

        if kind == TokenKind::Comment {
            // get everything until the end of the line
            while let Some(c) = self.peek() {
                if c == '\n' {
                    break;
                }
                self.advance();
            }

            value = &self.input[start..self.position];
        }

        Token {
            kind,
            value: value.to_string(),
        }
    }

    fn handle_punctuation(&mut self, c: char) -> Option<Token> {
        let punctuation_map: HashMap<char, PunctuationTokenKind> = HashMap::from([
            ('(', PunctuationTokenKind::LParen),
            (')', PunctuationTokenKind::RParen),
            ('{', PunctuationTokenKind::LBrace),
            ('}', PunctuationTokenKind::RBrace),
            ('[', PunctuationTokenKind::LBracket),
            (']', PunctuationTokenKind::RBracket),
            (';', PunctuationTokenKind::Semicolon),
            (',', PunctuationTokenKind::Comma),
            ('.', PunctuationTokenKind::Dot),
        ]);

        self.advance();

        if let Some(kind) = punctuation_map.get(&c) {
            Some(Token {
                kind: TokenKind::Punctuation(*kind),
                value: c.to_string(),
            })
        } else {
            None
        }
    }
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(c) = self.peek() {
            match c {
                ' ' | '\t' | '\n' | '\r' => self.skip_whitespace(),
                'a'..='z' | 'A'..='Z' | '_' => tokens.push(self.read_identifier_or_keyword()),
                '0'..='9' => tokens.push(self.read_number()),
                '"' => tokens.push(self.read_string()),
                '+' | '-' | '*' | '/' | '%' | '^' | '&' | '|' | '!' | '=' | '<' | '>' => {
                    tokens.push(self.read_operator())
                }
                _ => {
                    if let Some(token) = self.handle_punctuation(c) {
                        tokens.push(token);
                    }
                }
            }
        }

        tokens.push(Token {
            kind: TokenKind::EOF,
            value: "".to_string(),
        });

        tokens
    }
}
