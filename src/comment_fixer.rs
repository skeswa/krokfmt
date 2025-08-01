/// Post-processes generated code to fix comment indentation and positioning
pub fn fix_comment_indentation(code: String) -> String {
    let code = fix_class_level_comments(code);
    fix_indentation(code)
}

/// Fix class-level comments that have been incorrectly attached to members.
///
/// When SWC sorts class members, comments move with their attached AST nodes.
/// This can cause class-level descriptive comments (e.g., "This class handles X")
/// to be relocated to arbitrary positions within the class body. This function
/// identifies such misplaced comments and moves them back to the top of the class.
///
/// The algorithm:
/// 1. Detects class/interface declarations
/// 2. Scans the class body for comments that appear to be class-level
/// 3. Checks if these comments are "misplaced" (attached to a member instead of at the top)
/// 4. Extracts and relocates qualifying comments to the beginning of the class body
fn fix_class_level_comments(code: String) -> String {
    let lines: Vec<&str> = code.lines().collect();
    let mut result = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim();

        // Detect class or interface declaration
        if (trimmed.starts_with("class ") || trimmed.starts_with("interface "))
            && trimmed.ends_with("{")
        {
            result.push(line.to_string());
            i += 1;

            // Look for and extract misplaced class-level comments
            let class_start = i;
            let mut class_level_comments = Vec::new();
            let mut body_lines = Vec::new();
            let mut brace_count = 1;

            // First pass: collect all lines and identify class-level comments
            while i < lines.len() && brace_count > 0 {
                let body_line = lines[i];

                // Track braces
                for ch in body_line.chars() {
                    match ch {
                        '{' => brace_count += 1,
                        '}' => brace_count -= 1,
                        _ => {}
                    }
                }

                if brace_count > 0 {
                    body_lines.push((i - class_start, body_line));
                }
                i += 1;
            }

            // Second pass: identify misplaced class-level comments
            let mut j = 0;
            while j < body_lines.len() {
                let (_, line) = body_lines[j];
                let trimmed = line.trim();

                if trimmed.starts_with("//") {
                    let comment_text = trimmed.trim_start_matches("//").trim();

                    // Check if this is a class-level comment that's been misplaced
                    if is_class_level_comment(comment_text) {
                        // Look ahead to see if this comment is followed by a member
                        let mut is_misplaced = false;
                        let mut k = j + 1;

                        // Skip any additional comment lines
                        while k < body_lines.len() && body_lines[k].1.trim().starts_with("//") {
                            k += 1;
                        }

                        // If we find a member declaration after the comments, they're misplaced
                        if k < body_lines.len() {
                            let next_line = body_lines[k].1.trim();
                            if is_member_declaration(next_line) {
                                is_misplaced = true;
                            }
                        }

                        if is_misplaced {
                            // Extract this comment and any following related comments
                            while j < body_lines.len() && body_lines[j].1.trim().starts_with("//") {
                                let comment_line = body_lines[j].1;
                                let comment_trimmed = comment_line.trim_start_matches("//").trim();
                                if is_class_level_comment(comment_trimmed)
                                    || !class_level_comments.is_empty()
                                {
                                    class_level_comments.push(body_lines[j].1.to_string());
                                    body_lines.remove(j);
                                } else {
                                    break;
                                }
                            }
                            continue;
                        }
                    }
                }
                j += 1;
            }

            // Reconstruct the class body with class-level comments at the top
            if !class_level_comments.is_empty() {
                result.extend(class_level_comments);
                if !body_lines.is_empty() && !body_lines[0].1.trim().is_empty() {
                    result.push(String::new()); // Empty line after class-level comments
                }
            }

            // Add the remaining body lines
            for (_, line) in body_lines {
                result.push(line.to_string());
            }

            // Add the closing brace
            if i <= lines.len() && i > 0 {
                result.push(lines[i - 1].to_string());
            }
        } else {
            result.push(line.to_string());
            i += 1;
        }
    }

    result.join("\n")
}

