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
    Expr(Expr)
}


#[derive(Debug)]
struct FnStatement {
    ident: Ident,
    paren_ident: ParenIdent,
    statement: Box<Statement>
}


type ParenIdent = Vec<Ident>;
type BlockStatement = Vec<Statement>;

#[derive(Debug)]
struct IfStatement {
    paren: ParenExpr,
    statement: Box<Statement>
}

#[derive(Debug)]
struct ParenExpr {
    expr: Expr
}

#[derive(Debug)]
enum Expr {
    Test(Test),
    Assignment(AssignmentExpr)
}

#[derive(Debug)]
struct AssignmentExpr {
    ident: Ident,
    val: Box<Expr>
}

type Ident = String;
type Int = usize;

#[derive(Debug)]
enum Test {
    Unary(Sum),
    LT(Op)
}

#[derive(Debug)]
struct Op {
    op1: Box<Sum>,
    op2: Box<Sum>,
    operator: Operator
}

#[derive(Debug)]
enum Sum {
    Term(Term),
    AddOp(Op),
    SubOp(Op)
}

#[derive(Debug)]
enum Term {
    Ident(Ident),
    Int(Int),
    ParenExpr(Box<ParenExpr>)
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
            self.next_iter();
            return Err(ParseError::UnexpectedToken);
        }

        Ok(())
    }

    fn term(&mut self) -> ParsedStatement<Term> {
        unimplemented!()
    }

    fn sum(&mut self) -> ParsedStatement<Sum> {
        unimplemented!()
    }

    fn test(&mut self) -> ParsedStatement<Test> {
        unimplemented!()
    }

    fn expr(&mut self) -> ParsedStatement<Expr> {
        unimplemented!()
    }

    fn paren_expr(&mut self) -> ParsedStatement<ParenExpr> {
        unimplemented!()
    }

    fn paren_ident(&mut self) -> ParsedStatement<ParenIdent> {
        unimplemented!()
    }

    fn ident(&mut self) -> ParsedStatement<Ident> {
        unimplemented!()
    }

    fn statement(&mut self) -> ParsedStatement<Statement> {
        unimplemented!()
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
