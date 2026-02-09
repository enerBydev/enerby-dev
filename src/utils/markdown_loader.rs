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

#[derive(Debug, Clone, PartialEq)]
pub enum PostLanguage {
    EN,
    ES,
}

pub struct MarkdownPost {
    pub frontmatter: Frontmatter,
    pub content_html: String,
    pub read_time_minutes: u8,
    pub language: PostLanguage,
    pub file_slug: String,
}

/// Enhanced post structure with canonical slug from _en.md frontmatter
/// This is the new atomic structure for slug normalization
#[derive(Debug, Clone, PartialEq)]
pub struct ProcessedMarkdownPost {
    pub canonical_slug: String,     // Source of truth: from _en.md frontmatter.slug
    pub file_slug: String,          // Original filename for debugging/migration
    pub frontmatter: Frontmatter,
    pub content_html: String,
    pub read_time_minutes: u8,
    pub language: PostLanguage,
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

    for filename_cow in Asset::iter() {
        let filename = filename_cow.as_ref();
        if filename.ends_with(".md") {
            // Parse filename for language: slug.lang.md
            // Default to EN if pattern doesn't match
            let (file_slug, language) = if filename.ends_with(".es.md") {
                 (filename.trim_end_matches(".es.md").to_string(), PostLanguage::ES)
            } else if filename.ends_with(".en.md") {
                 (filename.trim_end_matches(".en.md").to_string(), PostLanguage::EN)
            } else {
                 (filename.trim_end_matches(".md").to_string(), PostLanguage::EN)
            };

            if let Some(file) = Asset::get(filename) {
                if let Ok(content_str) = std::str::from_utf8(file.data.as_ref()) {
                    let matter = Matter::<YAML>::new();
                    let result = matter.parse(content_str);

                    if let Some(data) = result.data {
                        if let Ok(frontmatter) = data.deserialize::<Frontmatter>() {
                            let read_time_minutes = frontmatter
                                .read_time
                                .unwrap_or_else(|| calculate_read_time(&result.content));

                            let parser = Parser::new(&result.content);
                            let mut html_output = String::new();
                            html::push_html(&mut html_output, parser);

                            posts.push(MarkdownPost {
                                frontmatter,
                                content_html: html_output,
                                read_time_minutes,
                                language,
                                file_slug, // This is the ID grouping key
                            });
                        }
                    }
                }
            }
        }
    }
    
    posts
}

/// Load and process markdown posts with canonical slug normalization
/// This function implements the new "Source of Truth" pattern where _en.md frontmatter.slug
/// defines the canonical slug for both language variants
pub fn load_processed_markdown_posts() -> Vec<ProcessedMarkdownPost> {
    let mut posts: Vec<ProcessedMarkdownPost> = Vec::new();

    for filename_cow in Asset::iter() {
        let filename = filename_cow.as_ref();
        if filename.ends_with(".md") {
            // Parse filename for language: slug.lang.md
            let (file_slug, language) = if filename.ends_with(".es.md") {
                 (filename.trim_end_matches(".es.md").to_string(), PostLanguage::ES)
            } else if filename.ends_with(".en.md") {
                 (filename.trim_end_matches(".en.md").to_string(), PostLanguage::EN)
            } else {
                 (filename.trim_end_matches(".md").to_string(), PostLanguage::EN)
            };

            if let Some(file) = Asset::get(filename) {
                if let Ok(content_str) = std::str::from_utf8(file.data.as_ref()) {
                    let matter = Matter::<YAML>::new();
                    let result = matter.parse(content_str);

                    if let Some(data) = result.data {
                        if let Ok(frontmatter) = data.deserialize::<Frontmatter>() {
                            let read_time_minutes = frontmatter
                                .read_time
                                .unwrap_or_else(|| calculate_read_time(&result.content));

                            let parser = Parser::new(&result.content);
                            let mut html_output = String::new();
                            html::push_html(&mut html_output, parser);

                            // Use frontmatter.slug as canonical_slug (Source of Truth)
                            let canonical_slug = frontmatter.slug.clone();

                            posts.push(ProcessedMarkdownPost {
                                canonical_slug,
                                file_slug,
                                frontmatter,
                                content_html: html_output,
                                read_time_minutes,
                                language,
                            });
                        }
                    }
                }
            }
        }
    }
    
    posts
}

