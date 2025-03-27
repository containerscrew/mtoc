use std::path::Path;
use walkdir::{DirEntry, WalkDir};

// Check if a directory should be excluded
fn is_excluded_dir(entry: &DirEntry, exclude_dirs: &[String]) -> bool {
    let path = entry.path();
    exclude_dirs
        .iter()
        .any(|exclude| path.to_string_lossy().contains(exclude))
}

// Find markdown files in the given directory
pub fn find_markdown_files(dir: &Path, exclude_dirs: &[String]) -> Vec<String> {
    let mut markdown_files = Vec::new();
    for entry in WalkDir::new(dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.path().is_file() && !is_excluded_dir(&entry, exclude_dirs) {
            if let Some(ext) = entry.path().extension() {
                if ext == "md" {
                    markdown_files.push(entry.path().to_string_lossy().to_string());
                }
            }
        }
    }
    markdown_files
}
