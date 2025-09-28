//! This utility helps in consolidating documentation directories across a project.
//! It scans a given root directory for all subdirectories named "docs" (case-insensitive),
//! copies their contents to a specified target directory, and reconstructs the path
//! structure, omitting the "docs" segment. This allows for a flattened or
//! reorganized documentation output.

use std::fs;
use std::io::{self};
use std::path::{Path, PathBuf};
use std::process::Command;

use walkdir::{DirEntry, WalkDir};

/// The name of the directory to search for, case-insensitive.
const FIND_THIS_DIR: &str = "docs";

/// Directories that should be skipped entirely during the traversal.
/// These are common project-related or dependency directories that do not
/// typically contain relevant documentation.
const DEFAULT_IGNORE_PATTERNS: [&str; 9] = [
    "venv",
    "site-packages",
    "__pycache__",
    "node_modules",
    ".git",
    "target",
    "build",
    "third_party",
    "tests",
];

/// Normalizes a given path string by ensuring it ends with a trailing slash.
/// If the path already ends with a slash, it is returned as is.
/// Otherwise, a slash is appended.
///
/// # Arguments
///
/// * `path` - A string slice representing the path to normalize.
///
/// # Returns
///
/// A `String` containing the normalized path with a trailing slash.
///
/// # Examples
///
/// ```
/// assert_eq!(normalize_path("path/to/dir"), "path/to/dir/");
/// assert_eq!(normalize_path("path/to/dir/"), "path/to/dir/");
/// ```
fn normalize_path(path: &str) -> String {
    if path.ends_with('/') {
        path.to_string()
    } else {
        format!("{}/", path)
    }
}

/// Resolves a given path to its canonical, absolute form.
/// This function will panic if the path cannot be resolved.
///
/// # Arguments
///
/// * `p` - A string slice representing the path to resolve.
///
/// # Returns
///
/// A `String` containing the canonicalized path.
///
/// # Panics
///
/// Panics if the path cannot be canonicalized (e.g., if it does not exist).
///
/// # Examples
///
/// ```no_run
/// // Assuming "/tmp" exists
/// let resolved = resolve_path("/tmp/../tmp");
/// // On Unix, this might resolve to "/private/tmp" or "/tmp"
/// // assert!(resolved.ends_with("/tmp/"));
/// ```
fn resolve_path(p: &str) -> String {
    fs::canonicalize(Path::new(p))
        .unwrap_or_else(|e| panic!("Could not resolve path: {} --err={}", p, e))
        .to_string_lossy()
        .to_string()
}

/// Cleans up the target directory by recursively removing its contents if it exists,
/// and then recreating it. This ensures a clean slate for copying documentation.
///
/// # Arguments
///
/// * `path` - A string slice representing the path to the target directory.
///
/// # Errors
///
/// Returns an `io::Result` indicating whether the operation was successful.
/// An error is returned if directory removal or creation fails.
///
/// # Examples
///
/// ```no_run
/// use std::fs;
/// use std::io;
/// // Create a dummy directory for testing
/// let _ = fs::create_dir_all("test_target/subdir");
/// let _ = fs::write("test_target/subdir/file.txt", "content");
///
/// // Clean up the directory
/// cleanup_dir("test_target").unwrap();
///
/// // Assert that the directory exists but is empty
/// assert!(fs::metadata("test_target").unwrap().is_dir());
/// assert!(fs::read_dir("test_target").unwrap().next().is_none());
///
/// // Clean up the created directory
/// let _ = fs::remove_dir("test_target");
/// ```
fn cleanup_dir(path: &str) -> io::Result<()> {
    println!("Cleaning the target dir: {}", path);
    let target = PathBuf::from(path);
    if target.exists() {
        fs::remove_dir_all(&target)?;
    }
    fs::create_dir_all(&target)?;
    Ok(())
}

