//! Utils Module - Helper functions
//! Common utilities for formatting, parsing, etc.

use pulldown_cmark::{Parser, html};

/// Convert markdown string to HTML
pub fn markdown_to_html(markdown: &str) -> String {
    let parser = Parser::new(markdown);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

/// Format date string for display
pub fn format_date(date_str: &str) -> String {
    // Simple date formatting - could be enhanced with chrono
    date_str.to_string()
}

/// Calculate reading time from content
pub fn calculate_reading_time(content: &str) -> u32 {
    let words = content.split_whitespace().count();
    let minutes = words / 200; // Average reading speed
    if minutes < 1 { 1 } else { minutes as u32 }
}

/// Generate slug from title
pub fn slugify(title: &str) -> String {
    title
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

/// Truncate text to specified length with ellipsis
pub fn truncate(text: &str, max_len: usize) -> String {
    if text.len() <= max_len {
        text.to_string()
    } else {
        format!("{}...", &text[..max_len.saturating_sub(3)])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slugify() {
        assert_eq!(slugify("Hello World"), "hello-world");
        assert_eq!(slugify("My First Blog Post!"), "my-first-blog-post");
    }

    #[test]
    fn test_reading_time() {
        let content = "word ".repeat(400);
        assert_eq!(calculate_reading_time(&content), 2);
    }

    #[test]
    fn test_truncate() {
        assert_eq!(truncate("Hello World", 20), "Hello World");
        assert_eq!(truncate("Hello World", 8), "Hello...");
    }

    #[test]
    fn test_markdown_to_html() {
        let md = "# Hello\n\nThis is **bold**.";
        let html = markdown_to_html(md);
        assert!(html.contains("<h1>Hello</h1>"));
        assert!(html.contains("<strong>bold</strong>"));
    }
}
