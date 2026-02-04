use gray_matter::{Matter, engine::YAML};
use pulldown_cmark::{Parser, html};
use rust_embed::RustEmbed;
use serde::Deserialize;

#[derive(RustEmbed)]
#[folder = "content/blog/"]
struct Asset;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Frontmatter {
    pub slug: String,
    pub title: String,
    pub date: String,
    pub excerpt: String,
    pub tags: Vec<String>,
    pub featured: bool,
    pub read_time: u8,
}

pub struct MarkdownPost {
    pub frontmatter: Frontmatter,
    pub content_html: String,
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
                            let parser = Parser::new(&result.content);
                            let mut html_output = String::new();
                            html::push_html(&mut html_output, parser);

                            posts.push(MarkdownPost {
                                frontmatter,
                                content_html: html_output,
                            });
                        } else {
                            println!(
                                "Failed to deserialize frontmatter for {}",
                                filename.as_ref()
                            );
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
