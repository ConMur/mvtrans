extern crate mvtrans;

use std::env;
use std::path::Path;

use mvtrans::parser::Parser;
use mvtrans::patcher::Patcher;

fn main() {
    //TODO: CLI for parse and patch
    let args: Vec<String> = env::args().collect();

    let source_file = format!("{}.json", &args[1]);
    let patch_file = format!("{}.txt", &args[1]);
/*
    let parser : Parser = Parser::new(&source_file);

    let lines = parser.parse();

    mvtrans::parser::write_to_file(&parser, lines);*/
    

    let mut patcher : Patcher = Patcher::new(source_file, patch_file);

    patcher.patch();

    patcher.write_to_file("test.json".to_string());
   
}
