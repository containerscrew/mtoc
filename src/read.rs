use std::fs::File;
use std::io;
use std::io::Read;

// Read the content of a file into a String
pub fn read_markdown_file(file_path: &str) -> io::Result<String> {
    match File::open(file_path) {
        Ok(file) => {
            let mut contents = String::new();
            let mut file = file;
            file.read_to_string(&mut contents)?;
            Ok(contents)
        }
        Err(e) => Err(e),
    }

    // let mut file = File::open(file_path)?;
    // let mut contents = String::new();
    // file.read_to_string(&mut contents)?;
    // Ok(contents)
}

// Extract headers from the Markdown content
// TODO: add tests
pub fn extract_headers(contents: &str) -> Vec<(usize, String)> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::Builder;
    use tempfile::NamedTempFile;

    // Helper function to create a temporary file with the given content
    fn create_temp_file(content: &str) -> NamedTempFile {
        // Create a named temporary file with a specific prefix or suffix
        let mut tmpfile = Builder::new()
            .prefix("temp_test_file_")
            .suffix(".md")
            .tempfile()
            .unwrap();

        // Write the content to the temporary file
        tmpfile.write_all(content.as_bytes()).unwrap();

        tmpfile
    }

    #[test]
    fn read_markdown_file_reads_content_correctly() {
        let tmpfile = create_temp_file("# Header 1\n## Header 2\n");
        let file_path = tmpfile.path().to_str().unwrap().to_string();

        // Reading the file using the provided function
        let result = read_markdown_file(&file_path).unwrap();

        assert_eq!(result, "# Header 1\n## Header 2\n");

        // The NamedTempFile is dropped here, which deletes the file
    }

    #[test]
    fn read_markdown_file_returns_error_for_nonexistent_file() {
        let result = read_markdown_file("nonexistent_file.md");
        assert!(result.is_err());
    }

    #[test]
    fn read_markdown_file_handles_empty_file() {
        let tmpfile = create_temp_file("");
        let file_path = tmpfile.path().to_str().unwrap().to_string();
        let result = read_markdown_file(&file_path).unwrap();
        assert_eq!(result, "");
    }
}
