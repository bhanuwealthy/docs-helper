mod tests;

use pathdiff::diff_paths;
use std::path::{Path, PathBuf};
use std::{fs, io};

fn get_files_in_folder(path: &str) -> io::Result<Vec<PathBuf>> {
    let entries = fs::read_dir(path)?;
    let all: Vec<PathBuf> = entries
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .collect();
    Ok(all)
}

fn copy_docs(source: &Path, destination: &Path) -> io::Result<()> {
    fs::create_dir_all(destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let filetype = entry.file_type()?;
        if filetype.is_dir() {
            continue;
        } else {
            println!("Copying to: {}",destination.join(entry.file_name()).display());
            fs::copy(entry.path(), destination.join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn helper(search_dir_path: &str, destination_dir_path: &str) {
    match get_files_in_folder(search_dir_path) {
        Ok(files) => {
            for file in files {
                if file.is_dir() {
                    println!("{} is a directory", file.display());
                    if let Some(file_name) = file.file_name().and_then(|f| f.to_str()) {
                        if file_name == "docs" {
                            println!("docs folder found, copying all contents");
                            let _ = copy_docs(
                                &file,
                                &Path::new(&format!(
                                    "{}/{}",
                                    destination_dir_path,
                                    diff_paths(file.parent().unwrap(), destination_dir_path).unwrap().to_str().unwrap().replace("../", "").as_str().replace("docs/", "")
                                )),
                            );
                        }
                    }
                    helper(&file.to_str().unwrap(), destination_dir_path);
                } else if file.is_symlink() {
                    println!("{} is a symlink", file.display());
                } else {
                    if let Ok(m) = file.metadata() {
                        if m.len() == 0 {
                            println!("{} is an empty file", file.display());
                        }
                    } else {
                        println!("Could not get metadata for {}", file.display());
                    }
                }
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let source_path = std::env::args()
        .nth(1)
        .expect("No source folder path provided!");

    let destination_path = std::env::args()
        .nth(2)
        .expect("Destination folder path not provided!");

    // Call the helper function with the provided paths
    helper(source_path.as_str(), destination_path.as_str());
}
