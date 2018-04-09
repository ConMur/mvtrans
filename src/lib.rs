extern crate serde_json;

use serde_json::Value;

use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::string::String;

pub struct Parser {
    pub json_data : Value,
}   

impl Parser {
    pub fn new(path : &Path) -> Parser {
        let mut file = File::open(path).expect("Invalid file provided");

        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Could not read from file"); 

        let json_data : Value = serde_json::from_str(&contents).expect("Unable to parse JSON data");

        Parser {json_data}
    } 
}

pub fn parse(parser : &Parser) -> Vec<String> {
    let mut untranslated_lines = Vec::new();

    for event in parser.json_data["events"].as_array().unwrap().iter() {
        if event["pages"].is_array() {
        for page in event["pages"].as_array().unwrap().iter() {
            if page["list"].is_array() {
            for list in page["list"].as_array().unwrap().iter() {
                if list["code"] == 401 {
                    if list["parameters"].is_array() {
                    for param in list["parameters"].as_array().unwrap().iter() {
                        untranslated_lines.push(String::from(param.as_str().unwrap()));
                    }
                    }
                }
            }
            }
        }
        }
    }

    untranslated_lines
}