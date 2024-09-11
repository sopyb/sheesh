#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Number(f64),
    String(String),
    Identifier(String),
    Binary(Box<Expr>, BinOp, Box<Expr>),
    Unary(UnOp, Box<Expr>),
    Call(Box<Expr>, Vec<Expr>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum BinOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Equal,
    NotEqual,
    And,
    Or,
}

#[derive(Clone, Debug, PartialEq)]
pub enum UnOp {
    Not,
    Negate,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Let(String, Expr),
    Const(String, Expr),
    Expr(Expr),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    While(Expr, Box<Stmt>),
    Function(String, Vec<String>, Box<Stmt>),
    Return(Option<Expr>),
    Block(Vec<Stmt>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum AstNode {
    Expr(Expr),
    Stmt(Stmt),
}