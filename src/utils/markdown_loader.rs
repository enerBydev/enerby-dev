use gray_matter::{Matter, engine::YAML};
use pulldown_cmark::{Parser, html};
use rust_embed::RustEmbed;
use serde::Deserialize;

#[derive(RustEmbed)]
#[folder = "content/blog/"]
struct Asset;

/// Frontmatter from markdown files
/// read_time is now optional - will be auto-calculated if not provided
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Frontmatter {
    pub slug: String,
    pub title: String,
    pub date: String,
    pub excerpt: String,
    pub tags: Vec<String>,
    pub featured: bool,
    #[serde(default)]
    pub read_time: Option<u8>,
}

pub struct MarkdownPost {
    pub frontmatter: Frontmatter,
    pub content_html: String,
    /// Calculated or provided read time in minutes
    pub read_time_minutes: u8,
}

/// Calculate read time based on word count
/// Uses industry standard of ~200 words per minute
fn calculate_read_time(content: &str) -> u8 {
    let word_count = content.split_whitespace().count();
    let minutes = (word_count as f32 / 200.0).ceil() as u8;
    // Minimum 1 minute
    minutes.max(1)
}

pub fn load_markdown_posts() -> Vec<MarkdownPost> {
    let mut posts: Vec<MarkdownPost> = Vec::new();

    for filename in Asset::iter() {
        if filename.as_ref().ends_with(".md") {
            if let Some(file) = Asset::get(filename.as_ref()) {
                if let Ok(content_str) = std::str::from_utf8(file.data.as_ref()) {
                    let matter = Matter::<YAML>::new();
                    let result = matter.parse(content_str);

                    if let Some(data) = result.data {
                        if let Ok(frontmatter) = data.deserialize::<Frontmatter>() {
                            // Calculate read time from content if not provided
                            let read_time_minutes = frontmatter.read_time
                                .unwrap_or_else(|| calculate_read_time(&result.content));

                            let parser = Parser::new(&result.content);
                            let mut html_output = String::new();
                            html::push_html(&mut html_output, parser);

                            posts.push(MarkdownPost {
                                frontmatter,
                                content_html: html_output,
                                read_time_minutes,
                            });
                        }
                    }
                }
            }
        }
    }

    // Sort by date (descending)
    posts.sort_by(|a, b| b.frontmatter.date.cmp(&a.frontmatter.date));

    posts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_read_time_short() {
        // 50 words = ~0.25 min, should round up to 1
        let content = "word ".repeat(50);
        assert_eq!(calculate_read_time(&content), 1);
    }

    #[test]
    fn test_calculate_read_time_medium() {
        // 400 words = 2 min
        let content = "word ".repeat(400);
        assert_eq!(calculate_read_time(&content), 2);
    }

    #[test]
    fn test_calculate_read_time_long() {
        // 1000 words = 5 min
        let content = "word ".repeat(1000);
        assert_eq!(calculate_read_time(&content), 5);
    }
}
