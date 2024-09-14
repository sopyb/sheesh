use sheesh::ast::*;
use sheesh::interpreter::token_kind::*;
use sheesh::parser::Parser;

#[test]
fn test_parse_let_declaration() {
    let tokens = vec![
        Token::new(TokenKind::Keyword(KeywordTokenKind::Let), "let"),
        Token::new(TokenKind::Literal(LiteralTokenKind::Identifier), "x"),
        Token::new(TokenKind::Operator(OperatorTokenKind::Assign), "="),
        Token::new(TokenKind::Literal(LiteralTokenKind::Number), "42"),
        Token::new(TokenKind::Punctuation(PunctuationTokenKind::Semicolon), ";"),
        Token::new(TokenKind::EOF, ""),
    ];

    let mut parser = Parser::new(tokens);
    let result = parser.parse().unwrap();

    assert_eq!(result.len(), 1);
    assert!(matches!(
        result[0],
        Stmt::Let(ref name, Expr::Number(value)) if name == "x" && value == 42.0
    ));
}

#[allow(clippy::approx_constant)]
#[test]
fn test_parse_const_declaration() {
    let tokens = vec![
        Token::new(TokenKind::Keyword(KeywordTokenKind::Const), "const"),
        Token::new(TokenKind::Literal(LiteralTokenKind::Identifier), "PI"),
        Token::new(TokenKind::Operator(OperatorTokenKind::Assign), "="),
        Token::new(TokenKind::Literal(LiteralTokenKind::Number), "3.14159"),
        Token::new(TokenKind::Punctuation(PunctuationTokenKind::Semicolon), ";"),
        Token::new(TokenKind::EOF, ""),
    ];

    let mut parser = Parser::new(tokens);
    let result = parser.parse().unwrap();

    assert_eq!(result.len(), 1);
    assert!(matches!(
        result[0],
        Stmt::Const(ref name, Expr::Number(value)) if name == "PI" && (value - 3.14159).abs() < f64::EPSILON
    ));
}

#[test]
fn test_parse_function_declaration() {
    let tokens = vec![
        Token::new(TokenKind::Keyword(KeywordTokenKind::Function), "function"),
        Token::new(TokenKind::Literal(LiteralTokenKind::Identifier), "add"),
        Token::new(TokenKind::Punctuation(PunctuationTokenKind::LParen), "("),
        Token::new(TokenKind::Literal(LiteralTokenKind::Identifier), "a"),
        Token::new(TokenKind::Punctuation(PunctuationTokenKind::Comma), ","),
        Token::new(TokenKind::Literal(LiteralTokenKind::Identifier), "b"),
        Token::new(TokenKind::Punctuation(PunctuationTokenKind::RParen), ")"),
        Token::new(TokenKind::Punctuation(PunctuationTokenKind::LBrace), "{"),
        Token::new(TokenKind::Keyword(KeywordTokenKind::Return), "return"),
        Token::new(TokenKind::Literal(LiteralTokenKind::Identifier), "a"),
        Token::new(TokenKind::Operator(OperatorTokenKind::Plus), "+"),
        Token::new(TokenKind::Literal(LiteralTokenKind::Identifier), "b"),
        Token::new(TokenKind::Punctuation(PunctuationTokenKind::Semicolon), ";"),
        Token::new(TokenKind::Punctuation(PunctuationTokenKind::RBrace), "}"),
        Token::new(TokenKind::EOF, ""),
    ];

    let mut parser = Parser::new(tokens);
    let result = parser.parse().unwrap();

    assert_eq!(result.len(), 1);
    if let Stmt::Function(name, params, body) = &result[0] {
        assert_eq!(name, "add");
        assert_eq!(params, &vec!["a".to_string(), "b".to_string()]);
        if let Stmt::Block(statements) = &**body {
            assert_eq!(statements.len(), 1);
            assert!(matches!(
                statements[0].clone(),
                Stmt::Return(Some(Expr::Binary(
                    left,
                    BinOp::Add,
                    right
                ))) if matches!(&*left, Expr::Identifier(ref left_name) if left_name == "a") && matches!(&*right, Expr::Identifier(ref right_name) if right_name == "b")
            ));
        } else {
            panic!("Expected function body to be a block");
        }
    } else {
        panic!("Expected function declaration");
    }
}

