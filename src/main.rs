use lexer::Cursor;

mod lexer;

fn main() {
    // let input_path: String = std::env::args().collect::<Vec<_>>()[1].to_owned();
    let input_path = "test.rpt";
    let input_string = std::fs::read_to_string(input_path).unwrap();

    let out = Cursor::tokenize(&input_string).collect::<Vec<_>>();

    println!("{:?}", out)
}