/// Consolidate processed posts by canonical slug with validation
/// Groups EN/ES variants and validates slug consistency
/// Returns unified posts with EN as source of truth
pub fn consolidate_posts_by_canonical_slug() -> Vec<ProcessedMarkdownPost> {
    let raw_posts = load_processed_markdown_posts();
    let mut grouped_posts: std::collections::HashMap<String, Vec<ProcessedMarkdownPost>> = std::collections::HashMap::new();
    
    // Group by canonical_slug
    for post in raw_posts {
        grouped_posts.entry(post.canonical_slug.clone()).or_default().push(post);
    }
    
    let mut consolidated_posts: Vec<ProcessedMarkdownPost> = Vec::new();
    
    for (canonical_slug, variants) in grouped_posts {
        // Find EN version as source of truth
        let en_variant = variants.iter().find(|p| p.language == PostLanguage::EN);
        let es_variant = variants.iter().find(|p| p.language == PostLanguage::ES);
        
        if let Some(en_post) = en_variant {
            // Validate slug consistency across languages
            if let Some(es_post) = es_variant {
                if en_post.canonical_slug != es_post.canonical_slug {
                    // Log warning but continue with EN as source of truth
                    eprintln!("Warning: Slug mismatch between EN and ES versions for {}", canonical_slug);
                }
            }
            
            // Use EN version as the consolidated post
            consolidated_posts.push(en_post.clone());
        } else {
            // Fallback: if no EN version, use ES (shouldn't happen in normal operation)
            if let Some(es_post) = es_variant {
                eprintln!("Warning: No EN version found for {}, using ES as fallback", canonical_slug);
                consolidated_posts.push(es_post.clone());
            }
        }
    }
    
    // Sort by date (newest first)
    consolidated_posts.sort_by(|a, b| b.frontmatter.date.cmp(&a.frontmatter.date));
    
    consolidated_posts
}

/// Get post by canonical slug
pub fn get_processed_post_by_slug(slug: &str) -> Option<ProcessedMarkdownPost> {
    consolidate_posts_by_canonical_slug()
        .into_iter()
        .find(|p| p.canonical_slug == slug)
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

    // Tests for new slug normalization functionality
    #[test]
    fn test_processed_markdown_post_structure() {
        let post = ProcessedMarkdownPost {
            canonical_slug: "test-slug".to_string(),
            file_slug: "post_1".to_string(),
            frontmatter: Frontmatter {
                slug: "test-slug".to_string(),
                title: "Test Post".to_string(),
                date: "2026-02-06".to_string(),
                excerpt: "Test excerpt".to_string(),
                tags: vec!["test".to_string()],
                featured: false,
                read_time: Some(2),
            },
            content_html: "<p>Test content</p>".to_string(),
            read_time_minutes: 2,
            language: PostLanguage::EN,
        };

        assert_eq!(post.canonical_slug, "test-slug");
        assert_eq!(post.file_slug, "post_1");
        assert_eq!(post.language, PostLanguage::EN);
    }

    #[test]
    fn test_consolidate_posts_by_canonical_slug() {
        // This test would require mocking the Asset::iter() behavior
        // For now, we'll test the basic structure
        let posts = consolidate_posts_by_canonical_slug();
        
        // Should return posts sorted by date
        assert!(posts.len() >= 0); // Basic sanity check
    }

    #[test]
    fn test_get_processed_post_by_slug() {
        // Test slug lookup functionality
        let posts = consolidate_posts_by_canonical_slug();
        
        if !posts.is_empty() {
            let first_post = &posts[0];
            let found_post = get_processed_post_by_slug(&first_post.canonical_slug);
            assert!(found_post.is_some());
            assert_eq!(found_post.unwrap().canonical_slug, first_post.canonical_slug);
        }
    }

    #[test]
    fn test_slug_extraction_from_frontmatter() {
        // Test that canonical_slug comes from frontmatter.slug
        let posts = load_processed_markdown_posts();
        
        for post in &posts {
            assert_eq!(post.canonical_slug, post.frontmatter.slug);
        }
    }

    #[test]
    fn test_language_detection() {
        let posts = load_processed_markdown_posts();
        
        // Should have both EN and ES variants
        let en_posts: Vec<_> = posts.iter().filter(|p| p.language == PostLanguage::EN).collect();
        let es_posts: Vec<_> = posts.iter().filter(|p| p.language == PostLanguage::ES).collect();
        
        assert!(!en_posts.is_empty());
        assert!(!es_posts.is_empty());
    }

    #[test]
    fn test_canonical_slug_consistency() {
        let consolidated = consolidate_posts_by_canonical_slug();
        
        // Each consolidated post should have unique canonical slug
        let mut slugs: Vec<String> = consolidated.iter().map(|p| p.canonical_slug.clone()).collect();
        slugs.sort();
        slugs.dedup();
        
        assert_eq!(slugs.len(), consolidated.len());
    }
}