#[test]
fn test_parse_if_statement() {
    let tokens = vec![
        Token::new(TokenKind::Keyword(KeywordTokenKind::If), "if"),
        Token::new(TokenKind::Punctuation(PunctuationTokenKind::LParen), "("),
        Token::new(TokenKind::Literal(LiteralTokenKind::Identifier), "x"),
        Token::new(TokenKind::Operator(OperatorTokenKind::Greater), ">"),
        Token::new(TokenKind::Literal(LiteralTokenKind::Number), "0"),
        Token::new(TokenKind::Punctuation(PunctuationTokenKind::RParen), ")"),
        Token::new(TokenKind::Punctuation(PunctuationTokenKind::LBrace), "{"),
        Token::new(TokenKind::Keyword(KeywordTokenKind::Return), "return"),
        Token::new(TokenKind::Literal(LiteralTokenKind::String), "\"positive\""),
        Token::new(TokenKind::Punctuation(PunctuationTokenKind::Semicolon), ";"),
        Token::new(TokenKind::Punctuation(PunctuationTokenKind::RBrace), "}"),
        Token::new(TokenKind::Keyword(KeywordTokenKind::Else), "else"),
        Token::new(TokenKind::Punctuation(PunctuationTokenKind::LBrace), "{"),
        Token::new(TokenKind::Keyword(KeywordTokenKind::Return), "return"),
        Token::new(
            TokenKind::Literal(LiteralTokenKind::String),
            "\"non-positive\"",
        ),
        Token::new(TokenKind::Punctuation(PunctuationTokenKind::Semicolon), ";"),
        Token::new(TokenKind::Punctuation(PunctuationTokenKind::RBrace), "}"),
        Token::new(TokenKind::EOF, ""),
    ];

    let mut parser = Parser::new(tokens);
    let result = parser.parse().unwrap();

    assert_eq!(result.len(), 1);
    if let Stmt::If(condition, then_branch, else_branch) = &result[0] {
        assert!(matches!(
            condition,
            Expr::Binary(
                ref left,
                BinOp::Greater,
                ref right
            ) if matches!(&**left, Expr::Identifier(ref left_name) if left_name == "x") && matches!(&**right, Expr::Number(right_val) if *right_val == 0.0)
        ));

        if let Stmt::Block(then_statements) = &**then_branch {
            assert_eq!(then_statements.len(), 1);
            assert!(matches!(
                then_statements[0],
                Stmt::Return(Some(Expr::String(ref s))) if s == "positive"
            ));
        } else {
            panic!("Expected then branch to be a block");
        }

        if let Some(else_branch) = else_branch {
            if let Stmt::Block(else_statements) = &**else_branch {
                assert_eq!(else_statements.len(), 1);
                assert!(matches!(
                    else_statements[0],
                    Stmt::Return(Some(Expr::String(ref s))) if s == "non-positive"
                ));
            } else {
                panic!("Expected else branch to be a block");
            }
        }
    } else {
        panic!("Expected if statement");
    }
}

