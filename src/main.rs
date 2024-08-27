use std::fs::File;
use std::io::{self, Read, Write};

// Read the content of a file into a String
fn read_markdown_file(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// Extract headers from the Markdown content
fn extract_headers(contents: &str) -> Vec<(usize, String)> {
    let mut headers = Vec::new();
    for line in contents.lines() {
        if line.starts_with('#') {
            let level = line.chars().take_while(|&c| c == '#').count();
            let header = line[level..].trim().to_string();
            headers.push((level, header));
        }
    }
    headers
}

// Generate a table of contents from the headers
fn generate_toc(headers: Vec<(usize, String)>) -> String {
    let mut toc = String::new();
    toc.push_str("<!-- START OF TOC !DO NOT EDIT THIS CONTENT MANUALLY-->\n");
    toc.push_str("**Table of Contents**  *generated with [mtoc](https://github.com/containerscrew/mtoc)*\n");
    for (level, header) in headers {
        let indent = "  ".repeat(level - 1);
        let anchor = header.to_lowercase().replace(" ", "-");
        toc.push_str(&format!("{}- [{}](#{})\n", indent, header, anchor));
    }
    toc.push_str("<!-- END OF TOC -->\n");
    toc
}

// Remove any existing TOC marked by comments
fn remove_existing_toc(content: &str) -> String {
    let mut lines = content.lines();
    let mut new_content = String::new();
    let mut in_toc_section = false;

    // Detect and remove existing TOC section marked by comments
    for line in lines.by_ref() {
        if line.contains("<!-- START OF TOC !DO NOT EDIT THIS CONTENT MANUALLY-->") {
            in_toc_section = true;
            continue;
        }
        if in_toc_section && line.contains("<!-- END OF TOC -->") {
            in_toc_section = false;
            continue;
        }
        if !in_toc_section {
            new_content.push_str(&format!("{}\n", line));
        }
    }

    new_content
}

// Insert the TOC at the beginning of the file content
fn insert_toc(original_content: &str, toc: &str) -> String {
    let mut new_content = String::new();

    // Insert the new TOC
    new_content.push_str(toc);
    new_content.push_str("\n");

    // Add the remaining content
    new_content.push_str(original_content);

    new_content
}

fn main() -> io::Result<()> {
    let file_path = "README.md"; // Change this to the name of the file you want to update

    // Read the original file content
    let contents = read_markdown_file(file_path)?;

    // Extract headers and generate TOC
    let headers = extract_headers(&contents);
    let toc = generate_toc(headers);

    // Remove any existing TOC from the original content
    let content_without_toc = remove_existing_toc(&contents);

    // Insert the new TOC at the beginning of the content
    let updated_content = insert_toc(&content_without_toc, &toc);

    // Write the updated content back to the file
    let mut file = File::create(file_path)?;
    file.write_all(updated_content.as_bytes())?;

    println!("Table of Contents updated in the file.");

    Ok(())
}
