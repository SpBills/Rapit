use crate::parser::{BlockStatement, Expr, FnStatement, IfStatement, Statement, AST};

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

    fn expr_template(&self, expr: &Expr) -> GeneratorResult {
        unimplemented!()
    }

    fn block_template(&self, block: &BlockStatement) -> GeneratorResult {
        unimplemented!()
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
