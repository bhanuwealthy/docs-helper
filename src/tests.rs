use std::fmt::format;

use super::*;

#[test]
fn test_get_files_in_folder() {
    // Create a temporary directory with some files and folders
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_dir_path = temp_dir.path();
    let file1_path = temp_dir_path.join("file1.txt");
    fs::File::create(&file1_path).unwrap();
    let dir1_path = temp_dir_path.join("dir1");
    fs::create_dir(&dir1_path).unwrap();
    let file2_path = dir1_path.join("file2.txt");
    fs::File::create(&file2_path).unwrap();

    // Call the function to get files in the temporary directory
    let result = get_files_in_folder(temp_dir_path.to_str().unwrap()).unwrap();

    // Check if the result contains the expected paths
    assert_eq!(result.len(), 2);
    assert!(result.contains(&file1_path));
    // assert!(result.contains(&file2_path));
}


#[test]
fn test_full_script() {

    //Create source dir with docs fodler
    let temp_dir = tempfile::tempdir().unwrap();
    let source_dir_path = temp_dir.path().join("source");
    fs::create_dir(&source_dir_path).unwrap();
    let docs_dir_path = source_dir_path.join("docs");
    fs::create_dir(&docs_dir_path).unwrap();
    let doc1_path = docs_dir_path.join("doc1.txt");
    fs::File::create(&doc1_path).unwrap();

    //Create subfolder in the source folder

    let subfolder_dir_path = source_dir_path.join("subfolder");
    fs::create_dir(&subfolder_dir_path).unwrap();
    let subfolder_docs_dir_path = subfolder_dir_path.join("docs");
    fs::create_dir(&subfolder_docs_dir_path).unwrap();
    let doc2_path = subfolder_docs_dir_path.join("doc2.txt");
    fs::File::create(&doc2_path).unwrap();

    // Create a temporary destination directory 
    let destination_dir_path = temp_dir.path().join("destination");


    println!("SOURCE: {}\nDESTINATION: {}",source_dir_path.display(),destination_dir_path.display());
    helper(source_dir_path.to_str().unwrap(),destination_dir_path.to_str().unwrap());
    let dest_str = destination_dir_path.to_str().unwrap();
    let src_str = source_dir_path.to_str().unwrap();
    let temp_str = format!("{}/source",dest_str);
    let dest_doc1_path = Path::new(&temp_str).join("doc1.txt");
    let sub_temp_str = format!("{}/source/subfolder",dest_str);
    let dest_doc2_path = Path::new(&sub_temp_str).join("doc2.txt");
    assert!(dest_doc1_path.exists());
    assert!(dest_doc2_path.exists());
}

