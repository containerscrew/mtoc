mod cli;
mod delete;
mod read;
mod utils;
mod write;

use crate::cli::Args;
use crate::delete::remove_existing_toc;
use crate::read::{extract_headers, read_markdown_file};
use crate::utils::find_markdown_files;
use crate::write::{generate_toc, insert_toc};
use clap::Parser;
use colored::*;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

fn main() -> io::Result<()> {
    // Parse command-line arguments
    let args = Args::parse();

    // Exclude directories to scan
    let exclude_dirs = args.exclude.as_deref().unwrap_or(&[]);
    if !exclude_dirs.is_empty() {
        eprintln!(
            "{} {:?}",
            "Excluding directories ".bright_red(),
            exclude_dirs
        );
    }

    // Only generate TOC for the specified file(s)
    let markdown_files = if let Some(files) = args.file.as_deref() {
        files.to_vec()
    } else {
        // Find markdown files in the given directory
        find_markdown_files(Path::new(&args.directory), exclude_dirs)
    };

    for markdown_file in &markdown_files {
        // Read the original file content
        let contents = read_markdown_file(markdown_file.as_str())?;

        // Extract headers and generate TOC
        let headers = extract_headers(&contents);
        let toc = generate_toc(headers);

        // Remove any existing TOC from the original content
        let (content_without_toc, toc_start_index) = remove_existing_toc(&contents);

        // Insert the new TOC at the beginning of the content
        let updated_content = insert_toc(&content_without_toc, &toc, toc_start_index);

        // Write the updated content back to the file
        let mut file = File::create(markdown_file.as_str())?;
        file.write_all(updated_content.as_bytes())?;

        eprintln!("{} {}", "Updated markdown file ".green(), markdown_file);
    }

    if markdown_files.is_empty() {
        eprintln!("{}", "No markdown files found in the given directory".red());
    }

    Ok(())
}
