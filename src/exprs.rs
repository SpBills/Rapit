/// Contains all parser tokens and structs.
/// Also implements the parser Display trait for codegen.
/// 
/// TODO: Refactor pub fields to private by impl ::new operator.
use std::fmt::Display;

use crate::lexer::Operator;

#[derive(Debug)]
pub enum Statement {
    If(IfStatement),
    Fn(FnStatement),
    Block(BlockStatement),
    Expr(Expr),
    Assignment(AssignmentStatement)
}

#[derive(Debug)]
pub struct FnStatement {
    pub ident: Ident,
    pub paren_ident: ParenIdent,
    pub statement: Box<Statement>,
}

#[derive(Debug)]
pub struct ParenExpr(pub Vec<Expr>);
#[derive(Debug)]
pub struct ParenIdent(pub Vec<Ident>);
#[derive(Debug)]
pub struct BlockStatement(pub Vec<Statement>);

#[derive(Debug)]
pub struct IfStatement {
    pub paren: ParenExpr,
    pub statement: Box<Statement>,
}

#[derive(Debug)]
pub enum Expr {
    Ident(Ident),
    Literal(Literal),
    ParenExpr(ParenExpr),
    Term(Box<Expr>),
    BinOp(Op),
}

#[derive(Debug)]
pub struct AssignmentStatement {
    pub ident: Ident,
    pub val: Box<Expr>,
}

#[derive(Debug)]
pub struct Op {
    pub op1: Box<Expr>,
    pub op2: Box<Expr>,
    pub operator: Operator,
}

pub type Ident = String;
pub type Literal = usize;

impl Display for BlockStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{\n")?;
        self.0
            .iter()
            .fold(Ok(()), |res, stmt| res.and_then(|_| write!(f, "{}", stmt)))?;
        write!(f, "}}")
    }
}

impl Display for ParenIdent {
    /// This likely does not need to be this complex
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;
        self.0
            .iter()
            .fold(Ok(()), |res, s| res.and_then(|_| write!(f, "{}", s)))?;
        write!(f, ")")
    }
}

impl Display for FnStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "void {}{}\n{}",
            self.ident, self.paren_ident, self.statement
        )
    }
}


impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::If(i) => write!(f, "{}", i),
            Self::Expr(i) => writeln!(f, "{};", i),
            Self::Fn(i) => write!(f, "{}", i),
            Self::Block(i) => write!(f, "{}", i),
            Self::Assignment(a) => write!(f, "{}", a)
        }
    }
}

impl Display for IfStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "if {}\n{}", self.paren, self.statement)
    }
}

impl Display for ParenExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;
        self.0
            .iter()
            .fold(Ok(()), |res, expr| res.and_then(|_| write!(f, "{}", expr)))?;
        write!(f, ")")
    }
}

impl Display for AssignmentStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {}", self.ident, self.val)
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ident(i) => write!(f, "{}", i.to_owned()),
            Self::Literal(i) => write!(f, "{}", i),
            Self::ParenExpr(i) => write!(f, "{}", i),
            Self::Term(i) => write!(f, "{}", i),
            Self::BinOp(i) => write!(f, "{}", i),
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.op_to_char().unwrap())
    }
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.op1, self.operator, self.op2)
    }
}
