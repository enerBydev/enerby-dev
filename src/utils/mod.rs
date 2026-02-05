//! Utils Module - Helper functions
//! Common utilities for formatting, parsing, etc.

pub mod github_api;
pub mod github_cache;
pub mod github_stats;
pub mod markdown_loader;
pub mod project_images;

// Re-export project_images public API
pub use project_images::{get_project_image_url, has_real_image, ImageSource};

// Re-export github_stats public API
pub use github_stats::{format_loc, get_github_stats, GitHubStats};

// Re-export github_api public API
pub use github_api::{
    ApiError, GitHubApiConfig, GitHubRepoInfo, GITHUB_API_BASE_URL, USER_AGENT,
    get_repo_info, get_repo_homepage, get_all_repos, parse_github_url, get_homepage_from_url,
};

// Re-export github_cache public API
pub use github_cache::{
    CacheEntry, GitHubCache, DEFAULT_TTL_SECONDS,
    global_cache, get_cached_repo, set_cached_repo,
};

use pulldown_cmark::{html, Parser};

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

/// Calculate reading time from content (alias for calculate_reading_time)
pub fn calculate_read_time(content: &str) -> u32 {
    calculate_reading_time(content)
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

/// Truncate string (alias for truncate)
pub fn truncate_string(text: &str, max_len: usize) -> String {
    truncate(text, max_len)
}

/// Check if link is external (starts with http/https)
pub fn is_external_link(url: &str) -> bool {
    url.starts_with("http://") || url.starts_with("https://")
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

    #[test]
    fn test_is_external_link() {
        assert!(is_external_link("https://example.com"));
        assert!(!is_external_link("/about"));
    }
}