/// Copies a directory from a source path to a destination path using the `cp -r` command.
///
/// # Arguments
///
/// * `src` - A string slice representing the source directory path.
/// * `dest` - A string slice representing the destination directory path.
///
/// # Errors
///
/// Returns an `io::Result` indicating whether the operation was successful.
/// An error is returned if the `cp` command fails or returns a non-zero exit status.
///
/// # Examples
///
/// ```no_run
/// use std::fs;
/// use std::io;
/// // Create dummy source and destination directories
/// let _ = fs::create_dir_all(\"source_dir/docs\");
/// let _ = fs::write(\"source_dir/docs/file.txt\", \"content\");
/// let _ = fs::create_dir_all(\"target_dir\");
///
/// // Copy the docs directory
/// copy_docs_dir(\"source_dir/docs\", \"target_dir/copied_docs\").unwrap();
///
/// assert!(fs::metadata(\"target_dir/copied_docs/file.txt\").unwrap().is_file());
///
/// // Clean up
/// let _ = fs::remove_dir_all(\"source_dir\");
/// let _ = fs::remove_dir_all(\"target_dir\");
/// ```
fn copy_docs_dir(src: &str, dest: &str) -> io::Result<()> {
    let src = normalize_path(src);
    let dest = normalize_path(dest);

    let output = Command::new("cp").arg("-r").arg(&src).arg(&dest).output()?;

    if output.status.success() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            String::from_utf8_lossy(&output.stderr).trim().to_string(),
        ))
    }
}

/// Determines whether a given directory entry should be traversed by the `WalkDir` iterator.
/// This function filters out non-directories, hidden directories (starting with '.'),
/// special directories (starting with '_'), and directories matching `DEFAULT_IGNORE_PATTERNS`.
///
/// # Arguments
///
/// * `entry` - A reference to a `DirEntry` to evaluate.
///
/// # Returns
///
/// `true` if the directory should be traversed, `false` otherwise.
///
/// # Examples
///
/// ```no_run
/// use walkdir::{DirEntry, WalkDir};
/// use std::path::PathBuf;
/// // Assume a DirEntry `entry` for a directory named "my_project"
/// // let entry: DirEntry = ...;
/// // assert_eq!(should_traverse(&entry), true);
///
/// // Assume a DirEntry `entry_hidden` for a directory named ".git"
/// // let entry_hidden: DirEntry = ...;
/// // assert_eq!(should_traverse(&entry_hidden), false);
/// ```
fn should_traverse(entry: &DirEntry) -> bool {
    let name = entry.file_name().to_string_lossy();
    print!("\x1B[2K\rScanning {}", name);
    if !entry.file_type().is_dir() || name.starts_with('.') || name.starts_with('_') {
        return false;
    }
    if DEFAULT_IGNORE_PATTERNS.iter().any(|p| name.contains(p)) {
        return false;
    }
    return true;
}

/// Filters `DirEntry` objects, returning true only for directories
/// that are named "docs" (case-insensitive).
///
/// # Arguments
///
/// * `entry` - A reference to a `DirEntry` to evaluate.
///
/// # Returns
///
/// `true` if the entry is a directory named "docs", `false` otherwise.
///
/// # Examples
///
/// ```no_run
/// use walkdir::{DirEntry, WalkDir};
/// use std::path::PathBuf;
/// // Assume a DirEntry `entry_docs` for a directory named "docs"
/// // let entry_docs: DirEntry = ...;
/// // assert_eq!(is_docs_dir(&entry_docs), true);
///
/// // Assume a DirEntry `entry_other` for a directory named "src"
/// // let entry_other: DirEntry = ...;
/// // assert_eq!(is_docs_dir(&entry_other), false);
/// ```
fn is_docs_dir(entry: &DirEntry) -> bool {
    entry.file_type().is_dir()
        && entry
            .file_name()
            .to_string_lossy()
            .eq_ignore_ascii_case(FIND_THIS_DIR)
}