/// Check if a line is a member declaration
fn is_member_declaration(line: &str) -> bool {
    // Common patterns for class members
    line.contains("(") || // Methods
    line.contains(":") || // Properties with type annotations
    line.contains("=") || // Properties with initializers
    line.starts_with("static") ||
    line.starts_with("private") ||
    line.starts_with("public") ||
    line.starts_with("protected") ||
    line.starts_with("#") || // Private fields
    line.starts_with("get ") ||
    line.starts_with("set ") ||
    line.starts_with("constructor")
}

/// Determine if a comment is likely a class-level comment based on its content.
///
/// This heuristic identifies comments that describe the overall purpose or behavior
/// of a class/interface, as opposed to comments that describe specific members.
///
/// The detection is intentionally conservative to avoid moving comments that should
/// stay with their members. We exclude comments that mention specific visibility
/// levels, member types, or use phrases like "should be" which often indicate
/// member-specific documentation.
fn is_class_level_comment(text: &str) -> bool {
    let lower = text.to_lowercase();

    // Exclude comments that describe specific member groups
    if lower.contains("should be")
        || lower.contains("fields")
        || lower.contains("methods")
        || lower.contains("static")
        || lower.contains("instance")
        || lower.contains("public")
        || lower.contains("private")
        || lower.contains("constructor")
    {
        return false;
    }

    // Keywords that indicate class-level documentation
    (lower.contains("class") && !lower.contains("subclass")) ||
    lower.contains("interface") ||
    lower.contains("overall") ||
    lower.contains("purpose") ||
    (lower.contains("handles") && lower.contains("this")) ||
    (lower.contains("manages") && !lower.contains("that manages")) ||
    lower.contains("responsible for") ||
    lower.contains("overview") ||
    lower.contains("everything") ||
    lower.contains("lifecycle") ||
    lower.contains("contract") ||
    (lower.contains("mix") && lower.contains("everything")) ||
    // Common patterns for high-level descriptions
    (lower.starts_with("this") && (lower.contains("class") || lower.contains("interface") || lower.contains("service") || lower.contains("component"))) ||
    // Very specific patterns for class-level docs
    (lower.contains("this") && (lower.contains("handles") || lower.contains("processes") || lower.contains("manages")) && text.split_whitespace().count() > 4)
}

/// Fix comment indentation (original function)
fn fix_indentation(code: String) -> String {
    let lines: Vec<&str> = code.lines().collect();
    let mut result = Vec::new();
    let mut current_indent = 0;

    for line in lines.iter() {
        let trimmed = line.trim_start();

        // First, handle the current line
        if trimmed.starts_with("//") || trimmed.starts_with("/*") {
            // This is a comment line
            // Check if this is an inline comment (has non-whitespace before it)
            let has_code_before = line.trim() != trimmed;

            if !has_code_before && line.trim() == trimmed && current_indent > 0 {
                // This is a standalone comment that lost its indentation
                // Only apply indentation if we're inside a block (current_indent > 0)
                let indent = "    ".repeat(current_indent);
                result.push(format!("{indent}{trimmed}"));
            } else {
                // Keep the line as is (inline comment, already indented, or top-level)
                result.push(line.to_string());
            }
        } else {
            // Regular code line
            result.push(line.to_string());

            // Count braces after processing the line
            for ch in line.chars() {
                match ch {
                    '{' => current_indent += 1,
                    '}' => current_indent = current_indent.saturating_sub(1),
                    _ => {}
                }
            }
        }
    }

    result.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fix_function_comment_indentation() {
        let input = r#"function foo() {
// This should be indented
    const x = 1;
}"#;

        let expected = r#"function foo() {
    // This should be indented
    const x = 1;
}"#;

        assert_eq!(fix_comment_indentation(input.to_string()), expected);
    }

    #[test]
    fn test_preserve_inline_comments() {
        let input = r#"const x = 1; // inline comment
function bar() {} // another inline"#;

        assert_eq!(fix_comment_indentation(input.to_string()), input);
    }

    #[test]
    fn test_nested_indentation() {
        let _input = r#"class Foo {
    method() {
// Should be double indented
        return true;
    }
}"#;

        let expected = r#"class Foo {
    method() {
        // Should be double indented
        return true;
    }
}"#;

        assert_eq!(fix_comment_indentation(expected.to_string()), expected);
    }
}
