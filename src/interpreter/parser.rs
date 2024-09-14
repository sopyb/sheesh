use crate::ast::*;
use crate::interpreter::token_kind::*;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, String> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        Ok(statements)
    }

    fn declaration(&mut self) -> Result<Stmt, String> {
        if self.match_token(&[TokenKind::Keyword(KeywordTokenKind::Let)]) {
            self.let_declaration()
        } else if self.match_token(&[TokenKind::Keyword(KeywordTokenKind::Const)]) {
            self.const_declaration()
        } else if self.match_token(&[TokenKind::Keyword(KeywordTokenKind::Function)]) {
            self.function_declaration()
        } else {
            self.statement()
        }
    }

    fn let_declaration(&mut self) -> Result<Stmt, String> {
        let name = {
            let token = self.consume(
                TokenKind::Literal(LiteralTokenKind::Identifier),
                "Expect variable name.",
            )?;
            token.value.clone()
        };

        let initializer = if self.match_token(&[TokenKind::Operator(OperatorTokenKind::Assign)]) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(
            TokenKind::Punctuation(PunctuationTokenKind::Semicolon),
            "Expect ';' after variable declaration.",
        )?;
        Ok(Stmt::Let(name, initializer.unwrap_or(Expr::Number(0.0))))
    }

    fn const_declaration(&mut self) -> Result<Stmt, String> {
        let name = {
            let token = self.consume(
                TokenKind::Literal(LiteralTokenKind::Identifier),
                "Expect constant name.",
            )?;
            token.value.clone()
        };

        self.consume(
            TokenKind::Operator(OperatorTokenKind::Assign),
            "Expect '=' after constant name.",
        )?;
        let initializer = self.expression()?;
        self.consume(
            TokenKind::Punctuation(PunctuationTokenKind::Semicolon),
            "Expect ';' after constant declaration.",
        )?;
        Ok(Stmt::Const(name, initializer))
    }

    fn function_declaration(&mut self) -> Result<Stmt, String> {
        let name = {
            let token = self.consume(
                TokenKind::Literal(LiteralTokenKind::Identifier),
                "Expect function name.",
            )?;
            token.value.clone()
        };

        self.consume(
            TokenKind::Punctuation(PunctuationTokenKind::LParen),
            "Expect '(' after function name.",
        )?;

        let mut parameters = Vec::new();
        if !self.check(TokenKind::Punctuation(PunctuationTokenKind::RParen)) {
            loop {
                let param = {
                    let token = self.consume(
                        TokenKind::Literal(LiteralTokenKind::Identifier),
                        "Expect parameter name.",
                    )?;
                    token.value.clone()
                };
                parameters.push(param);

                if !self.match_token(&[TokenKind::Punctuation(PunctuationTokenKind::Comma)]) {
                    break;
                }
            }
        }

        self.consume(
            TokenKind::Punctuation(PunctuationTokenKind::RParen),
            "Expect ')' after parameters.",
        )?;
        self.consume(
            TokenKind::Punctuation(PunctuationTokenKind::LBrace),
            "Expect '{' before function body.",
        )?;
        let body = self.block()?;
        Ok(Stmt::Function(name, parameters, Box::new(body)))
    }

    fn statement(&mut self) -> Result<Stmt, String> {
        if self.match_token(&[TokenKind::Keyword(KeywordTokenKind::If)]) {
            self.if_statement()
        } else if self.match_token(&[TokenKind::Keyword(KeywordTokenKind::While)]) {
            self.while_statement()
        } else if self.match_token(&[TokenKind::Keyword(KeywordTokenKind::Return)]) {
            self.return_statement()
        } else if self.match_token(&[TokenKind::Punctuation(PunctuationTokenKind::LBrace)]) {
            Ok(self.block()?)
        } else {
            self.expression_statement()
        }
    }

    fn if_statement(&mut self) -> Result<Stmt, String> {
        self.consume(
            TokenKind::Punctuation(PunctuationTokenKind::LParen),
            "Expect '(' after 'if'.",
        )?;
        let condition = self.expression()?;
        self.consume(
            TokenKind::Punctuation(PunctuationTokenKind::RParen),
            "Expect ')' after if condition.",
        )?;

        let then_branch = Box::new(self.statement()?);
        let else_branch = if self.match_token(&[TokenKind::Keyword(KeywordTokenKind::Else)]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };

        Ok(Stmt::If(condition, then_branch, else_branch))
    }

    fn while_statement(&mut self) -> Result<Stmt, String> {
        self.consume(
            TokenKind::Punctuation(PunctuationTokenKind::LParen),
            "Expect '(' after 'while'.",
        )?;
        let condition = self.expression()?;
        self.consume(
            TokenKind::Punctuation(PunctuationTokenKind::RParen),
            "Expect ')' after while condition.",
        )?;

        let body = Box::new(self.statement()?);

        Ok(Stmt::While(condition, body))
    }

    fn return_statement(&mut self) -> Result<Stmt, String> {
        let value = if !self.check(TokenKind::Punctuation(PunctuationTokenKind::Semicolon)) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(
            TokenKind::Punctuation(PunctuationTokenKind::Semicolon),
            "Expect ';' after return value.",
        )?;
        Ok(Stmt::Return(value))
    }

    fn block(&mut self) -> Result<Stmt, String> {
        let mut statements = Vec::new();
        while !self.check(TokenKind::Punctuation(PunctuationTokenKind::RBrace)) && !self.is_at_end()
        {
            statements.push(self.declaration()?);
        }
        self.consume(
            TokenKind::Punctuation(PunctuationTokenKind::RBrace),
            "Expect '}' after block.",
        )?;
        Ok(Stmt::Block(statements))
    }

    fn expression_statement(&mut self) -> Result<Stmt, String> {
        let expr = self.expression()?;
        self.consume(
            TokenKind::Punctuation(PunctuationTokenKind::Semicolon),
            "Expect ';' after expression.",
        )?;
        Ok(Stmt::Expr(expr))
    }

    fn expression(&mut self) -> Result<Expr, String> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, String> {
        let expr = self.or()?;

        if self.match_token(&[
            TokenKind::Operator(OperatorTokenKind::Assign),
            TokenKind::Operator(OperatorTokenKind::PlusAssign),
            TokenKind::Operator(OperatorTokenKind::MinusAssign),
            TokenKind::Operator(OperatorTokenKind::StarAssign),
            TokenKind::Operator(OperatorTokenKind::SlashAssign),
        ]) {
            let operator = self.previous().clone();
            let value = self.assignment()?;

            if let Expr::Identifier(ref name) = expr {
                let bin_op = match operator.kind {
                    TokenKind::Operator(OperatorTokenKind::Assign) => BinOp::Equal,
                    TokenKind::Operator(OperatorTokenKind::PlusAssign) => BinOp::Add,
                    TokenKind::Operator(OperatorTokenKind::MinusAssign) => BinOp::Subtract,
                    TokenKind::Operator(OperatorTokenKind::StarAssign) => BinOp::Multiply,
                    TokenKind::Operator(OperatorTokenKind::SlashAssign) => BinOp::Divide,
                    _ => unreachable!(),
                };

                // if equal return just equal else return equal with right being left (op) right
                let value = if bin_op == BinOp::Equal {
                    value
                } else {
                    Expr::Binary(Box::new(Expr::Identifier(name.clone())), bin_op, Box::new(value))
                };
                return Ok(Expr::Assign(Box::new(expr.clone()), Box::new(value)));
            }

            return Err(format!("Invalid assignment target at {:?}", operator));
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr, String> {
        let mut expr = self.and()?;

        while self.match_token(&[TokenKind::Operator(OperatorTokenKind::Or)]) {
            let right = self.and()?;
            expr = Expr::Binary(Box::new(expr), BinOp::Or, Box::new(right));
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, String> {
        let mut expr = self.equality()?;

        while self.match_token(&[TokenKind::Operator(OperatorTokenKind::And)]) {
            let right = self.equality()?;
            expr = Expr::Binary(Box::new(expr), BinOp::And, Box::new(right));
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.comparison()?;

        while self.match_token(&[
            TokenKind::Operator(OperatorTokenKind::Equal),
            TokenKind::Operator(OperatorTokenKind::NotEqual),
        ]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary(
                Box::new(expr),
                match operator.kind {
                    TokenKind::Operator(OperatorTokenKind::Equal) => BinOp::Equal,
                    TokenKind::Operator(OperatorTokenKind::NotEqual) => BinOp::NotEqual,
                    _ => unreachable!(),
                },
                Box::new(right),
            );
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.term()?;

        while self.match_token(&[
            TokenKind::Operator(OperatorTokenKind::Less),
            TokenKind::Operator(OperatorTokenKind::LessEqual),
            TokenKind::Operator(OperatorTokenKind::Greater),
            TokenKind::Operator(OperatorTokenKind::GreaterEqual),
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary(
                Box::new(expr),
                match operator.kind {
                    TokenKind::Operator(OperatorTokenKind::Less) => BinOp::Less,
                    TokenKind::Operator(OperatorTokenKind::LessEqual) => BinOp::LessEqual,
                    TokenKind::Operator(OperatorTokenKind::Greater) => BinOp::Greater,
                    TokenKind::Operator(OperatorTokenKind::GreaterEqual) => BinOp::GreaterEqual,
                    _ => unreachable!(),
                },
                Box::new(right),
            );
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, String> {
        let mut expr = self.factor()?;

        while self.match_token(&[
            TokenKind::Operator(OperatorTokenKind::Plus),
            TokenKind::Operator(OperatorTokenKind::Minus),
        ]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary(
                Box::new(expr),
                match operator.kind {
                    TokenKind::Operator(OperatorTokenKind::Plus) => BinOp::Add,
                    TokenKind::Operator(OperatorTokenKind::Minus) => BinOp::Subtract,
                    _ => unreachable!(),
                },
                Box::new(right),
            );
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.unary()?;

        while self.match_token(&[
            TokenKind::Operator(OperatorTokenKind::Star),
            TokenKind::Operator(OperatorTokenKind::Slash),
            TokenKind::Operator(OperatorTokenKind::Percent),
        ]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary(
                Box::new(expr),
                match operator.kind {
                    TokenKind::Operator(OperatorTokenKind::Star) => BinOp::Multiply,
                    TokenKind::Operator(OperatorTokenKind::Slash) => BinOp::Divide,
                    TokenKind::Operator(OperatorTokenKind::Percent) => BinOp::Modulus,
                    _ => unreachable!(),
                },
                Box::new(right),
            );
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, String> {
        if self.match_token(&[
            TokenKind::Operator(OperatorTokenKind::Not),
            TokenKind::Operator(OperatorTokenKind::Minus),
        ]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary(
                match operator.kind {
                    TokenKind::Operator(OperatorTokenKind::Not) => UnOp::Not,
                    TokenKind::Operator(OperatorTokenKind::Minus) => UnOp::Negate,
                    _ => unreachable!(),
                },
                Box::new(right),
            ));
        }

        self.call()
    }
    fn call(&mut self) -> Result<Expr, String> {
        let mut expr = self.primary()?;

        loop {
            if self.match_token(&[TokenKind::Punctuation(PunctuationTokenKind::LParen)]) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, String> {
        let mut arguments = Vec::new();
        if !self.check(TokenKind::Punctuation(PunctuationTokenKind::RParen)) {
            loop {
                arguments.push(self.expression()?);

                if !self.match_token(&[TokenKind::Punctuation(PunctuationTokenKind::Comma)]) {
                    break;
                }
            }
        }

        self.consume(
            TokenKind::Punctuation(PunctuationTokenKind::RParen),
            "Expect ')' after arguments.",
        )?;

        Ok(Expr::Call(Box::new(callee), arguments))
    }

    fn primary(&mut self) -> Result<Expr, String> {
        if self.match_token(&[TokenKind::Literal(LiteralTokenKind::Number)]) {
            let value: f64 = self
                .previous()
                .value
                .parse()
                .map_err(|e| format!("Failed to parse number: {}", e))?;
            return Ok(Expr::Number(value));
        }

        if self.match_token(&[TokenKind::Literal(LiteralTokenKind::String)]) {
            let value = self.previous().value[1..self.previous().value.len() - 1].to_string();

            return Ok(Expr::String(value));
        }

        if self.match_token(&[TokenKind::Literal(LiteralTokenKind::Identifier)]) {
            return Ok(Expr::Identifier(self.previous().value.clone()));
        }

        if self.match_token(&[TokenKind::Punctuation(PunctuationTokenKind::LParen)]) {
            let expr = self.expression()?;
            self.consume(
                TokenKind::Punctuation(PunctuationTokenKind::RParen),
                "Expect ')' after expression.",
            )?;
            return Ok(expr);
        }

        Err(format!("Unexpected token: {:?}", self.peek()))
    }

    // Helper methods - TODO: Use IteratorExt trait

    fn match_token(&mut self, kinds: &[TokenKind]) -> bool {
        for &kind in kinds {
            if self.check(kind) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, kind: TokenKind) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().kind == kind
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().kind == TokenKind::EOF
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn consume(&mut self, kind: TokenKind, message: &str) -> Result<&Token, String> {
        if self.check(kind) {
            Ok(self.advance())
        } else {
            Err(format!("{} Got {:?}", message, self.peek()))
        }
    }
}
