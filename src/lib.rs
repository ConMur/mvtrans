extern crate serde_json;

use serde_json::Value;

use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::string::String;

pub struct Parser {
    pub file_name : String,
    pub json_data : Value,
}   

pub struct UntransLine {
    pub context : String,
    pub speaker: String,
    pub data : Vec<String>,
}

impl Parser {
    pub fn new(path : &Path) -> Parser {
        let mut file = File::open(path).expect("Invalid file provided");

        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Could not read from file"); 

        let json_data : Value = serde_json::from_str(&contents).expect("Unable to parse JSON data");
        let file_name = String::from(path.file_name().expect("Unable to determine file name").to_str().unwrap());

        Parser {file_name, json_data}
    } 
}

pub fn parse(parser : &Parser) -> Vec<UntransLine> {
    let mut untranslated_lines = Vec::new();
    let mut line = UntransLine{context: "".to_string(), speaker: "".to_string(), data: Vec::new()};

    for (event_num, event) in parser.json_data["events"].as_array().unwrap().iter().enumerate() {
        if event["pages"].is_array() {
        for (page_num, page) in event["pages"].as_array().unwrap().iter().enumerate() {
            if page["list"].is_array() {
            for (list_num, list) in page["list"].as_array().unwrap().iter().enumerate() {
                if list["code"] == 101 {
                    //New character is speaking
                    if line.context != "" {
                        untranslated_lines.push(line);
                    }

                    let speaker = String::from(list["parameters"][0].as_str().unwrap());
                    let context = String::from(format!("{}/events/{}/pages/{}/list/{}", parser.file_name, event_num, page_num, list_num));
                    line = UntransLine{context, speaker, data: Vec::new()};
                }
                else if list["code"] == 401 {
                    if list["parameters"].is_array() {
                    for param in list["parameters"].as_array().unwrap().iter() {
                        //line.context = String::from(format!("{}/events/{}/pages/{}/list/{}", parser.file_name, event_num, page_num, list_num));
                        line.data.push(String::from(param.as_str().unwrap()));
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

pub fn write_to_file(parser: &Parser, lines: &Vec<UntransLine>) {

}