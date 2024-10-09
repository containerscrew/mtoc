use regex::Regex;

// Generate a table of contents from the headers
pub fn generate_toc(headers: Vec<(usize, String)>) -> String {
    let mut toc = String::new();
    toc.push_str("<!-- START OF TOC !DO NOT EDIT THIS CONTENT MANUALLY-->\n");
    toc.push_str(
        "**Table of Contents**  *generated with [mtoc](https://github.com/containerscrew/mtoc)*\n",
    );

    // Regex to match and remove or replace unwanted characters
    let re = Regex::new(r"[^a-zA-Z0-9-_ ]").unwrap();

    for (level, header) in headers {
        let indent = "  ".repeat(level - 1);
        let sanitized_header = re.replace_all(&header.to_lowercase(), "").to_string();
        let anchor = sanitized_header.replace(" ", "-");
        toc.push_str(&format!("{}- [{}](#{})\n", indent, header, anchor));
    }

    toc.push_str("<!-- END OF TOC -->");
    toc
}

// Insert the TOC at the beginning of the file content
pub fn insert_toc(original_content: &str, toc: &str, toc_start_index: usize) -> String {
    let mut lines: Vec<&str> = original_content.lines().collect();

    // Insert the new TOC at the specific index
    lines.insert(toc_start_index, toc);

    // Join lines and ensure there's a newline at the end
    let mut updated_content = lines.join("\n");

    // Add a newline at the end if it's not already there
    if !updated_content.ends_with('\n') {
        updated_content.push('\n');
    }

    updated_content
}