/// The main function of the documentation helper utility.
/// It takes two command-line arguments: a root directory to scan and a target directory
/// where the consolidated documentation will be placed.
///
/// It performs the following steps:
/// 1. Parses command-line arguments and validates their count.
/// 2. Resolves and normalizes the root and target paths.
/// 3. Cleans up the target directory.
/// 4. Walks the root directory, filtering for "docs" directories using `should_traverse`
///    and `is_docs_dir`.
/// 5. For each found "docs" directory, it constructs a new path in the target
///    directory, removing the "docs" segment from the relative path.
/// 6. Copies the contents of the "docs" directory to the newly constructed path.
/// 7. Prints progress and error messages during the copying process.
/// 8. On successful completion, prints a success message.
///
/// # Arguments
///
/// * `args` - Command line arguments: `[0]` - program name, `[1]` - root directory, `[2]` - target directory.
///
/// # Errors
///
/// Returns an `io::Result` indicating whether the overall operation was successful.
/// Errors can occur during path resolution, directory cleanup, directory creation,
/// or file copying.
///
/// # Examples
///
/// To run this utility, you would typically compile it first:
/// ```bash
/// cargo build --release
/// ```
/// Then, you can execute it from your project root, specifying a root directory
/// to scan for "docs" folders and a target directory for the consolidated output:
/// ```bash
/// # Assuming 'target/release/cp_docs' is your compiled binary
/// # and you want to scan the current directory ('.') for docs and output to './dist'
/// ./target/release/cp_docs . ./dist
/// ```
/// This would find all "docs" directories within the current directory and its subdirectories,
/// and copy their contents into the `./dist` folder, reconstructing the relative paths
/// but omitting the "docs" segment. For example, if you have `my_project/src/docs`
/// and `my_project/api/docs`, their contents would be copied to `dist/src` and `dist/api`
/// respectively.
fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <root> <target>", args[0]);
        std::process::exit(1);
    }

    let root: String = resolve_path(&normalize_path(&args[1]));
    let target: String = normalize_path(&args[2]);

    cleanup_dir(&target)?;
    println!("root={}; target={}", root, target);

    // Collect all docs directories first to get total count
    let docs_dirs: Vec<PathBuf> = WalkDir::new(&root)
        .follow_links(false)
        .into_iter()
        .filter_entry(should_traverse)
        .filter_map(Result::ok)
        .filter(is_docs_dir)
        .map(|e| e.path().to_path_buf())
        .collect();

    let total: usize = docs_dirs.len();

    println!("\x1B[2K\rFound {} docs directories", total);
    let mut done: u16 = 0;
    let width = ((total as f64).log10().floor() as usize) + 1;
    for entry_path in docs_dirs {
        let relative = entry_path.strip_prefix(&root).unwrap();

        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        let tartget_path_buf = PathBuf::from(&target);
        // Construct the new path in the target directory, omitting the "docs" segment
        let constructed_path: PathBuf = tartget_path_buf.join(relative.parent().unwrap());
        let constructed_path_str = constructed_path.to_string_lossy();
        let constructed_path_str = constructed_path_str.replace("/docs/", "/"); // Replace "/docs/" with "/"
        let constructed_path = PathBuf::from(constructed_path_str);

        // Ensure parent directories exist before copying
        if let Some(parent) = constructed_path.parent() {
            if let Err(e) = fs::create_dir_all(parent) {
                eprintln!("failed to create parent {}: {}", parent.display(), e);
                continue;
            }
        }
        // Perform the copy operation
        if let Err(e) = copy_docs_dir(
            entry_path.to_str().unwrap(),
            constructed_path.to_str().unwrap(),
        ) {
            eprintln!(
                "failed copying {} -> {}: {}",
                entry_path.display(),
                constructed_path.display(),
                e
            );
        } else {
            // Increment counter and print progress
            done += 1;

            println!(
                "({:0width$}/{:0width$}) finished copying {} ",
                done,
                total,
                relative.display()
            );
        }
    }

    println!("✅ All docs directories copied successfully.");
    Ok(())
}
