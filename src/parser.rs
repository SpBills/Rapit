use std::{iter::Peekable, slice::Iter};

use crate::{lexer::{KeywordKind, Token, TokenKind}, exprs::{Statement, Expr, Op, ParenExpr, ParenIdent, Literal, Ident, IfStatement, FnStatement, BlockStatement}};

#[derive(Debug)]
pub enum ParseError {
    FalseInner,
    UnexpectedEOF,
    UnexpectedToken,
}

/// The top node of the AST, with the body
/// representing all expressions in the body of the file.
#[derive(Debug)]
pub struct AST {
    pub program: Vec<Statement>,
}

type ParsedStatement<T> = Result<T, ParseError>;

pub struct Parser<'a> {
    iter: &'a mut Peekable<Iter<'a, Token>>,
}

impl Parser<'_> {
    fn peek_iter(&mut self) -> Result<&&Token, ParseError> {
        self.iter.peek().ok_or(ParseError::UnexpectedEOF)
    }

    fn next_iter(&mut self) -> Result<&Token, ParseError> {
        self.iter.next().ok_or(ParseError::UnexpectedEOF)
    }

    fn assert_next(&mut self, next: TokenKind) -> Result<&Token, ParseError> {
        let n = self.next_iter()?;
        if next != n.kind {
            return Err(ParseError::UnexpectedToken);
        }

        Ok(n)
    }

    fn term(&mut self) -> ParsedStatement<Expr> {
        let next = self.peek_iter()?;

        match next.kind {
            TokenKind::Ident(_) => Ok(Expr::Ident(self.ident()?)),
            TokenKind::Literal(_) => Ok(Expr::Literal(self.literal()?)),

            _ => Err(ParseError::UnexpectedToken),
        }
    }

    fn expr(&mut self) -> ParsedStatement<Expr> {
        let term = self.term()?;
        self.expr_1(term, 0)
    }

    /// https://docs.rs/pest/latest/src/pest/prec_climber.rs.html#89-91
    fn expr_1(&mut self, mut lhs: Expr, min_prec: usize) -> ParsedStatement<Expr> {
        while self.peek_iter().is_ok() {
            let lookahead = self.peek_iter()?;
            let op_option = lookahead.inner_operator();

            if let Some(op) = op_option {
                let prec = op.precedence();
                if prec >= min_prec {
                    self.next_iter()?;
                    let mut rhs = self.term()?;

                    while self.peek_iter().is_ok() {
                        let lookahead = self.peek_iter()?;
                        if let Some(lookahead_op) = lookahead.inner_operator() {
                            if lookahead_op.precedence() > prec {
                                rhs = self.expr_1(rhs, prec + 1)?;
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }

                    lhs = Expr::BinOp(Op {
                        op1: Box::new(lhs),
                        operator: op,
                        op2: Box::new(rhs),
                    });
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        Ok(Expr::Term(Box::new(lhs)))
    }

    fn paren_expr(&mut self) -> ParsedStatement<ParenExpr> {
        self.assert_next(TokenKind::OpenParen)?;

        let mut block = vec![];
        while self.peek_iter()?.kind != TokenKind::CloseParen {
            block.push(self.expr()?)
        }

        self.assert_next(TokenKind::CloseParen)?;

        Ok(ParenExpr(block))
    }

    fn paren_ident(&mut self) -> ParsedStatement<ParenIdent> {
        self.assert_next(TokenKind::OpenParen)?;

        let mut block = vec![];
        while self.peek_iter().is_ok() {
            if self.peek_iter()?.kind != TokenKind::CloseParen {
                block.push(self.ident()?);

                // don't require a comma before final paren
                if self.peek_iter()?.kind != TokenKind::CloseParen {
                    self.assert_next(TokenKind::Comma)?;
                }
            } else {
                break;
            }
        }

        self.assert_next(TokenKind::CloseParen)?;

        Ok(ParenIdent(block))
    }

    fn literal(&mut self) -> ParsedStatement<Literal> {
        let lit = self
            .next_iter()?
            .inner_int()
            .ok_or(ParseError::FalseInner)?;

        Ok(lit)
    }

    fn ident(&mut self) -> ParsedStatement<Ident> {
        let id = self
            .next_iter()?
            .inner_string()
            .ok_or(ParseError::FalseInner)?;

        Ok(id)
    }

    fn statement(&mut self) -> ParsedStatement<Statement> {
        let next = self.peek_iter()?;

        match next.kind {
            TokenKind::Keyword(KeywordKind::If) => {
                self.assert_next(TokenKind::Keyword(KeywordKind::If))?;

                let paren_expr = self.paren_expr()?;

                let stmt = self.statement()?;

                Ok(Statement::If(IfStatement {
                    paren: paren_expr,
                    statement: Box::new(stmt),
                }))
            }
            TokenKind::Keyword(KeywordKind::Fn) => {
                self.assert_next(TokenKind::Keyword(KeywordKind::Fn))?;

                let ident = self.ident()?;

                let paren_ident = self.paren_ident()?;

                let stmt = self.statement()?;

                Ok(Statement::Fn(FnStatement {
                    ident: ident,
                    paren_ident: paren_ident,
                    statement: Box::new(stmt),
                }))
            }
            TokenKind::OpenBrace => {
                self.assert_next(TokenKind::OpenBrace)?;

                let mut block = vec![];
                while self.peek_iter()?.kind != TokenKind::CloseBrace {
                    block.push(self.statement()?);
                }

                self.assert_next(TokenKind::CloseBrace)?;
                Ok(Statement::Block(BlockStatement(block)))
            }
            _ => {
                let expr = self.expr()?;

                Ok(Statement::Expr(expr))
            }
        }
    }

    pub fn parse(tokens: Vec<Token>) -> AST {
        let mut iter = tokens.iter().peekable();

        let mut parser = Parser { iter: &mut iter };

        let mut body: Vec<Statement> = vec![];

        while let Ok(_) = parser.peek_iter() {
            let expr = parser.statement().unwrap();
            body.push(expr);
        }

        AST { program: body }
    }
}
