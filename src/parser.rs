use std::{iter::Peekable, slice::Iter};

use crate::lexer::{KeywordKind, Token, TokenKind};

#[derive(Debug)]
enum ParseError {
    FunctionDeclInvalid,
    FalseInner,
    UnexpectedEOF,
    FalseOperator,
    UnexpectedToken
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
enum Expr {
    Assignment(AssignmentExpr),
    FunctionDecl(FunctionDeclExpr),
    IfDecl(IfDeclExpr),
    ArgumentList(Vec<Expr>),
    ParamList(Vec<Expr>),
    FunctionCall(FunctionCallExpr),
    BinOp(BinOpExpr),
    Ident(String),
    Literal(String),
}

#[derive(Debug)]
struct IfDeclExpr {
    /// Reminder that an argument_list is an expr.
    arguments: Box<Expr>,
    body: Vec<Expr>,
}

#[derive(Debug)]
struct BinOpExpr {
    op: Operator,
    lhs: Box<Expr>,
    rhs: Box<Expr>,
}

#[derive(Debug)]
struct FunctionDeclExpr {
    symbol: Box<Expr>,
    params: Box<Expr>,
    body: Vec<Expr>,
}

#[derive(Debug)]
struct FunctionCallExpr {
    symbol: Box<Expr>,
    params: Vec<Expr>,
}

#[derive(Debug)]
struct AssignmentExpr {
    symbol: String,
    value: Box<Expr>,
}

/// The top node of the AST, with the body
/// representing all expressions in the body of the file.
#[derive(Debug)]
pub struct AST {
    program: Vec<Expr>,
}

pub struct Parser<'a> {
    iter: &'a mut Peekable<Iter<'a, Token>>,
}

type ParsedExpr = Result<Expr, ParseError>;

impl Parser<'_> {
    fn peek_iter(&mut self) -> Result<&&Token, ParseError> {
        self.iter.peek().ok_or(ParseError::UnexpectedEOF)
    }

    fn next_iter(&mut self) -> Result<&Token, ParseError> {
        self.iter.next().ok_or(ParseError::UnexpectedEOF)
    }

    fn fn_decl(&mut self) -> ParsedExpr {
        // fn
        self.next_iter()?;

        // function name
        let symbol = self.expression()?;
        // left paren, param list, right paren
        let params = self.param_list()?;

        // open brace
        self.next_iter()?;

        let mut body: Vec<Expr> = vec![];
        loop {
            let next = self.peek_iter()?;

            if next.kind == TokenKind::CloseBrace {
                // close brace
                self.next_iter()?;
                break;
            }

            body.push(self.expression()?);
        }

        Ok(Expr::FunctionDecl(FunctionDeclExpr {
            symbol: Box::new(symbol),
            params: Box::new(params),
            body,
        }))
    }

    /// Consumes open paren, body, and close paren.
    fn argument_list(&mut self) -> ParsedExpr {
        // open paren
        self.next_iter()?;

        let mut args: Vec<Expr> = vec![];
        loop {
            let next = self.peek_iter()?;

            if next.kind == TokenKind::CloseParen {
                // close paren
                self.next_iter()?;
                break;
            }

            args.push(self.expression()?);

            // comma
            self.next_iter()?;
        }

        Ok(Expr::ArgumentList(args))
    }

    fn fn_call(&mut self) -> ParsedExpr {
        todo!()
    }

    fn if_decl(&mut self) -> ParsedExpr {
        // if
        self.next_iter()?;

        let args = self.argument_list()?;

        // open brace
        self.next_iter()?;

        let mut body: Vec<Expr> = vec![];
        loop {
            let next = self.peek_iter()?;

            if next.kind == TokenKind::CloseBrace {
                // close brace
                self.next_iter()?;
                break;
            }

            body.push(self.expression()?);
        }

        Ok(Expr::IfDecl(IfDeclExpr {
            arguments: Box::new(args),
            body,
        }))
    }

    fn bin_op(&mut self) -> ParsedExpr {
        let lhs = self.literal()?;
        let op = Token { kind: TokenKind::Operator('+'), len: 1};
        let rhs = self.expression()?;

        Ok(Expr::BinOp(BinOpExpr {
            op: Operator::char_to_op(op.inner_operator().ok_or(ParseError::FalseInner)?)
                .ok_or(ParseError::FalseOperator)?,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }))
    }

    fn assignment(&mut self, id: &Token) -> ParsedExpr {
        let eq = self.next_iter()?;

        let expr = self.expression()?;

        Ok(Expr::Assignment(AssignmentExpr {
            symbol: id.inner_string().ok_or(ParseError::FalseInner)?,
            value: Box::new(expr)
        }))
    }

    fn ident(&mut self) -> ParsedExpr {
        let id = self.next_iter()?;
        let next = self.peek_iter()?;

        if next.kind == TokenKind::Equals {
            return self.assignment(id);
        }

        Ok(Expr::Ident(
            id.inner_string().ok_or(ParseError::FalseInner)?,
        ))
    }

    fn param_list(&mut self) -> ParsedExpr {
        // left paren
        self.next_iter()?;

        let mut params: Vec<Expr> = vec![];
        loop {
            let next = self.peek_iter()?;

            if next.kind == TokenKind::CloseParen {
                self.next_iter()?;
                break;
            }

            params.push(self.expression()?)
        }

        Ok(Expr::ParamList(params))
    }

    fn literal(&mut self) -> ParsedExpr {
        let symbol = self.next_iter()?;

        Ok(Expr::Literal(symbol.inner_string().ok_or(ParseError::FalseInner)?))
    }

    fn expression(&mut self) -> ParsedExpr {
        let next = self.peek_iter()?;

        let expr = match &next.kind {
            TokenKind::Operator(_) => self.bin_op(),
            TokenKind::Literal(_) => self.literal(),
            TokenKind::Keyword(KeywordKind::Fn) => self.fn_call(),
            TokenKind::Keyword(KeywordKind::If) => self.if_decl(),
            TokenKind::Ident(_) => self.ident(),
            _ => Err(ParseError::UnexpectedToken)
        }?;

        Ok(expr)
    }

    pub fn parse(tokens: Vec<Token>) -> AST {
        let mut iter = tokens.iter().peekable();

        let mut parser = Parser { iter: &mut iter };

        let mut body: Vec<Expr> = vec![];

        while let Ok(_) = parser.peek_iter() {
            let expr = parser.expression().unwrap();
            println!("{:?}", expr);

            body.push(expr);
        }

        AST { program: body }
    }
}
