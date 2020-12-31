// TODO check out walkdir
// https://docs.rs/walkdir/2.3.1/walkdir/
use std::env;
use std::path::{Path, PathBuf};
use std::fs;
use glob::glob;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    match args.len() {
        2 => {
            let path_arg = &args[1][..]; // Convert String to &str slice
            println!("{}", path_arg);
            let files = find_files(path_arg).unwrap();
            print_result(&files);
        }
        _ => println!("Invalid number of arguments"),
    };
}

pub fn find_files(path: &str) -> Result<Vec<(String, u64)>, &'static str> {
    let filter_extensions = vec!("png", "jpg"); // TODO read from optional config file
    let path = Path::new(&path);
    
    // Check that given path is a directory
    if !path.is_dir(){
        return Err("Not a directory");
    }
    // Make sure path is absolute
    // TODO add absolute path if possible
    if !path.is_absolute() {
        return Err("Path not absolute");
    }

    // Find files that has the correct extension
    let mut file_names: Vec<PathBuf> = Vec::new();
    for extension in filter_extensions {
        let pattern = &format!("{}/**/*.{}", path.to_str().unwrap(), extension)[..];
        for entry in glob(pattern).expect("Pattern error") {
            file_names.push(entry.unwrap());
        }
    }

    // Find file sizes
    let mut file_sizes: Vec<u64> = Vec::new();
    for file in &file_names {
        file_sizes.push(fs::metadata(file).unwrap().len());
    }
   
    let mut file_list: Vec<(String, u64)> = Vec::new();
    for (name, size) in file_names.iter().zip(file_sizes.iter()) {
        file_list.push((String::from(name.to_str().unwrap()), *size));
    }
    
    Ok(file_list)
}

fn print_result(file_list: &Vec<(String, u64)>) {
    for entry in file_list {
        println!("Name: {}, Size: {}", entry.0, entry.1);
    }
}