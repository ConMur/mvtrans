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

    for line in lines.iter() {
        println!("{}", line.context);
        println!("{}", line.speaker);
        for d in line.data.iter() {
            println!("{}", d);
        }
    }
}
