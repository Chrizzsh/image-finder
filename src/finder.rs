use std::env;
use std::io;
use std::path::{Path, PathBuf};
use std::ffi::OsStr;
use std::fs::metadata;

fn main() {
    let argc = env::args().count(); 

    match argc {
        1 => {
            let arg = env::args().next().unwrap();
            let path = Path::new(&arg[..]);
            let files = find_files(Path::new(path)).unwrap();
            print_result(&files);
        }
        _ => println!("Invalid number of arguments"),
    };
}

pub fn find_files(path: &Path) -> Result<Vec<(&str, u64)>, &'static str> {
    let filter_extensions = vec!("png", "jpg");
    let root_dir = Path::new(&path);
    
    // Check that path is directory
    if !root_dir.is_dir() {
        return Err("Not a directory");
    }
    
    // Go recursively through dir contents
    let mut list: Vec<(&str, u64)> = Vec::new();
    search_dir(&root_dir.to_path_buf(), &mut list, &filter_extensions);
    Ok(list)
}

fn search_dir(
    dir_path: &PathBuf, 
    list: &mut Vec<(&str, u64)>, 
    filter_extensions: &Vec<&str>
) -> Result<(), &'static str> {
    // Get contents of directory
    let contents_res = get_dir_contents(dir_path);
    
    match contents_res {
        Ok(contents) => {
            // Iterate through contents
            for item in contents {
                if item.is_dir() {
                    search_dir(&item, list, &filter_extensions)?;
                } else {
                    // Check that file is in extensions
                    if filter_file(&item, &filter_extensions){
                        let file_path = item.to_str().unwrap();
                        let file_size = metadata(item).unwrap().len();
                        list.push((file_path, file_size));
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

fn print_result(file_list: &Vec<(&str, u64)>) {
    
}