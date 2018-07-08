extern crate mvtrans;
extern crate tempfile;
#[macro_use] extern crate pretty_assertions;

use mvtrans::parser::Parser;

use std::fs;
use std::fs::File;
use std::io::Read;
use tempfile::tempdir;
use std::io::{self, Write};

use std::path::PathBuf;

#[test]
fn code_101_then_401() {
    let input_dir = tempdir().unwrap().into_path();
    let patch_dir = tempdir().unwrap().into_path();
    
    let file_path = input_dir.join("101_then_401.json");
    
    //Copy over the needed files
    fs::copy("./tests/input_files/101_then_401.json", &file_path).unwrap();

    let mut parser = Parser::new(&input_dir);
    parser.parse();
    parser.write_to_file(&patch_dir);

    //See if the files are the same
    let mut expected_result_file = File::open("./tests/expected_files/101_then_401.txt").unwrap();
    let mut actual_result_file = File::open(&patch_dir.join("101_then_401.txt")).unwrap();

    let mut expected_contents = String::new();
    expected_result_file.read_to_string(&mut expected_contents).unwrap();

    let mut actual_contents = String::new();
    actual_result_file.read_to_string(&mut actual_contents).unwrap();

    assert_eq!(expected_contents, actual_contents);
}

#[test]
fn code_102_then_402() {
    let input_dir = tempdir().unwrap().into_path();
    let patch_dir = tempdir().unwrap().into_path();
    
    let file_path = input_dir.join("102_then_402.json");
    
    //Copy over the needed files
    fs::copy("./tests/input_files/102_then_402.json", &file_path).unwrap();

    let mut parser = Parser::new(&input_dir);
    parser.parse();
    parser.write_to_file(&patch_dir);

    //See if the files are the same
    let mut expected_result_file = File::open("./tests/expected_files/102_then_402.txt").unwrap();
    let mut actual_result_file = File::open(&patch_dir.join("102_then_402.txt")).unwrap();

    let mut expected_contents = String::new();
    expected_result_file.read_to_string(&mut expected_contents).unwrap();

    let mut actual_contents = String::new();
    actual_result_file.read_to_string(&mut actual_contents).unwrap();

    assert_eq!(expected_contents, actual_contents);
}