#[test]
fn test_parse_while_loop() {
    let tokens = vec![
        Token::new(TokenKind::Keyword(KeywordTokenKind::While), "while"),
        Token::new(TokenKind::Punctuation(PunctuationTokenKind::LParen), "("),
        Token::new(TokenKind::Literal(LiteralTokenKind::Identifier), "i"),
        Token::new(TokenKind::Operator(OperatorTokenKind::Less), "<"),
        Token::new(TokenKind::Literal(LiteralTokenKind::Number), "10"),
        Token::new(TokenKind::Punctuation(PunctuationTokenKind::RParen), ")"),
        Token::new(TokenKind::Punctuation(PunctuationTokenKind::LBrace), "{"),
        Token::new(TokenKind::Literal(LiteralTokenKind::Identifier), "i"),
        Token::new(TokenKind::Operator(OperatorTokenKind::PlusAssign), "+="),
        Token::new(TokenKind::Literal(LiteralTokenKind::Number), "1"),
        Token::new(TokenKind::Punctuation(PunctuationTokenKind::Semicolon), ";"),
        Token::new(TokenKind::Punctuation(PunctuationTokenKind::RBrace), "}"),
        Token::new(TokenKind::EOF, ""),
    ];

    let mut parser = Parser::new(tokens);
    let result = parser.parse().unwrap();

    assert_eq!(result.len(), 1);
    if let Stmt::While(condition, body) = &result[0] {
        assert!(matches!(
            condition,
            Expr::Binary(
                ref left,
                BinOp::Less,
                ref right
            ) if matches!(&**left, Expr::Identifier(ref left_name) if left_name == "i") && matches!(&**right, Expr::Number(right_val) if *right_val == 10.0)
        ));

        if let Stmt::Block(statements) = &**body {
            assert_eq!(statements.len(), 1);
            assert!(matches!(
                statements[0],
                Stmt::Expr(Expr::Assign(
                    ref left,
                    ref right
                )) if matches!(&**left, Expr::Identifier(ref left_name) if left_name == "i") && matches!(&**right, Expr::Binary(
                    ref left,
                    BinOp::Add,
                    ref right
                ) if matches!(&**left, Expr::Identifier(ref left_name) if left_name == "i") && matches!(&**right, Expr::Number(right_val) if *right_val == 1.0))));
        } else {
            panic!("Expected while loop body to be a block");
        }
    } else {
        panic!("Expected while loop");
    }
}

#[test]
fn test_parse_complex_expression() {
    let tokens = vec![
        // 2 + 3 * 4 - 5 / 2;
        Token::new(TokenKind::Literal(LiteralTokenKind::Number), "2"),
        Token::new(TokenKind::Operator(OperatorTokenKind::Plus), "+"),
        Token::new(TokenKind::Literal(LiteralTokenKind::Number), "3"),
        Token::new(TokenKind::Operator(OperatorTokenKind::Star), "*"),
        Token::new(TokenKind::Literal(LiteralTokenKind::Number), "4"),
        Token::new(TokenKind::Operator(OperatorTokenKind::Minus), "-"),
        Token::new(TokenKind::Literal(LiteralTokenKind::Number), "5"),
        Token::new(TokenKind::Operator(OperatorTokenKind::Slash), "/"),
        Token::new(TokenKind::Literal(LiteralTokenKind::Number), "2"),
        Token::new(TokenKind::Punctuation(PunctuationTokenKind::Semicolon), ";"),
        Token::new(TokenKind::EOF, ""),
    ];

    let mut parser = Parser::new(tokens);
    let result = parser.parse().unwrap();

    assert_eq!(result.len(), 1);

    // (2 + (3 * 4)) - (5 / 2);
    if let Stmt::Expr(expr) = &result[0] {
        assert!(matches!(
            expr,
            Expr::Binary(

                // 2 + 3 * 4
                left,
                BinOp::Subtract,
                // 5 / 2
                right
            ) if matches!(&**left, Expr::Binary(
                // 2
                left,
                BinOp::Add,
                // 3 * 4
                right
            ) if matches!(&**left, Expr::Number(left_val) if *left_val == 2.0) && matches!(&**right, Expr::Binary(
                // 3
                left,
                BinOp::Multiply,
                // 4
                right
            ) if matches!(&**left, Expr::Number(left_val) if *left_val == 3.0) && matches!(&**right, Expr::Number(right_val) if *right_val == 4.0))) && matches!(&**right, Expr::Binary(
                // 5
                left,
                BinOp::Divide,
                // 2
                right
            ) if matches!(&**left, Expr::Number(left_val) if *left_val == 5.0) && matches!(&**right, Expr::Number(right_val) if *right_val == 2.0)))
        );
    } else {
        panic!("Expected expression statement");
    }
}

#[test]
fn test_parse_unary_expression() {
    let tokens = vec![
        Token::new(TokenKind::Operator(OperatorTokenKind::Minus), "-"),
        Token::new(TokenKind::Literal(LiteralTokenKind::Number), "5"),
        Token::new(TokenKind::Punctuation(PunctuationTokenKind::Semicolon), ";"),
        Token::new(TokenKind::EOF, ""),
    ];

    let mut parser = Parser::new(tokens);
    let result = parser.parse().unwrap();

    assert_eq!(result.len(), 1);
    if let Stmt::Expr(expr) = &result[0] {
        assert!(matches!(
            expr,
            Expr::Unary(UnOp::Negate, expr) if matches!(&**expr, Expr::Number(value) if *value == 5.0)
        ));
    } else {
        panic!("Expected expression statement");
    }
}

