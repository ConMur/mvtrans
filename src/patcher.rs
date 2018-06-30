use std::fs::File;
use std::fs;
use std::io::prelude::*;
use std::string::String;

use std::path::PathBuf;
use std::io::BufReader;

use serde_json;
use serde_json::Value;

pub struct Patcher {
    json_data : Value,
    reader : BufReader<File>,
}

impl Patcher {
    /// Creates a new patcher 
    /// 
    /// #Arguments
    /// * source_file - a path to the file containing the original json file
    /// * patch_file - a path to the file containing the translated patch
    /// 
    /// #Returns
    /// A new patcher
    pub fn new(source_file : &PathBuf, patch_file : &PathBuf) -> Patcher {
        let mut source = File::open(source_file).expect("Invalid file provided");
        let mut patch = File::open(patch_file).expect("Invalid file provided");
        
        let mut contents = String::new();
        source.read_to_string(&mut contents).expect("Could not read from file"); 
        let mut json_data : Value = serde_json::from_str(&contents).expect("Unable to parse JSON data");

        let mut reader = BufReader::new(patch);

        Patcher{json_data, reader}
    }

    /// Applies the patch to the data in memory
    pub fn patch(&mut self) {
        let mut contexts = Vec::new();
        let mut last_line_was_begin = false;

        for line in self.reader.by_ref().lines() { 
            let mut l = line.unwrap();
            if l.starts_with("> ") {
                //This is a control line

                if l.starts_with("> CONTEXT: ") {
                    l = l.split_off(11);
                    contexts.push(l);
                }
                else if l.contains("> BEGIN STRING") {
                    last_line_was_begin = true;
                }
                else if l.contains("> END STRING") {
                    //Reset contexts
                    contexts.clear();
                }
            }
            else {
                //This is either a translated or untranslated line
                if !last_line_was_begin {
                    //This is a translated line
                    for context in contexts.iter() {
                        let (event, page, list) = parse_context(context.clone());
                        
                        self.json_data["events"][event]["pages"][page]["list"][list] = serde_json::Value::String(l.clone());
                    }
                }
                //Untranslated lines are always after a > BEGIN STRING so this is where we reset the bool
                last_line_was_begin = false;
            }
        }
    }  

    /// Writes the patch to a file
    /// 
    /// #Arguments
    /// * out_file - the path to the file to write to
    pub fn write_to_file(&self, out_file: &PathBuf) {
        fs::write(out_file,  self.json_data.to_string()).expect("Unable to write to file");
    }
}

/// Looks through a context line to determine the event, page and list numbers
/// 
/// #Arguments
/// * context - the context to parse
/// 
/// #Returns
/// A tuple containing (event, page, list) numbers as usize
/// 
/// #Remarks
/// A context looks like: Map018/events/6/pages/0/list/100
fn parse_context(context: String) -> (usize, usize, usize) {
    //RPGMaker MV uses a backslash ('\') to escape characters so splitting by forward slash should be fine
    let parts: Vec<&str> = context.split("/").collect();

    let event : usize = parts[2].parse().unwrap();
    let page : usize = parts[4].parse().unwrap();
    let list : usize = parts[6].parse().unwrap();

    (event, page, list)
}

