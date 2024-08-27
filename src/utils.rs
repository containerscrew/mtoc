use colored::Colorize;
use std::path::Path;
use walkdir::WalkDir;

// Find markdown files in the given directory
pub fn find_markdown_files(dir: &Path) -> Vec<String> {
    let mut markdown_files = Vec::new();
    for entry in WalkDir::new(dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.path().is_file() {
            if let Some(ext) = entry.path().extension() {
                if ext == "md" {
                    markdown_files.push(entry.path().to_string_lossy().to_string());
                }
            }
        }
    }
    markdown_files
}
