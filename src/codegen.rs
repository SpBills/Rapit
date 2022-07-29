use std::{io::{BufWriter, Write, Error, LineWriter},};

use crate::parser::AST;

#[derive(Debug)]
enum GeneratorError {
    Unknown,
}

pub struct Generator {
    ast: AST,
    buf: LineWriter<std::fs::File>,
}

impl Generator {
    pub fn new(ast: AST, path: &str) -> Self {
        let f = std::fs::File::create(path).unwrap();

        Self {
            ast,
            buf: LineWriter::new(f),
        }
    }

    pub fn run(&mut self) -> Result<(), Error> {
        // TODO: Do something with the GeneratorError
        self.ast
            .program
            .iter()
            .map(|s| writeln!(&mut self.buf, "{}", s))
            .collect::<Result<(), Error>>()

    }
}
