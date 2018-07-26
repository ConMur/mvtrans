extern crate mvtrans;
extern crate tempfile;
#[macro_use] extern crate pretty_assertions;

use mvtrans::patcher::Patcher;

use std::fs;
use std::fs::File;
use std::io::Read;
use tempfile::tempdir;

#[test]
fn patcher_code_101_then_401() {
    let input_dir = tempdir().unwrap().into_path();
    let output_dir = tempdir().unwrap().into_path();
    let patch_dir = tempdir().unwrap().into_path();
    
    let json_file_path = input_dir.join("101_then_401.json");
    let patch_file_path = patch_dir.join("101_then_401.txt");
    
    //Copy over the needed files
    fs::copy("./tests/input_files/patcher/101_then_401.json", &json_file_path).unwrap();
    fs::copy("./tests/input_files/patcher/101_then_401.txt", &patch_file_path).unwrap();

    let mut patcher = Patcher::new(&input_dir, &patch_dir);
    patcher.patch();
    patcher.write_to_file(&output_dir);

    //See if the files are the same
    let mut expected_result_file = File::open("./tests/expected_files/patcher/101_then_401.json").unwrap();
    let mut actual_result_file = File::open(&output_dir.join("101_then_401.json")).unwrap();

    let mut expected_contents = String::new();
    expected_result_file.read_to_string(&mut expected_contents).unwrap();

    let mut actual_contents = String::new();
    actual_result_file.read_to_string(&mut actual_contents).unwrap();

    assert_eq!(expected_contents, actual_contents);
}

#[test]
fn patcher_no_translation_given() {
    let input_dir = tempdir().unwrap().into_path();
    let output_dir = tempdir().unwrap().into_path();
    let patch_dir = tempdir().unwrap().into_path();
    
    let json_file_path = input_dir.join("no_translation_given.json");
    let patch_file_path = patch_dir.join("no_translation_given.txt");
    
    //Copy over the needed files
    fs::copy("./tests/input_files/patcher/no_translation_given.json", &json_file_path).unwrap();
    fs::copy("./tests/input_files/patcher/no_translation_given.txt", &patch_file_path).unwrap();

    let mut patcher = Patcher::new(&input_dir, &patch_dir);
    patcher.patch();
    patcher.write_to_file(&output_dir);

    //See if the files are the same
    let mut expected_result_file = File::open("./tests/expected_files/patcher/no_translation_given.json").unwrap();
    let mut actual_result_file = File::open(&output_dir.join("no_translation_given.json")).unwrap();

    let mut expected_contents = String::new();
    expected_result_file.read_to_string(&mut expected_contents).unwrap();

    let mut actual_contents = String::new();
    actual_result_file.read_to_string(&mut actual_contents).unwrap();

    assert_eq!(expected_contents, actual_contents);
}

#[test]
fn patcher_comments() {
    let input_dir = tempdir().unwrap().into_path();
    let output_dir = tempdir().unwrap().into_path();
    let patch_dir = tempdir().unwrap().into_path();
    
    let json_file_path = input_dir.join("comments.json");
    let patch_file_path = patch_dir.join("comments.txt");
    
    //Copy over the needed files
    fs::copy("./tests/input_files/patcher/comments.json", &json_file_path).unwrap();
    fs::copy("./tests/input_files/patcher/comments.txt", &patch_file_path).unwrap();

    let mut patcher = Patcher::new(&input_dir, &patch_dir);
    patcher.patch();
    patcher.write_to_file(&output_dir);

    //See if the files are the same
    let mut expected_result_file = File::open("./tests/expected_files/patcher/comments.json").unwrap();
    let mut actual_result_file = File::open(&output_dir.join("comments.json")).unwrap();

    let mut expected_contents = String::new();
    expected_result_file.read_to_string(&mut expected_contents).unwrap();

    let mut actual_contents = String::new();
    actual_result_file.read_to_string(&mut actual_contents).unwrap();

    assert_eq!(expected_contents, actual_contents);
}

#[test]
fn patcher_spaces_before_translation() {
     let input_dir = tempdir().unwrap().into_path();
    let output_dir = tempdir().unwrap().into_path();
    let patch_dir = tempdir().unwrap().into_path();
    
    let json_file_path = input_dir.join("spaces_before_translation.json");
    let patch_file_path = patch_dir.join("spaces_before_translation.txt");
    
    //Copy over the needed files
    fs::copy("./tests/input_files/patcher/spaces_before_translation.json", &json_file_path).unwrap();
    fs::copy("./tests/input_files/patcher/spaces_before_translation.txt", &patch_file_path).unwrap();

    let mut patcher = Patcher::new(&input_dir, &patch_dir);
    patcher.patch();
    patcher.write_to_file(&output_dir);

    //See if the files are the same
    let mut expected_result_file = File::open("./tests/expected_files/patcher/spaces_before_translation.json").unwrap();
    let mut actual_result_file = File::open(&output_dir.join("spaces_before_translation.json")).unwrap();

    let mut expected_contents = String::new();
    expected_result_file.read_to_string(&mut expected_contents).unwrap();

    let mut actual_contents = String::new();
    actual_result_file.read_to_string(&mut actual_contents).unwrap();

    assert_eq!(expected_contents, actual_contents);
}

