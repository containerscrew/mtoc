// Remove any existing TOC marked by comments
pub fn remove_existing_toc(content: &str) -> String {
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
