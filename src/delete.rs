// Remove any existing TOC marked by comments
pub fn remove_existing_toc(content: &str) -> String {
    let mut new_content = String::new();
    let mut in_toc_section = false;
    let mut inside_code_block = false;

    for line in content.lines() {
        if line.trim_start().starts_with("```") {
            inside_code_block = !inside_code_block;
        }

        if !inside_code_block {
            if line.contains("<!-- START OF TOC !DO NOT EDIT THIS CONTENT MANUALLY-->") {
                in_toc_section = true;
                continue;
            }
            if in_toc_section && line.contains("<!-- END OF TOC -->") {
                in_toc_section = false;
                continue;
            }
        }

        if !in_toc_section {
            new_content.push_str(&format!("{}\n", line));
        }
    }

    new_content
}
