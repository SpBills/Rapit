use std::{iter::Peekable, slice::Iter};

use crate::lexer::{KeywordKind, Token, TokenKind};

#[derive(Debug)]
enum ParseError {
    FalseInner,
    UnexpectedEOF,
    UnexpectedToken,
    ActualAssignment(Ident),
    ActualTest,
    ActualSum,
}

#[derive(Debug)]
enum Operator {
    Add,
    Subtract,
    Divide,
    Multiply,
    LessThan,
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
type Literal = usize;

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
    Literal(Literal),
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

    fn assert_not_next(&mut self, next: TokenKind, err: ParseError) -> Result<(), ParseError> {
        if next == self.peek_iter()?.kind {
            self.next_iter()?;
            return Err(err);
        }

        Ok(())
    }

    fn assert_next(&mut self, next: TokenKind) -> Result<&Token, ParseError> {
        let n = self.next_iter()?;
        if next != n.kind {
            return Err(ParseError::UnexpectedToken);
        }

        Ok(n)
    }

    fn term(&mut self) -> ParsedStatement<Term> {
        let next = self.peek_iter()?;

        println!("{:?}", next);

        match next.kind {
            TokenKind::Ident(_) => Ok(Term::Ident(self.ident()?)),
            TokenKind::Literal(_) => Ok(Term::Literal(self.literal()?)),

            _ => Err(ParseError::UnexpectedToken),
        }
    }

    fn sum(&mut self) -> ParsedStatement<Sum> {
        let t = self.term();

        match t {
            Err(ParseError::ActualSum) => {
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
            _ => Ok(Sum::Term(t?)),
        }
    }

    fn test(&mut self) -> ParsedStatement<Test> {
        let s = self.sum();

        match s {
            Err(ParseError::ActualTest) => {
                let op1 = Box::new(self.sum()?);
                self.assert_next(TokenKind::Operator('<'))?;
                let op2 = Box::new(self.sum()?);

                Ok(Test::LT(Op {
                    op1,
                    op2,
                    operator: Operator::LessThan,
                }))
            }
            _ => Ok(Test::Unary(s?)),
        }
    }

    fn expr(&mut self) -> ParsedStatement<Expr> {
        let t = self.test();

        match t {
            Err(ParseError::ActualAssignment(x)) => {
                let val = Box::new(self.expr()?);

                Ok(Expr::Assignment(AssignmentExpr { ident: x, val }))
            }
            _ => Ok(Expr::Test(t?)),
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

    fn literal(&mut self) -> ParsedStatement<Literal> {
        let lit = self
            .next_iter()?
            .inner_int()
            .ok_or(ParseError::FalseInner)?;

        self.assert_not_next(TokenKind::Operator('<'), ParseError::ActualTest)?;
        self.assert_not_next(TokenKind::Operator('+'), ParseError::ActualSum)?;
        self.assert_not_next(TokenKind::Operator('-'), ParseError::ActualSum)?;

        Ok(lit)
    }

    fn ident(&mut self) -> ParsedStatement<Ident> {
        let id = self
            .next_iter()?
            .inner_string()
            .ok_or(ParseError::FalseInner)?;

        self.assert_not_next(TokenKind::Equals, ParseError::ActualAssignment(id.clone()))?;
        self.assert_not_next(TokenKind::Operator('<'), ParseError::ActualTest)?;
        self.assert_not_next(TokenKind::Operator('+'), ParseError::ActualSum)?;
        self.assert_not_next(TokenKind::Operator('-'), ParseError::ActualSum)?;

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
            body.push(expr);
        }

        AST { program: body }
    }
}
