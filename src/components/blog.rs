//! Blog Section and Post Components
//! Blog post listing and preview cards

use crate::components::atoms::{Badge, Button, ButtonVariant};
use crate::components::layout_components::{Container, Grid, Section};
use crate::components::molecules::{Card, SectionTitle};
use crate::routes::Route;
use dioxus::prelude::*;

use crate::utils::markdown_loader::{load_markdown_posts, PostLanguage};
use crate::i18n::Language;
use std::collections::HashMap;

/// Post Status (P10-A4)
#[derive(Clone, PartialEq, Copy, Debug)]
pub enum PostStatus {
    Published,
    Draft,
}

/// Blog Post data structure (P10-A1, P10-A2)
/// Updated for i18n (F18-D)
#[derive(Clone, PartialEq, Debug)]
pub struct BlogPost {
    pub slug: String,
    
    // Internal localized fields
    title_en: String,
    title_es: String,
    
    excerpt_en: String,
    excerpt_es: String,
    
    content_en: String,
    content_es: String,
    
    pub date: String,
    pub read_time: u8,
    pub tags: Vec<String>,
    pub status: PostStatus,
    pub featured: bool,
}

impl BlogPost {
    pub fn title(&self, lang: &Language) -> &str {
        match lang {
            Language::EN => &self.title_en,
            Language::ES => &self.title_es,
        }
    }
    
    pub fn excerpt(&self, lang: &Language) -> &str {
        match lang {
            Language::EN => &self.excerpt_en,
            Language::ES => &self.excerpt_es,
        }
    }
    
    pub fn content(&self, lang: &Language) -> &str {
        match lang {
            Language::EN => &self.content_en,
            Language::ES => &self.content_es,
        }
    }
}

/// Get all blog posts - Dynamic from Markdown (F8)
/// Consolidated by slug across languages
pub fn get_blog_posts() -> Vec<BlogPost> {
    let raw_posts = load_markdown_posts();
    let mut grouped_posts: HashMap<String, Vec<crate::utils::markdown_loader::MarkdownPost>> = HashMap::new();
    
    // Group by file_slug
    for post in raw_posts {
        grouped_posts.entry(post.file_slug.clone()).or_default().push(post);
    }
    
    let mut posts: Vec<BlogPost> = grouped_posts
        .into_iter()
        .filter_map(|(slug, variants)| {
            // We need at least an EN version to form a valid post (Source of Truth)
            // Or just any version? Let's assume EN is primary.
            let en_variant = variants.iter().find(|p| p.language == PostLanguage::EN);
            let es_variant = variants.iter().find(|p| p.language == PostLanguage::ES);
            
            if let Some(base) = en_variant {
                let tags = base.frontmatter.tags.clone(); // Shared tags?? Or localized tags? Assuming shared for now.
                
                // ES Fallback: Use ES if present, else clone EN
                let (title_es, excerpt_es, content_es) = if let Some(es) = es_variant {
                    (es.frontmatter.title.clone(), es.frontmatter.excerpt.clone(), es.content_html.clone())
                } else {
                    (base.frontmatter.title.clone(), base.frontmatter.excerpt.clone(), base.content_html.clone())
                };

                Some(BlogPost {
                    slug: slug, // Use the file slug as the ID
                    title_en: base.frontmatter.title.clone(),
                    title_es,
                    excerpt_en: base.frontmatter.excerpt.clone(),
                    excerpt_es,
                    content_en: base.content_html.clone(),
                    content_es,
                    date: base.frontmatter.date.clone(),
                    read_time: base.read_time_minutes,
                    tags,
                    status: PostStatus::Published,
                    featured: base.frontmatter.featured,
                })
            } else {
                // Warning: Found ES post without EN base? Skip for now.
                None
            }
        })
        .collect();
        
    // Sort by date
    posts.sort_by(|a, b| b.date.cmp(&a.date));
    
    posts
}

