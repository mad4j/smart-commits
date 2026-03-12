#[derive(Debug, Default)]
pub struct FileChanges {
    pub filename: String,
    pub added: Vec<String>,
    pub removed: Vec<String>,
}

pub fn preprocess_diff(raw_diff: &str, max_lines: usize) -> String {
    let mut file_groups: Vec<FileChanges> = Vec::new();
    let mut current_file: Option<FileChanges> = None;
    let mut total_lines = 0;

    for line in raw_diff.lines() {
        if total_lines >= max_lines {
            break;
        }

        if line.starts_with("diff --git ") || line.starts_with("diff -r ") {
            if let Some(fc) = current_file.take() {
                file_groups.push(fc);
            }
            // Extract filename from diff header
            let filename = extract_filename_from_diff(line);
            current_file = Some(FileChanges {
                filename,
                added: Vec::new(),
                removed: Vec::new(),
            });
            continue;
        }

        // Skip metadata lines
        if line.starts_with("index ")
            || line.starts_with("old mode")
            || line.starts_with("new mode")
            || line.starts_with("new file mode")
            || line.starts_with("deleted file mode")
            || line.starts_with("similarity index")
            || line.starts_with("rename from")
            || line.starts_with("rename to")
            || line.starts_with("Binary files")
        {
            continue;
        }

        // Extract filename from --- and +++ lines
        if line.starts_with("--- ") || line.starts_with("+++ ") {
            let path = &line[4..];
            if path != "/dev/null" && !path.starts_with("a/") && !path.starts_with("b/") {
                // SVN-style diff, update filename
                if let Some(ref mut fc) = current_file {
                    if fc.filename.is_empty() {
                        fc.filename = path.trim().to_string();
                    }
                }
            } else if path.starts_with("b/") {
                // git-style +++ b/filename
                if line.starts_with("+++ ") {
                    if let Some(ref mut fc) = current_file {
                        if fc.filename.is_empty() {
                            fc.filename = path[2..].trim().to_string();
                        }
                    }
                }
            }
            continue;
        }

        // Skip hunk headers but keep track
        if line.starts_with("@@ ") {
            continue;
        }

        // Handle added/removed lines
        if line.starts_with('+') && !line.starts_with("+++") {
            if let Some(ref mut fc) = current_file {
                fc.added.push(line[1..].to_string());
                total_lines += 1;
            }
        } else if line.starts_with('-') && !line.starts_with("---") {
            if let Some(ref mut fc) = current_file {
                fc.removed.push(line[1..].to_string());
                total_lines += 1;
            }
        }
    }

    if let Some(fc) = current_file {
        file_groups.push(fc);
    }

    // Format output
    let mut output = String::new();
    for fc in &file_groups {
        if fc.added.is_empty() && fc.removed.is_empty() {
            continue;
        }
        output.push_str(&format!("File: {}\n", fc.filename));
        for line in &fc.removed {
            output.push_str(&format!("  - {}\n", line));
        }
        for line in &fc.added {
            output.push_str(&format!("  + {}\n", line));
        }
        output.push('\n');
    }

    if output.is_empty() {
        output = raw_diff.lines().take(max_lines).collect::<Vec<_>>().join("\n");
    }

    output
}

fn extract_filename_from_diff(line: &str) -> String {
    // "diff --git a/src/main.rs b/src/main.rs"
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() >= 4 {
        let b_path = parts[parts.len() - 1];
        if b_path.starts_with("b/") {
            return b_path[2..].to_string();
        }
        return b_path.to_string();
    }
    line.to_string()
}
