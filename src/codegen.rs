use crate::parser::{BlockStatement, Expr, FnStatement, IfStatement, Statement, AST, AssignmentExpr, Op, ParenExpr};

enum GeneratorError {
    Unknown,
}

struct Generator {
    ast: AST,
}

type GeneratorResult = Result<String, GeneratorError>;

impl Generator {
    fn new(ast: AST) -> Self {
        Self { ast }
    }

    fn if_template(&self, fi: &IfStatement) -> GeneratorResult {
        unimplemented!()
    }

    fn fn_template(&self, func: &FnStatement) -> GeneratorResult {
        unimplemented!()
    }

    fn assignment_template(&self, assn: &AssignmentExpr) -> GeneratorResult {
        unimplemented!()
    }

    fn binop_template(&self, binop: &Op) -> GeneratorResult {
        unimplemented!()
    }

    fn expr_template(&self, expr: &Expr) -> GeneratorResult {
        match expr {
            Expr::Assignment(a) => self.assignment_template(a),
            Expr::BinOp(b) => self.binop_template(b),
            Expr::Ident(i) => Ok(i.to_owned()),
            Expr::Literal(l) => Ok(l.to_string()),
            Expr::ParenExpr(p) => p.iter().map(|e| self.expr_template(e)).collect(),
            Expr::Term(t) => self.expr_template(t)
        }
    }

    fn block_template(&self, block: &BlockStatement) -> GeneratorResult {
        block.iter().map(|s| self.statement_template(s)).collect::<GeneratorResult>()
    }

    fn statement_template(&self, stmt: &Statement) -> GeneratorResult {
        match stmt {
            Statement::Block(b) => self.block_template(b),
            Statement::Expr(e) => self.expr_template(e),
            Statement::Fn(f) => self.fn_template(f),
            Statement::If(i) => self.if_template(i),
        }
    }

    fn run(&self) {
        self.ast
            .program
            .iter()
            .map(|statement| self.statement_template(statement));
    }
}