/// Get published posts only
pub fn get_published_posts() -> Vec<BlogPost> {
    get_blog_posts()
        .into_iter()
        .filter(|p| p.status == PostStatus::Published)
        .collect()
}

/// Get post by slug
pub fn get_post_by_slug(slug: &str) -> Option<BlogPost> {
    get_blog_posts().into_iter().find(|p| p.slug == slug)
}

use crate::i18n::I18N_CONFIG;

/// Blog Section for Home Page (P10-C1)
#[component]
pub fn BlogSection() -> Element {
    let posts = get_published_posts();
    let recent_posts: Vec<_> = posts.into_iter().filter(|p| p.featured).take(3).collect();

    rsx! {
        Section { id: "blog",
            Container {
                SectionTitle {
                    text: "Latest Posts".to_string(),
                    subtitle: "From the Blog".to_string(),
                    center: true
                }

                // Post Grid (P10-C3)
                Grid { cols: 1, md_cols: 3, gap: 6,
                    for post in recent_posts.iter() {
                        BlogPostPreview { post: post.clone() }
                    }
                }

                // View All Button
                div { class: "text-center mt-12",
                    Button {
                        variant: ButtonVariant::Ghost,
                        to: Route::BlogPage {},
                        "View All Posts →"
                    }
                }
            }
        }
    }
}

/// Blog Post Preview Card (P10-C2) - Clickeable
#[component]
pub fn BlogPostPreview(post: BlogPost) -> Element {
    let slug = post.slug.to_string();
    let lang = I18N_CONFIG.read().language;

    rsx! {
        Link {
            to: Route::BlogPostPage { slug },
            class: "block h-full",

            Card { hover_effect: true, class: "h-full flex flex-col cursor-pointer".to_string(),
                // Date and read time
                div { class: "flex items-center justify-between text-xs text-muted mb-3",
                    span { "{post.date}" }
                    span { "{post.read_time} min read" }
                }

                // Title
                h3 { class: "text-lg font-bold text-white mb-2 group-hover:text-primary transition-colors line-clamp-2",
                    "{post.title(&lang)}"
                }

                // Excerpt
                p { class: "text-muted text-sm mb-4 line-clamp-3 flex-grow",
                    "{post.excerpt(&lang)}"
                }

                // Tags (P10-C4)
                div { class: "flex flex-wrap gap-2 mt-auto",
                    for tag in post.tags.iter().take(2) {
                        Badge { color: "purple".to_string(), "{tag}" }
                    }
                }
            }
        }
    }
}

/// Full Blog Post List Page Component
#[component]
pub fn BlogListSection() -> Element {
    let posts = get_published_posts();

    rsx! {
        Section { id: "blog-list",
            Container {
                SectionTitle {
                    text: "All Posts".to_string(),
                    subtitle: "Blog Archive".to_string()
                }

                div { class: "space-y-6",
                    for post in posts.iter() {
                        BlogPostRow { post: post.clone() }
                    }
                }
            }
        }
    }
}

/// Blog Post Row (for list view)
#[component]
fn BlogPostRow(post: BlogPost) -> Element {
    let lang = I18N_CONFIG.read().language;
    rsx! {
        Card { hover_effect: true,
            div { class: "flex flex-col md:flex-row md:items-center gap-4",
                // Date column
                div { class: "md:w-32 flex-shrink-0",
                    span { class: "text-sm text-primary font-mono", "{post.date}" }
                }

                // Content
                div { class: "flex-grow",
                    h3 { class: "text-lg font-bold text-white mb-1 group-hover:text-primary transition-colors",
                        "{post.title(&lang)}"
                    }
                    p { class: "text-muted text-sm line-clamp-1", "{post.excerpt(&lang)}" }
                }

                // Meta
                div { class: "flex items-center gap-4 text-xs text-muted",
                    span { "{post.read_time} min" }
                    div { class: "flex gap-1",
                        for tag in post.tags.iter().take(2) {
                            Badge { "{tag}" }
                        }
                    }
                }
            }
        }
    }
}
