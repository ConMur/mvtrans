extern crate serde_json;
extern crate rusqlite;

use serde_json::Value;

use rusqlite::Connection;

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
    pub line : String,
}

struct Line {
    line : String,
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
    let mut line = UntransLine{context: "".to_string(), speaker: "".to_string(), line:"".to_string()};

    let conn = Connection::open_in_memory().unwrap();
    conn.execute("PRAGMA encoding=\"UTF-8\";", &[]).unwrap();
    conn.execute("CREATE TABLE trans (
                  line    TEXT PRIMARY KEY NOT NULL
                  );", &[]).unwrap();
    conn.execute("CREATE TABLE context (
                  context TEXT NOT NULL,
                  speaker TEXT,
                  line TEXT,
                  FOREIGN KEY(line) REFERENCES trans(line)
                  );", &[]).unwrap();             

    let mut speaker = String::from("");

    for (event_num, event) in parser.json_data["events"].as_array().unwrap().iter().enumerate() {
        if event["pages"].is_array() {
        for (page_num, page) in event["pages"].as_array().unwrap().iter().enumerate() {
            if page["list"].is_array() {
            for (list_num, list) in page["list"].as_array().unwrap().iter().enumerate() {
                if list["code"] == 101 {
                    //New character is speaking
                    speaker = String::from(list["parameters"][0].as_str().unwrap());   
                }
                else if list["code"] == 401 {
                    if list["parameters"].is_array() {
                    for param in list["parameters"].as_array().unwrap().iter() {
                        let context = String::from(format!("{}/events/{}/pages/{}/list/{}", parser.file_name, event_num, page_num, list_num));
                        let dialogue = String::from(param.as_str().unwrap());
                        let speaker_clone = speaker.clone();
                        line = UntransLine{context, speaker: speaker_clone, line: dialogue};

                        conn.execute("INSERT OR IGNORE INTO trans (line)
                                      VALUES (?1);", 
                                      &[&line.line]).unwrap();
                        conn.execute("INSERT INTO context(context, speaker, line)
                                      VALUES (?1, ?2, ?3);",
                                      &[&line.context, &line.speaker, &line.line]).unwrap();

                        println!("INSERTED LINE: {}", line.line);
                    }
                    }
                }
            }
            }
        }
        }
    }

    let mut stmt = conn.prepare("SELECT line FROM trans;").unwrap();
    let line_iter = stmt.query_map(&[], |row| {
        Line {
            line : row.get(0)
        }
    }).unwrap();


    for line in line_iter {
        let dialogue = escape_quotes(line.unwrap().line);

        let mut stmt = conn.prepare(format!("SELECT context, speaker FROM context WHERE line = \'{}\';", dialogue.clone()).as_str()).unwrap();
        let context_iter = stmt.query_map(&[], |row| {
            UntransLine {
                context: row.get(0),
                speaker: row.get(1),
                line: dialogue.clone(),
            }
        }).unwrap();

        for context in context_iter {
            untranslated_lines.push(context.unwrap());
        }
    }

    untranslated_lines
}

pub fn write_to_file(parser: &Parser, lines: &Vec<UntransLine>) {
    let mut file_name = parser.file_name.clone();
    file_name.push_str(".txt");
    let mut file = File::create(file_name.as_str()).unwrap();

    for line in lines.iter() {
        file.write_all(line.context.as_bytes());
        file.write_all(b"\n");
        file.write_all(line.speaker.as_bytes());
        file.write_all(b"\n");
        file.write_all(line.line.as_bytes());
        file.write_all(b"\n");
    }
}

fn escape_quotes(line : String) -> String {
    let mut v: Vec<char> = Vec::new();

    for c in  line.chars() {
        v.push(c);
        if c == '"' {
            v.push('"');
        }
    }

    v.into_iter().collect()
}