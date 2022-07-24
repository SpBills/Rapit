use std::{iter::Peekable, slice::Iter};

use crate::lexer::{KeywordKind, Token, TokenKind};

#[derive(Debug)]
enum ParseError {
    FunctionDeclInvalid,
    FalseInner,
    UnexpectedEOF,
    FalseOperator,
    UnexpectedToken,
}

#[derive(Debug)]
enum Operator {
    Add,
    Subtract,
    Divide,
    Multiply,
    LessThan
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
enum Statement {
    If(IfStatement),
    Fn(FnStatement),
    Block(BlockStatement),
    Expr(Expr),
}

#[derive(Debug)]
struct FnStatement {
    ident: Ident,
    paren_ident: ParenIdent,
    statement: Box<Statement>,
}

type ParenExpr = Vec<Expr>;
type ParenIdent = Vec<Ident>;
type BlockStatement = Vec<Statement>;

#[derive(Debug)]
struct IfStatement {
    paren: ParenExpr,
    statement: Box<Statement>,
}

#[derive(Debug)]
enum Expr {
    Test(Test),
    Assignment(AssignmentExpr),
}

#[derive(Debug)]
struct AssignmentExpr {
    ident: Ident,
    val: Box<Expr>,
}

type Ident = String;
type Int = usize;

#[derive(Debug)]
enum Test {
    Unary(Sum),
    LT(Op),
}

#[derive(Debug)]
struct Op {
    op1: Box<Sum>,
    op2: Box<Sum>,
    operator: Operator,
}

#[derive(Debug)]
enum Sum {
    Term(Term),
    AddOp(Op),
    SubOp(Op),
}

#[derive(Debug)]
enum Term {
    Ident(Ident),
    Int(Int),
    ParenExpr(ParenExpr),
}

/// The top node of the AST, with the body
/// representing all expressions in the body of the file.
#[derive(Debug)]
pub struct AST {
    program: Vec<Statement>,
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

    fn assert_next(&mut self, next: TokenKind) -> Result<(), ParseError> {
        if next != self.peek_iter()?.kind {
            self.next_iter()?;
            return Err(ParseError::UnexpectedToken);
        }

        Ok(())
    }

    fn term(&mut self) -> ParsedStatement<Term> {
        let next = self.peek_iter()?;

        match next.kind {
            TokenKind::Ident(_) => Ok(Term::Ident(self.ident()?)),

            _ => Err(ParseError::UnexpectedToken),
        }
    }

    fn sum(&mut self) -> ParsedStatement<Sum> {
        if let Ok(x) = self.term() {
            Ok(Sum::Term(x))
        } else {
            let op1 = Box::new(self.sum()?);
            let op_string = self
                .next_iter()?
                .inner_string()
                .ok_or(ParseError::FalseInner)?;
            let op = Operator::char_to_op(op_string.chars().next().unwrap())
                .ok_or(ParseError::UnexpectedToken)?;
            let op2 = Box::new(self.sum()?);

            Ok(Sum::AddOp(Op {
                op1,
                op2,
                operator: op,
            }))
        }
    }

    fn test(&mut self) -> ParsedStatement<Test> {
        if let Ok(x) = self.sum() {
            Ok(Test::Unary(x))
        } else {
            let op1 = Box::new(self.sum()?);
            self.assert_next(TokenKind::Operator('<'))?;
            let op2 = Box::new(self.sum()?);

            Ok(Test::LT(Op {
                op1,
                op2,
                operator: Operator::LessThan
            }))
        }
    }

    fn expr(&mut self) -> ParsedStatement<Expr> {
        if let Ok(x) = self.test() {
            Ok(Expr::Test(x))
        } else {
            let ident = self.ident()?;
            self.assert_next(TokenKind::Equals)?;
            let val = Box::new(self.expr()?);

            Ok(Expr::Assignment(AssignmentExpr {
                ident,
                val
            }))
        }
    }

    fn paren_expr(&mut self) -> ParsedStatement<ParenExpr> {
        self.assert_next(TokenKind::OpenParen)?;

        let mut block = vec![];
        while self.peek_iter()?.kind != TokenKind::CloseParen {
            block.push(self.expr()?)
        }

        self.assert_next(TokenKind::CloseParen)?;

        Ok(block)
    }

    fn paren_ident(&mut self) -> ParsedStatement<ParenIdent> {
        self.assert_next(TokenKind::OpenParen)?;

        let mut block = vec![];
        while self.peek_iter()?.kind != TokenKind::CloseParen {
            block.push(self.ident()?)
        }

        self.assert_next(TokenKind::CloseParen)?;

        Ok(block)
    }

    fn int(&mut self) -> ParsedStatement<Int> {
        self.next_iter()?.inner_int().ok_or(ParseError::FalseInner)
    }

    fn ident(&mut self) -> ParsedStatement<Ident> {
        self.next_iter()?
            .inner_string()
            .ok_or(ParseError::FalseInner)
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
                Ok(Statement::Block(block))
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
            println!("{:?}", expr);

            body.push(expr);
        }

        AST { program: body }
    }
}
