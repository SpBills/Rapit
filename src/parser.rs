use std::{iter::Peekable, slice::Iter, ops::Sub};

use crate::lexer::{KeywordKind, Token, TokenKind};

#[derive(Debug)]
enum Operator {
    Add,
    Subtract,
    Divide,
    Multiply,
}

impl Operator {
    fn char_to_op(op: char) -> Self {
        match op {
            '+' => Operator::Add,
            '-' => Operator::Subtract,
            '/' => Operator::Divide,
            '*' => Operator::Multiply
        }
    }
}

#[derive(Debug)]
enum Expr {
    Assignment(AssignmentExpr),
    Function(FunctionExpr),
    BinOp(BinOpExpr),
    Literal(String),
}

#[derive(Debug)]
struct BinOpExpr {
    op: Operator,
    lhs: Box<Expr>,
    rhs: Box<Expr>,
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
    value: Box<Expr>,
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

pub struct Parser<'a> {
    iter: &'a mut Peekable<Iter<'a, Token>>,
}

impl Parser<'_> {
    fn expression(&mut self) -> Result<Expr, ()> {
        let mut expr = self.expression()?;

        loop {
            let next = self.iter.peek().unwrap();

            match &next.kind {
                TokenKind::Literal(lit) => {
                    self.iter.next();

                    expr = Expr::Literal(lit.clone())
                }
                TokenKind::Operator(o) => {
                    self.iter.next();

                    let rhs = self.expression()?;

                    expr = Expr::BinOp(BinOpExpr {
                        op: Operator::char_to_op(*o),
                        lhs: Box::new(expr),
                        rhs: Box::new(rhs),
                    })
                }

                _ => break,
            };
        }

        Ok(expr)
    }

    pub fn parse(tokens: Vec<Token>) -> AST {
        let mut iter = tokens.iter().peekable();

        let mut parser = Parser {
            iter: &mut iter
        };

        parser.expression();

        todo!()
    }
}