#[test]
fn test_parse_function_call() {
    let tokens = vec![
        Token::new(TokenKind::Literal(LiteralTokenKind::Identifier), "print"),
        Token::new(TokenKind::Punctuation(PunctuationTokenKind::LParen), "("),
        Token::new(
            TokenKind::Literal(LiteralTokenKind::String),
            "\"Hello, World!\"",
        ),
        Token::new(TokenKind::Punctuation(PunctuationTokenKind::RParen), ")"),
        Token::new(TokenKind::Punctuation(PunctuationTokenKind::Semicolon), ";"),
        Token::new(TokenKind::EOF, ""),
    ];

    let mut parser = Parser::new(tokens);
    let result = parser.parse().unwrap();

    assert_eq!(result.len(), 1);
    if let Stmt::Expr(Expr::Call(callee, args)) = &result[0] {
        assert!(matches!(&**callee, Expr::Identifier(name) if name == "print"));
        assert_eq!(args.len(), 1);
        assert!(matches!(&args[0], Expr::String(s) if s == "Hello, World!"));
    } else {
        panic!("Expected function call");
    }
}

#[test]
fn test_multi_operation_box() {
    let tokens = vec![
        Token::new(TokenKind::Keyword(KeywordTokenKind::Function), "fun"),
        Token::new(TokenKind::Literal(LiteralTokenKind::Identifier), "add"),
        Token::new(TokenKind::Punctuation(PunctuationTokenKind::LParen), "("),
        Token::new(TokenKind::Literal(LiteralTokenKind::Identifier), "a"),
        Token::new(TokenKind::Punctuation(PunctuationTokenKind::Comma), ","),
        Token::new(TokenKind::Literal(LiteralTokenKind::Identifier), "b"),
        Token::new(TokenKind::Punctuation(PunctuationTokenKind::RParen), ")"),
        Token::new(TokenKind::Punctuation(PunctuationTokenKind::LBrace), "{"),
        Token::new(TokenKind::Literal(LiteralTokenKind::Identifier), "a"),
        Token::new(TokenKind::Operator(OperatorTokenKind::Assign), "="),
        Token::new(TokenKind::Literal(LiteralTokenKind::Number), "1"),
        Token::new(TokenKind::Operator(OperatorTokenKind::Plus), "+"),
        Token::new(TokenKind::Literal(LiteralTokenKind::Identifier), "b"),
        Token::new(TokenKind::Punctuation(PunctuationTokenKind::Semicolon), ";"),
        Token::new(TokenKind::Keyword(KeywordTokenKind::Return), "return"),
        Token::new(TokenKind::Literal(LiteralTokenKind::Identifier), "a"),
        Token::new(TokenKind::Punctuation(PunctuationTokenKind::Semicolon), ";"),
        Token::new(TokenKind::Punctuation(PunctuationTokenKind::RBrace), "}"),
        Token::new(TokenKind::EOF, ""),
    ];

    let mut parser = Parser::new(tokens);
    let result = parser.parse().unwrap();

    assert_eq!(result.len(), 1);

    dbg!(&result);

    if let Stmt::Function(name, params, body) = &result[0] {
        assert_eq!(name, "add");
        assert_eq!(params, &vec!["a".to_string(), "b".to_string()]);

        if let Stmt::Block(statements) = &**body {
            assert_eq!(statements.len(), 2);

            assert!(matches!(
                statements[0].clone(),
                Stmt::Expr(Expr::Assign(
                    left,
                    right
                )) if matches!(&*left, Expr::Identifier(ref left_name) if left_name == "a") && matches!(&*right, Expr::Binary(
                    left,
                    BinOp::Add,
                    right
                ) if matches!(&**left, Expr::Number(value) if *value == 1.0) && matches!(&**right, Expr::Identifier(ref right_name) if right_name == "b"))
            ));

            assert!(matches!(
                statements[1],
                Stmt::Return(Some(Expr::Identifier(ref name))) if name == "a"
            ));
        } else {
            panic!("Expected function body to be a block");
        }
    } else {
        panic!("Expected function declaration");
    }
}