use crate::interpreter::token_kind::{Token, TokenKind};
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
    pub fn new(input: &'a str) -> Self {
        Tokenizer { input, position: 0, peeked: None }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(c) = self.peek() {
            match c {
                // implement the tokenization logic here
                _ => {
                    self.advance();
                }
            }
        }
        tokens
    }
}