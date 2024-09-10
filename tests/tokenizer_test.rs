use sosh::interpreter::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenizer() {
        let input = r#"
            let x = 42;
            const y = 3.14;
            if (x == 10) {
                y += x;
            } else {
                y -= 1;
            }

            let z = 0.2E-3;

            let my_string = "Hello, world!";
            while (x < 100) {
                x += 1;
            }

            fun add(a, b) {
                return a + b;
            }

            // This is a comment
        "#;

        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize();

        let expected_tokens = vec![
            // let x = 42;
            Token::new(TokenKind::Keyword(KeywordTokenKind::Let), "let"),
            Token::new(TokenKind::Literal(LiteralTokenKind::Identifier), "x"),
            Token::new(TokenKind::Operator(OperatorTokenKind::Assign), "="),
            Token::new(TokenKind::Literal(LiteralTokenKind::Number), "42"),
            Token::new(TokenKind::Punctuation(PunctuationTokenKind::Semicolon), ";"),
            // const y = 3.14;
            Token::new(TokenKind::Keyword(KeywordTokenKind::Const), "const"),
            Token::new(TokenKind::Literal(LiteralTokenKind::Identifier), "y"),
            Token::new(TokenKind::Operator(OperatorTokenKind::Assign), "="),
            Token::new(TokenKind::Literal(LiteralTokenKind::Number), "3.14"),
            Token::new(TokenKind::Punctuation(PunctuationTokenKind::Semicolon), ";"),
            // if (x == 10) { y += x; } else { y -= 1; }
            Token::new(TokenKind::Keyword(KeywordTokenKind::If), "if"),
            Token::new(TokenKind::Punctuation(PunctuationTokenKind::LParen), "("),
            Token::new(TokenKind::Literal(LiteralTokenKind::Identifier), "x"),
            Token::new(TokenKind::Operator(OperatorTokenKind::Equal), "=="),
            Token::new(TokenKind::Literal(LiteralTokenKind::Number), "10"),
            Token::new(TokenKind::Punctuation(PunctuationTokenKind::RParen), ")"),
            Token::new(TokenKind::Punctuation(PunctuationTokenKind::LBrace), "{"),
            // y += x;
            Token::new(TokenKind::Literal(LiteralTokenKind::Identifier), "y"),
            Token::new(TokenKind::Operator(OperatorTokenKind::PlusAssign), "+="),
            Token::new(TokenKind::Literal(LiteralTokenKind::Identifier), "x"),
            Token::new(TokenKind::Punctuation(PunctuationTokenKind::Semicolon), ";"),
            // } else {
            Token::new(TokenKind::Punctuation(PunctuationTokenKind::RBrace), "}"),
            Token::new(TokenKind::Keyword(KeywordTokenKind::Else), "else"),
            Token::new(TokenKind::Punctuation(PunctuationTokenKind::LBrace), "{"),
            // y -= 1;
            Token::new(TokenKind::Literal(LiteralTokenKind::Identifier), "y"),
            Token::new(TokenKind::Operator(OperatorTokenKind::MinusAssign), "-="),
            Token::new(TokenKind::Literal(LiteralTokenKind::Number), "1"),
            Token::new(TokenKind::Punctuation(PunctuationTokenKind::Semicolon), ";"),
            // }
            Token::new(TokenKind::Punctuation(PunctuationTokenKind::RBrace), "}"),
            // let z = 0.2E-3;
            Token::new(TokenKind::Keyword(KeywordTokenKind::Let), "let"),
            Token::new(TokenKind::Literal(LiteralTokenKind::Identifier), "z"),
            Token::new(TokenKind::Operator(OperatorTokenKind::Assign), "="),
            Token::new(TokenKind::Literal(LiteralTokenKind::Number), "0.2E-3"),
            Token::new(TokenKind::Punctuation(PunctuationTokenKind::Semicolon), ";"),
            // let my_string = "Hello, world!";
            Token::new(TokenKind::Keyword(KeywordTokenKind::Let), "let"),
            Token::new(TokenKind::Literal(LiteralTokenKind::Identifier), "my_string"),
            Token::new(TokenKind::Operator(OperatorTokenKind::Assign), "="),
            Token::new(TokenKind::Literal(LiteralTokenKind::String), "Hello, world!"),
            Token::new(TokenKind::Punctuation(PunctuationTokenKind::Semicolon), ";"),
            // while (x < 100) {
            Token::new(TokenKind::Keyword(KeywordTokenKind::While), "while"),
            Token::new(TokenKind::Punctuation(PunctuationTokenKind::LParen), "("),
            Token::new(TokenKind::Literal(LiteralTokenKind::Identifier), "x"),
            Token::new(TokenKind::Operator(OperatorTokenKind::Less), "<"),
            Token::new(TokenKind::Literal(LiteralTokenKind::Number), "100"),
            Token::new(TokenKind::Punctuation(PunctuationTokenKind::RParen), ")"),
            Token::new(TokenKind::Punctuation(PunctuationTokenKind::LBrace), "{"),
            // x += 1;
            Token::new(TokenKind::Literal(LiteralTokenKind::Identifier), "x"),
            Token::new(TokenKind::Operator(OperatorTokenKind::PlusAssign), "+="),
            Token::new(TokenKind::Literal(LiteralTokenKind::Number), "1"),
            Token::new(TokenKind::Punctuation(PunctuationTokenKind::Semicolon), ";"),
            // }
            Token::new(TokenKind::Punctuation(PunctuationTokenKind::RBrace), "}"),
            // fun add(a, b) {
            Token::new(TokenKind::Keyword(KeywordTokenKind::Function), "fun"),
            Token::new(TokenKind::Literal(LiteralTokenKind::Identifier), "add"),
            Token::new(TokenKind::Punctuation(PunctuationTokenKind::LParen), "("),
            Token::new(TokenKind::Literal(LiteralTokenKind::Identifier), "a"),
            Token::new(TokenKind::Punctuation(PunctuationTokenKind::Comma), ","),
            Token::new(TokenKind::Literal(LiteralTokenKind::Identifier), "b"),
            Token::new(TokenKind::Punctuation(PunctuationTokenKind::RParen), ")"),
            Token::new(TokenKind::Punctuation(PunctuationTokenKind::LBrace), "{"),
            // return a + b;
            Token::new(TokenKind::Keyword(KeywordTokenKind::Return), "return"),
            Token::new(TokenKind::Literal(LiteralTokenKind::Identifier), "a"),
            Token::new(TokenKind::Operator(OperatorTokenKind::Plus), "+"),
            Token::new(TokenKind::Literal(LiteralTokenKind::Identifier), "b"),
            Token::new(TokenKind::Punctuation(PunctuationTokenKind::Semicolon), ";"),
            // }
            Token::new(TokenKind::Punctuation(PunctuationTokenKind::RBrace), "}"),
            // // This is a comment
            Token::new(TokenKind::Comment, "// This is a comment"),
            // EOF token (end of input)
            Token::new(TokenKind::EOF, ""),
        ];

        assert_eq!(tokens.len(), expected_tokens.len());

        for (i, token) in tokens.iter().enumerate() {
            assert_eq!(token.kind, expected_tokens[i].kind, "Token kind mismatch at index {}", i);
            assert_eq!(token.value, expected_tokens[i].value, "Token value mismatch at index {}", i);
        }
    }
}
