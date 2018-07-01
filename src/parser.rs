use serde_json::Value;
use serde_json;

use rusqlite::Connection;

use std::path::PathBuf;
use std::fs::File;
use std::fs;
use std::io::prelude::*;
use std::string::String;
use std::ffi::OsString;

pub struct Parser {
    input_dir : PathBuf,
    json_data : Vec<(Value, String)>,
    untranslated_lines: Vec<UntransLine>,
    file_names : Vec<String>,
}   

pub struct UntransLine {
    pub context : Vec<String>,
    pub speaker: Vec<String>,
    pub line : String,
}

impl Parser {
    /// Creates a new parser and loads in the contents of the files the parse
    /// 
    /// # Arguments
    /// * `input_dir` - The directory with all the files to parse
    /// 
    /// # Returns 
    /// A parser
    pub fn new(input_dir: &PathBuf) -> Parser {
        let entries = fs::read_dir(&input_dir.as_path()).unwrap();

        let mut json_data = Vec::new();
        let mut file_names = Vec::new();

        //Load all the json data
        for entry in entries {
            let path = entry.unwrap().path();
            let file_name = String::from(path.file_name().unwrap().to_str().unwrap());
            file_names.push(file_name.clone());
            let mut file = File::open(path).expect("Invalid file provided");
            let mut contents = String::new();
            file.read_to_string(&mut contents).expect("Could not read from file"); 

            let data : Value = serde_json::from_str(&contents).expect("Unable to parse JSON data");
            json_data.push((data, file_name));
        }

        Parser {input_dir: input_dir.clone(), json_data, untranslated_lines: Vec::new(), file_names: file_names}
    } 

    /// Parses the given file into lines grouped together if they are the same.
    ///
    /// #Arguments
    /// * `parser` - the parser used to parse the file
    ///
    /// #Returns
    /// A vector of untranslated lines grouped together if they are the same line
    pub fn parse(&mut self) {
        for data in self.json_data.iter() {
            //Remove the 'json' from the end of the file name
            let (ref data, ref file_name) = data;

            //Create the tables in the in memory database
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

            //Read through each line in the file
            for (event_num, event) in data["events"].as_array().unwrap().iter().enumerate() {
                if event["pages"].is_array() {
                for (page_num, page) in event["pages"].as_array().unwrap().iter().enumerate() {
                    if page["list"].is_array() {
                    for (list_num, list) in page["list"].as_array().unwrap().iter().enumerate() {
                        process_mv_code(list, &conn, &file_name, event_num, page_num, list_num);
                    }
                    }
                }
                }
            }

            self.untranslated_lines = collect_lines(&conn);
        }
    }


    /// Writes parsed results to a file
    ///
    /// #Arguments
    /// * `lines` - The lines that were parsed
    /// * `patch_dir` - The directory to place the patch
    pub fn write_to_file(&mut self, patch_dir: &PathBuf) {
        for file_name in self.file_names.iter_mut() {
            //Remove the 'json' from the end of the file name
            let pos = file_name.rfind('.').unwrap();
            file_name.split_off(pos);

            //Add the .txt extension
            file_name.push_str(".txt");

            //Create the file and write to it
            let mut path = patch_dir.clone();
            path.push(file_name);
            let mut file = File::create(path).unwrap();

            //File version
            file.write_all(b"> RPGMAKER TRANS PATCH FILE VERSION 3.2\n").unwrap();
            for line in self.untranslated_lines.iter() {
                file.write_all(b"> BEGIN STRING\n").unwrap();
                file.write_all(line.line.as_bytes()).unwrap();
                file.write_all(b"\n").unwrap();

                for context in line.context.iter() {
                    file.write_all(b"> CONTEXT: ").unwrap();
                    file.write_all(context.as_bytes()).unwrap();
                    file.write_all(b"\n").unwrap();
                }
                file.write_all(b"\n").unwrap();

                file.write_all(b"> END STRING\n").unwrap();
            }
        }
    }
}


/// Takes the lines in the database and returns them organized into a `Vec<UntransLine>`. It groups together
/// lines that are the same and makes note about which context they are from
///
/// #Arguments
/// * `conn` - the connection to the database
///
/// #Returns
/// Returns the untranslated lines grouped together if they are the same line
fn collect_lines(conn: &Connection) -> Vec<UntransLine>{
    let mut untranslated_lines = Vec::new();

    let mut stmt = conn.prepare("SELECT line FROM trans;").unwrap();
    let line_iter = stmt.query_map(&[], |row| {
            row.get(0)
    }).unwrap();


    for l in line_iter {
        let dialogue = escape_quotes(l.unwrap());

        let mut stmt = conn.prepare(format!("SELECT context, speaker FROM context WHERE line = \'{}\';", dialogue.clone()).as_str()).unwrap();
        let context_iter = stmt.query_map(&[], |row| {
            row.get(0)
        }).unwrap();

        let mut contexts = Vec::new();

        for context in context_iter {
            contexts.push(context.unwrap());
        }

        let dialogue = unescape_quotes(dialogue);
        untranslated_lines.push(UntransLine{context: contexts, speaker: Vec::new(), line: dialogue})
    }

    untranslated_lines
}


/// Puts the line in the given list into the database depending on its code
///
/// #Arguments
/// * `list` - the list of data about this line
/// * `conn` - the connection to the database. Used to insert into the database
/// * `file_name` - the name of the file the line comes from without the extension
/// * `event_num` - the number of the event this line is in
/// * `page_num` - the number of the page this line is in
/// * `list_num` - the number of the list this line is in
///
/// #Remarks
/// * 101 - A new character is speaking
/// * 401 - A line of dialogue
/// * 402
/// * 403
fn process_mv_code(list: &serde_json::Value, conn: &Connection, file_name: &String, event_num: usize, page_num: usize, list_num: usize){
    if list["code"] == 101 {
        //New character is speaking 
    }
    else if list["code"] == 401 {
        if list["parameters"].is_array() {
        for param in list["parameters"].as_array().unwrap().iter() {
            let context = String::from(format!("{}/events/{}/pages/{}/list/{}", file_name, event_num, page_num, list_num));
            //TODO: see if we can work with a &Value the allow escaped characters such as \"
            let dialogue = String::from(param.as_str().unwrap());
            let speaker_clone = String::from("");

            conn.execute("INSERT OR IGNORE INTO trans (line)
                          VALUES (?1);", 
                        &[&dialogue]).unwrap();
            conn.execute("INSERT INTO context(context, speaker, line)
                          VALUES (?1, ?2, ?3);",
                         &[&context, &speaker_clone, &dialogue]).unwrap();
        }
        }
    }
}



/// This method changes \" into "" in the given line
///
/// #Arguments 
/// `line` - The line to escape
///
/// #Returns
/// The escaped line
///
/// #Remarks
/// We use this function because RPGMakerMV uses \" to escape quotes but SQLite uses "". 
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

/// This method changes "" into \" in the given line
///
/// #Arguments 
/// `line` - The line to unescape
///
/// #Returns
/// The unescaped line
///
/// #Remarks
/// We use this function because RPGMakerMV uses \" to escape quotes but SQLite uses "". 
fn unescape_quotes(line: String) -> String {
    let mut v: Vec<char> = Vec::new();

    let mut first_quote = false;

    for c in line.chars() {
        if c == '"' && first_quote == false {
            v.push('\\');
            first_quote = true;
            continue;
        }

        first_quote = false;
        v.push(c);
    }

    v.into_iter().collect()
}