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
use std::io::{self, Read, Write};
use std::path::Path;

fn main() -> io::Result<()> {
    // Parse command-line arguments
    let args = Args::parse();

    // Find markdown files in all the repo
    let markdown_files = find_markdown_files(Path::new(&args.directory));

    for markdown_file in &markdown_files {
        // Read the original file content
        let contents = read_markdown_file(markdown_file.as_str())?;

        // Extract headers and generate TOC
        let headers = extract_headers(&contents);
        let toc = generate_toc(headers);

        // Remove any existing TOC from the original content
        let content_without_toc = remove_existing_toc(&contents);

        // Insert the new TOC at the beginning of the content
        let updated_content = insert_toc(&content_without_toc, &toc);

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
