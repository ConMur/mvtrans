extern crate mvtrans;
extern crate clap;

use std::path::Path;
use std::path::PathBuf;
use std::io;
use std::io::ErrorKind;
use std::fs;

use mvtrans::parser::Parser;
use mvtrans::patcher::Patcher;

use clap::{Arg, App};

fn main() {
    let matches = App::new("mvtrans")
                          .version("0.2")
                          .author("Connor")
                          .about("Creates files for translating RPGMaker MV Games and then applies those patches to the game")
                          .arg(Arg::with_name("input")
                               .help("The input directory. Must contain a supported game.")
                               .required(true)
                               .index(1))
                          .arg(Arg::with_name("patch")
                               .short("p")
                               .value_name("PATCH")
                               .takes_value(true)
                               .help("The patch to use. To use an existing patch, supply a directory containing a patch or a zip file. To create a new patch, supply a target empty directory."))
                          .arg(Arg::with_name("output")
                               .short("o")
                               .value_name("OUTPUT")
                               .takes_value(true)
                               .help("The target directory. Cannot be the same as the input directory. Supply a directory containing a previously translated game or a new directory."))
                          .get_matches();

    let mut input_dir : Option<PathBuf> = None;
    let mut patch_dir : Option<PathBuf> = None;
    let mut output_dir : Option<PathBuf> = None;

    if let Some(input) = matches.value_of("input") {
        input_dir = Some(PathBuf::from(input));

        //Create default directories
        if let Some(patch) = matches.value_of("patch") {
            patch_dir = Some(PathBuf::from(patch));
        }
        else {
            //TODO: replace with log - warn
            eprintln!("WARN: No patch directory provided!");
                
            //Set the path directory to default
            let mut default_patch_dir = input_dir.clone().unwrap().into_os_string();
            default_patch_dir.push("_patch");
            patch_dir = Some(PathBuf::from(default_patch_dir));
        }

        if let Some(output) = matches.value_of("output"){
            output_dir = Some(PathBuf::from(output));
        }
        else {
            //TODO: replace with log - warn
            eprintln!("WARN: No output directory provided!");

            let mut default_output_dir = input_dir.clone().unwrap().into_os_string();
            default_output_dir.push("_translated");
            output_dir = Some(PathBuf::from(default_output_dir));
        }

        let input_dir = input_dir.unwrap();
        let patch_dir = patch_dir.unwrap();
        let output_dir = output_dir.unwrap();

        //Ensure these are directories
        if !input_dir.is_dir() {
            eprintln!("ERROR: Input path must be a directory!");
        }
        if !patch_dir.is_dir() {
            eprintln!("ERROR: Patch path must be a directory!");
        }
        if !output_dir.is_dir() {
            eprintln!("ERROR: Output path must be a directory!");
        }

        if empty_dir(&patch_dir).unwrap() {
            //Parse stuff from the input directory
            let mut parser = Parser::new(&input_dir);
            parser.parse();

            //Write parsed data to patch directory
            parser.write_to_file(&patch_dir);
        }
        else {
            //Read from the patch folder
            let mut patcher = Patcher::new(&input_dir, &patch_dir);
            patcher.patch();

            //Write patched output to output directory
            patcher.write_to_file(&output_dir);
        }
    }
    else {
        //TODO log- error and exit
        eprintln!("ERROR: no input directory provided!");
    }
}

fn empty_dir(dir: &Path) -> io::Result<bool> {
    if dir.is_dir() {
        let entry = fs::read_dir(dir)?;

        Ok(entry.count() == 0)
    }
    else {
        Err(io::Error::new(ErrorKind::InvalidInput, "The given path is not a directory"))
    }
}
