extern crate mvtrans;

use std::env;
use std::path::Path;

use mvtrans::parser::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let path = Path::new(filename);

    let parser : Parser = Parser::new(path);

    let lines = parser.parse();

    mvtrans::parser::write_to_file(&parser, lines);
   
}
