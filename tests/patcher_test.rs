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
    
    let json_file_path = input_dir.join("Map101_then_401.json");
    let patch_file_path = patch_dir.join("Map101_then_401.txt");
    
    //Copy over the needed files
    fs::copy("./tests/input_files/patcher/Map101_then_401.json", &json_file_path).unwrap();
    fs::copy("./tests/input_files/patcher/Map101_then_401.txt", &patch_file_path).unwrap();

    let mut patcher = Patcher::new(&input_dir, &patch_dir);
    patcher.patch();
    patcher.write_to_file(&output_dir);

    //See if the files are the same
    let mut expected_result_file = File::open("./tests/expected_files/patcher/Map101_then_401.json").unwrap();
    let mut actual_result_file = File::open(&output_dir.join("Map101_then_401.json")).unwrap();

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
    
    let json_file_path = input_dir.join("Mapno_translation_given.json");
    let patch_file_path = patch_dir.join("Mapno_translation_given.txt");
    
    //Copy over the needed files
    fs::copy("./tests/input_files/patcher/Mapno_translation_given.json", &json_file_path).unwrap();
    fs::copy("./tests/input_files/patcher/Mapno_translation_given.txt", &patch_file_path).unwrap();

    let mut patcher = Patcher::new(&input_dir, &patch_dir);
    patcher.patch();
    patcher.write_to_file(&output_dir);

    //See if the files are the same
    let mut expected_result_file = File::open("./tests/expected_files/patcher/Mapno_translation_given.json").unwrap();
    let mut actual_result_file = File::open(&output_dir.join("Mapno_translation_given.json")).unwrap();

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
    
    let json_file_path = input_dir.join("Mapcomments.json");
    let patch_file_path = patch_dir.join("Mapcomments.txt");
    
    //Copy over the needed files
    fs::copy("./tests/input_files/patcher/Mapcomments.json", &json_file_path).unwrap();
    fs::copy("./tests/input_files/patcher/Mapcomments.txt", &patch_file_path).unwrap();

    let mut patcher = Patcher::new(&input_dir, &patch_dir);
    patcher.patch();
    patcher.write_to_file(&output_dir);

    //See if the files are the same
    let mut expected_result_file = File::open("./tests/expected_files/patcher/Mapcomments.json").unwrap();
    let mut actual_result_file = File::open(&output_dir.join("Mapcomments.json")).unwrap();

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
    
    let json_file_path = input_dir.join("Mapspaces_before_translation.json");
    let patch_file_path = patch_dir.join("Mapspaces_before_translation.txt");
    
    //Copy over the needed files
    fs::copy("./tests/input_files/patcher/Mapspaces_before_translation.json", &json_file_path).unwrap();
    fs::copy("./tests/input_files/patcher/Mapspaces_before_translation.txt", &patch_file_path).unwrap();

    let mut patcher = Patcher::new(&input_dir, &patch_dir);
    patcher.patch();
    patcher.write_to_file(&output_dir);

    //See if the files are the same
    let mut expected_result_file = File::open("./tests/expected_files/patcher/Mapspaces_before_translation.json").unwrap();
    let mut actual_result_file = File::open(&output_dir.join("Mapspaces_before_translation.json")).unwrap();

    let mut expected_contents = String::new();
    expected_result_file.read_to_string(&mut expected_contents).unwrap();

    let mut actual_contents = String::new();
    actual_result_file.read_to_string(&mut actual_contents).unwrap();

    assert_eq!(expected_contents, actual_contents);
}

#[test]
fn patcher_multiple_files() {
     let input_dir = tempdir().unwrap().into_path();
    let output_dir = tempdir().unwrap().into_path();
    let patch_dir = tempdir().unwrap().into_path();
    
    let json_file_path1 = input_dir.join("Map101_then_401.json");
    let json_file_path2 = input_dir.join("Mapcomments.json");
    let patch_file_path1 = patch_dir.join("Map101_then_401.txt");
    let patch_file_path2 = patch_dir.join("Mapcomments.txt");
    
    //Copy over the needed files
    fs::copy("./tests/input_files/patcher/Map101_then_401.json", &json_file_path1).unwrap();
    fs::copy("./tests/input_files/patcher/Mapcomments.json", &json_file_path2).unwrap();
    fs::copy("./tests/input_files/patcher/Map101_then_401.txt", &patch_file_path1).unwrap();
    fs::copy("./tests/input_files/patcher/Mapcomments.txt", &patch_file_path2).unwrap();

    let mut patcher = Patcher::new(&input_dir, &patch_dir);
    patcher.patch();
    patcher.write_to_file(&output_dir);

    //See if the files are the same
    let mut expected_result_file1 = File::open("./tests/expected_files/patcher/Map101_then_401.json").unwrap();
    let mut expected_result_file2 = File::open("./tests/expected_files/patcher/Mapcomments.json").unwrap();
    let mut actual_result_file1 = File::open(&output_dir.join("Map101_then_401.json")).unwrap();
    let mut actual_result_file2 = File::open(&output_dir.join("Mapcomments.json")).unwrap();


    let mut expected_contents1 = String::new();
    let mut expected_contents2 = String::new();
    expected_result_file1.read_to_string(&mut expected_contents1).unwrap();
    expected_result_file2.read_to_string(&mut expected_contents2).unwrap();

    let mut actual_contents1 = String::new();
    let mut actual_contents2 = String::new();
    actual_result_file1.read_to_string(&mut actual_contents1).unwrap();
    actual_result_file2.read_to_string(&mut actual_contents2).unwrap();

    assert_eq!(expected_contents1, actual_contents1);
    assert_eq!(expected_contents2, actual_contents2);
}

