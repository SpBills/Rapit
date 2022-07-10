use std::{iter::Peekable, slice::Iter};

use crate::lexer::{KeywordKind, Token, TokenKind};

#[derive(Debug)]
enum Operator {
    Add,
    Subtract,
    Divide,
    Multiply,
}

impl Operator {
    fn char_to_op(op: char) -> Option<Self> {
        match op {
            '+' => Some(Operator::Add),
            '-' => Some(Operator::Subtract),
            '/' => Some(Operator::Divide),
            '*' => Some(Operator::Multiply),

            _ => None,
        }
    }
}

#[derive(Debug)]
enum Expr {
    Assignment(AssignmentExpr),
    FunctionDecl(FunctionDeclExpr),
    FunctionCall(FunctionCallExpr),
    BinOp(BinOpExpr),
    Literal(String),
}

/// Represented by IDENT OPERATOR IDENT
#[derive(Debug)]
struct BinOpExpr {
    op: Operator,
    lhs: Box<Expr>,
    rhs: Box<Expr>,
}

/// Represented by KEYWORD IDENT OPEN_PAREN IDENT+ CLOSE_PAREN
#[derive(Debug)]
struct FunctionDeclExpr {
    symbol: String,
    params: Vec<Expr>,
    body: Vec<Expr>,
}

/// Represented by IDENT OPEN_PAREN IDENT+ CLOSE_PAREN
#[derive(Debug)]
struct FunctionCallExpr {
    symbol: String,
    params: Vec<Expr>,
}

/// Represented by IDENT EQUALS IDENT
#[derive(Debug)]
struct AssignmentExpr {
    symbol: String,
    value: Box<Expr>,
}

/// The top node of the AST, with the body
/// representing all expressions in the body of the file.
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
        // Must consume at least one token here to avoid LHS recursion...

        let lhs_expr_parse = match &self.iter.peek().unwrap().kind {
            TokenKind::Ident(ident) => {
                self.iter.next();

                if self.iter.peek().unwrap().kind == TokenKind::Equals {
                    self.iter.next();
                } else {
                    panic!()
                };

                let val = self.expression()?;

                let expr = Expr::Assignment(AssignmentExpr {
                    symbol: ident.clone(),
                    value: Box::new(val),
                });

                Some(expr)
            }
            TokenKind::Literal(lit) => {
                self.iter.next();

                let expr = Expr::Literal(lit.clone());

                Some(expr)
            }
            TokenKind::Keyword(KeywordKind::Fn) => {
                self.iter.next();

                let sym = self.iter.next().unwrap().inner_string().unwrap();

                let expr = Expr::FunctionDecl(FunctionDeclExpr {
                    symbol: sym,
                    params: todo!(),
                    body: todo!(),
                });

                Some(expr)
            }
            _ => None,
        };

        match lhs_expr_parse {
            Some(e) => return Ok(e),
            _ => {}
        }

        let mut lhs = self.expression()?;

        loop {
            let next = self.iter.peek().unwrap();

            match &next.kind {
                TokenKind::Operator(o) => {
                    self.iter.next();

                    let rhs = self.expression()?;

                    lhs = Expr::BinOp(BinOpExpr {
                        op: Operator::char_to_op(*o).unwrap(),
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                    })
                }

                _ => break,
            };
        }

        Ok(lhs)
    }

    pub fn parse(tokens: Vec<Token>) -> AST {
        let mut iter = tokens.iter().peekable();

        let mut parser = Parser { iter: &mut iter };

        let mut body: Vec<Expr> = vec![];

        while let Some(_) = parser.iter.peek() {
            let expr = parser.expression().unwrap();

            body.push(expr);
        }

        AST { body }
    }
}
