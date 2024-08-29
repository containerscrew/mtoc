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
pub fn insert_toc(content: &str, toc: &str, index: usize) -> String {
    let mut lines: Vec<&str> = content.lines().collect();
    lines.insert(index, toc);
    lines.join("\n")
}
