use std::io;
use std::path::{Path, PathBuf};
use std::ffi::OsStr;

use imgui::*;
mod support;

fn main() {
    match run_app() {
        Ok(()) => {},
        Err(err) => eprintln!("{}", err),
    };
    
}

fn run_app() -> Result<(), &'static str> {
    let filter_extensions = vec!("png", "jpg");

    // Read path from input
    let mut input = String::new();
    let stdin = io::stdin().read_line(&mut input);

    match stdin {
        Err(_) => return Err("Error reading input"),
        _ => {}
    }

    // Trim trailing line break
    let input = input.trim();
    let root_dir = Path::new(&input);
    
    // Check that path is directory
    if !root_dir.is_dir() {
        return Err("Not a directory");
    }
    
    // Go recursively through dir contents
    print_files(&root_dir.to_path_buf(), &filter_extensions)
}

fn print_files(dir_path: &PathBuf, filter_extensions: &Vec<&str>) -> Result<(), &'static str> {
    // Get contents of directory
    let contents_res = get_dir_contents(dir_path);
    
    match contents_res {
        Ok(contents) => {
            // Iterate through contents
            for item in contents {
                if item.is_dir() {
                    print_files(&item, &filter_extensions)?;
                } else {
                    // Check that file is in extensions
                    if filter_file(&item, filter_extensions){
                        println!("{}", item.display());
                    }
                }
            }
        }
        Err(_) => return Err("io error")
    }
    Ok(())
}

// Takes a directory path and return a vector of PathBuf with all entries in the given directory
fn get_dir_contents(dir_path: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut entries: Vec<PathBuf> = Vec::new();
    for entry in dir_path.read_dir()? {
        match entry {
            Err(e) => return Err(e),
            Ok(f) => entries.push(f.path()),
        }
    }
    return Ok(entries);
}

fn filter_file(path: &Path, filter_extensions: &Vec<&str>) -> bool {
    let extension = path
        .extension()
        .and_then(OsStr::to_str);

    match extension {
        Some(e) => {
            filter_extensions
                .iter()
                .any(|ex| *ex == e)
        },
        None => false
    }
}