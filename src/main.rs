use lexer::{Cursor, TokenKind};
use parser::Parser;

mod lexer;
mod parser;
mod codegen;

fn main() {
    // let input_path: String = std::env::args().collect::<Vec<_>>()[1].to_owned();
    let input_path = "test.rpt";
    let input_string = std::fs::read_to_string(input_path).unwrap();

    let tokens = Cursor::tokenize(&input_string).collect::<Vec<_>>();

    let filtered_tokens = tokens.into_iter().filter(|token| match token.kind {
        TokenKind::Comment(_) | TokenKind::Unknown | TokenKind::Whitespace => false,
        _ => true
    }).collect::<Vec<_>>();
    let ast = Parser::parse(filtered_tokens);

    println!("{:#?}", ast)
}
