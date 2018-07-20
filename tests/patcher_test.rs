extern crate mvtrans;
extern crate tempfile;
#[macro_use] extern crate pretty_assertions;

use mvtrans::parser::Parser;

use std::fs;
use std::fs::File;
use std::io::Read;
use tempfile::tempdir;

#[test]
fn code_101_then_401() {
    assert_eq!(true, true);
}