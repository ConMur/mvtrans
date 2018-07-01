extern crate mvtrans;
extern crate clap;

use std::path::Path;
use std::path::PathBuf;
use std::io;
use std::io::ErrorKind;
use std::fs;
use std::process;

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
    let mut patch_given = false;
    let mut output_given = false;

    if let Some(input) = matches.value_of("input") {
        input_dir = Some(PathBuf::from(input));

        //Create default directories
        if let Some(patch) = matches.value_of("patch") {
            patch_dir = Some(PathBuf::from(patch));
            patch_given = true;
        }
        else {
            //TODO: replace with log - warn
            eprintln!("WARN: No patch directory provided!");
                
            //Set the path directory to default
            let mut default_patch_dir = input_dir.clone().unwrap();
            let mut file_name = default_patch_dir.file_name().unwrap().to_os_string();
            file_name.push("_patch");
            default_patch_dir.set_file_name(file_name);

            //Create the default directory
            if !default_patch_dir.exists() {
                let result = fs::create_dir(default_patch_dir.clone());
                match result {
                    Ok(val) => {/*do nothing*/}
                    Err(e) => {eprintln!("{}", e); process::exit(1);}
                }
            }
            
            patch_dir = Some(PathBuf::from(default_patch_dir));
        }

        if let Some(output) = matches.value_of("output"){
            output_dir = Some(PathBuf::from(output));
            output_given = true;
        }
        else {
            //TODO: replace with log - warn
            eprintln!("WARN: No output directory provided!");

            //Set the path directory to default
            let mut default_output_dir = input_dir.clone().unwrap();
            let mut file_name = default_output_dir.file_name().unwrap().to_os_string();
            file_name.push("_translated");
            default_output_dir.set_file_name(file_name);

            //Create the default directory if it does not already exist
            if !default_output_dir.exists() {
                let result = fs::create_dir(default_output_dir.clone());
                match result {
                    Ok(val) => {/*do nothing*/}
                    Err(e) => {eprintln!("{}", e); process::exit(1);}
                }
            }

            output_dir = Some(PathBuf::from(default_output_dir));
        }

        let input_dir = input_dir.unwrap();
        let patch_dir = patch_dir.unwrap();
        let output_dir = output_dir.unwrap();

        //Ensure these are directories
        if !input_dir.is_dir() {
            eprintln!("ERROR: Input path must be a directory!");
            process::exit(1);
        }
        if patch_given && !patch_dir.is_dir(){
            eprintln!("ERROR: Patch path must be a directory!");
            process::exit(1);
        }
        if output_given && !output_dir.is_dir() {
            eprintln!("ERROR: Output path must be a directory!");
            process::exit(1);
        }

        if empty_dir(&patch_dir) {
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
        process::exit(1);
    }
}

fn empty_dir(dir: &Path) -> bool {
    if dir.is_dir() {
        let entry = fs::read_dir(dir).unwrap();

        return entry.count() == 0;
    }
    else {
        return false;
    }
}
