// Generate a table of contents from the headers
pub fn generate_toc(headers: Vec<(usize, String)>) -> String {
    let mut toc = String::new();
    toc.push_str("<!-- START OF TOC !DO NOT EDIT THIS CONTENT MANUALLY-->\n");
    toc.push_str(
        "**Table of Contents**  *generated with [mtoc](https://github.com/containerscrew/mtoc)*\n",
    );
    for (level, header) in headers {
        let indent = "  ".repeat(level - 1);
        let anchor = header.to_lowercase().replace(" ", "-");
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
