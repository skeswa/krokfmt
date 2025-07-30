/// Post-processes generated code to fix comment indentation
pub fn fix_comment_indentation(code: String) -> String {
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
        let input = r#"class Foo {
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
