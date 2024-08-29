// Remove any existing TOC marked by comments
pub fn remove_existing_toc(content: &str) -> (String, usize) {
    let mut new_content = String::new();
    let mut in_toc_section = false;
    let mut inside_code_block = false;
    let mut toc_start_index = None;

    for (i, line) in content.lines().enumerate() {
        if line.trim_start().starts_with("```") {
            inside_code_block = !inside_code_block;
        }

        if !inside_code_block {
            if line.contains("<!-- START OF TOC !DO NOT EDIT THIS CONTENT MANUALLY-->") {
                in_toc_section = true;
                if toc_start_index.is_none() {
                    toc_start_index = Some(i);
                }
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

    (new_content, toc_start_index.unwrap_or(0))
}
