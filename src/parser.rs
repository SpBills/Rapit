use crate::lexer::{KeywordKind, Token, TokenKind};

#[derive(Debug)]
enum Operator {
    Add,
    Subtract,
    Divide,
    Multiply,
}

#[derive(Debug)]
enum Expr {
    Assignment(Box<AssignmentExpr>),
    Function(Box<FunctionExpr>),
    BinOp(Box<BinOpExpr>),
    Literal(String),
}

#[derive(Debug)]
struct BinOpExpr {
    op: Operator,
    lhs: Expr,
    rhs: Expr,
}

#[derive(Debug)]
struct FunctionExpr {
    symbol: String,
    params: Vec<Expr>,
    body: Vec<Expr>,
}

#[derive(Debug)]
struct AssignmentExpr {
    symbol: String,
    value: Expr,
}

#[derive(Debug)]
pub struct AST {
    body: Vec<Expr>,
}

impl AST {
    fn new(body: Vec<Expr>) -> Self {
        Self { body }
    }
}

pub struct Parser;

impl Parser {
    pub fn parse(tokens: Vec<Token>) -> AST {
        let mut iter = tokens.iter().peekable();

        let mut exprs: Vec<Expr> = vec![];

        loop {
            let next = iter.peek().unwrap();

            let expr = match &next.kind {
                TokenKind::Literal(x) => Expr::Literal(x.clone()),

                _ => break
            };

            exprs.push(expr);
        }

        AST::new(exprs);

        todo!()
    }
}
