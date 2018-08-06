extern crate mvtrans;
extern crate tempfile;
#[macro_use] extern crate pretty_assertions;

use mvtrans::parser::Parser;

use std::fs;
use std::fs::File;
use std::io::Read;
use tempfile::tempdir;

#[test]
fn parser_code_101_then_401() {
    let input_dir = tempdir().unwrap().into_path();
    let patch_dir = tempdir().unwrap().into_path();
    
    let file_path = input_dir.join("Map101_then_401.json");
    
    //Copy over the needed files
    fs::copy("./tests/input_files/parser/Map101_then_401.json", &file_path).unwrap();

    let mut parser = Parser::new(&input_dir);
    parser.parse();
    parser.write_to_file(&patch_dir);

    //See if the files are the same
    let mut expected_result_file = File::open("./tests/expected_files/parser/Map101_then_401.txt").unwrap();
    let mut actual_result_file = File::open(&patch_dir.join("Map101_then_401.txt")).unwrap();

    let mut expected_contents = String::new();
    expected_result_file.read_to_string(&mut expected_contents).unwrap();

    let mut actual_contents = String::new();
    actual_result_file.read_to_string(&mut actual_contents).unwrap();

    assert_eq!(expected_contents, actual_contents);
}

#[test]
fn parser_code_102_then_402() {
    let input_dir = tempdir().unwrap().into_path();
    let patch_dir = tempdir().unwrap().into_path();
    
    let file_path = input_dir.join("Map102_then_402.json");
    
    //Copy over the needed files
    fs::copy("./tests/input_files/parser/Map102_then_402.json", &file_path).unwrap();

    let mut parser = Parser::new(&input_dir);
    parser.parse();
    parser.write_to_file(&patch_dir);

    //See if the files are the same
    let mut expected_result_file = File::open("./tests/expected_files/parser/Map102_then_402.txt").unwrap();
    let mut actual_result_file = File::open(&patch_dir.join("Map102_then_402.txt")).unwrap();

    let mut expected_contents = String::new();
    expected_result_file.read_to_string(&mut expected_contents).unwrap();

    let mut actual_contents = String::new();
    actual_result_file.read_to_string(&mut actual_contents).unwrap();

    assert_eq!(expected_contents, actual_contents);
}

#[test]
fn parser_escaped_quotes() {
    let input_dir = tempdir().unwrap().into_path();
    let patch_dir = tempdir().unwrap().into_path();
    
    let file_path = input_dir.join("Mapescaped_quotes.json");
    
    //Copy over the needed files
    fs::copy("./tests/input_files/parser/Mapescaped_quotes.json", &file_path).unwrap();

    let mut parser = Parser::new(&input_dir);
    parser.parse();
    parser.write_to_file(&patch_dir);

    //See if the files are the same
    let mut expected_result_file = File::open("./tests/expected_files/parser/Mapescaped_quotes.txt").unwrap();
    let mut actual_result_file = File::open(&patch_dir.join("Mapescaped_quotes.txt")).unwrap();

    let mut expected_contents = String::new();
    expected_result_file.read_to_string(&mut expected_contents).unwrap();

    let mut actual_contents = String::new();
    actual_result_file.read_to_string(&mut actual_contents).unwrap();

    assert_eq!(expected_contents, actual_contents);
}

#[test]
fn parser_multiple_files() {
    let input_dir = tempdir().unwrap().into_path();
    let patch_dir = tempdir().unwrap().into_path();
    
    let file_path1 = input_dir.join("Map101_then_401.json");
    let file_path2 = input_dir.join("Map102_then_402.json");
    
    //Copy over the needed files
    fs::copy("./tests/input_files/parser/Map101_then_401.json", &file_path1).unwrap();
    fs::copy("./tests/input_files/parser/Map102_then_402.json", &file_path2).unwrap();

    let mut parser = Parser::new(&input_dir);
    parser.parse();
    parser.write_to_file(&patch_dir);

    //See if the files are the same
    let mut expected_result_file1 = File::open("./tests/expected_files/parser/Map101_then_401.txt").unwrap();
    let mut expected_result_file2 = File::open("./tests/expected_files/parser/Map102_then_402.txt").unwrap();
    let mut actual_result_file1 = File::open(&patch_dir.join("Map101_then_401.txt")).unwrap();
    let mut actual_result_file2 = File::open(&patch_dir.join("Map102_then_402.txt")).unwrap();

    let mut expected_contents1 = String::new();
    expected_result_file1.read_to_string(&mut expected_contents1).unwrap();
    let mut expected_contents2 = String::new();
    expected_result_file2.read_to_string(&mut expected_contents2).unwrap();

    let mut actual_contents1 = String::new();
    actual_result_file1.read_to_string(&mut actual_contents1).unwrap();
    let mut actual_contents2 = String::new();
    actual_result_file2.read_to_string(&mut actual_contents2).unwrap();

    assert_eq!(expected_contents1, actual_contents1);
    assert_eq!(expected_contents2, actual_contents2);
}