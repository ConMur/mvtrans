extern crate mvtrans;

use std::env;
use std::path::Path;

use mvtrans::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let path = Path::new(filename);

    let parser : Parser = Parser::new(path);

    let lines = mvtrans::parse(&parser);

    mvtrans::write_to_file(&parser, lines);
   
}
