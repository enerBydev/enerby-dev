//! Blog Section and Post Components
//! Blog post listing and preview cards

use dioxus::prelude::*;
use crate::routes::Route;
use crate::components::molecules::{Card, SectionTitle};
use crate::components::atoms::{Button, ButtonVariant, Badge};
use crate::components::layout_components::{Container, Section, Grid};

/// Post Status (P10-A4)
#[derive(Clone, PartialEq, Copy)]
#[derive(Debug)]
pub enum PostStatus {
    Published,
    Draft,
}

/// Blog Post data structure (P10-A1, P10-A2)
#[derive(Clone, PartialEq)]
pub struct BlogPost {
    pub slug: &'static str,
    pub title: &'static str,
    pub excerpt: &'static str,
    pub content: &'static str,
    pub date: &'static str,
    pub read_time: u8,
    pub tags: Vec<&'static str>,
    pub status: PostStatus,
}

/// Get all blog posts (P10-B1 - hardcoded, can be moved to markdown files later)
pub fn get_blog_posts() -> Vec<BlogPost> {
    vec![
        BlogPost {
            slug: "why-i-chose-rust",
            title: "Why I Chose Rust for Web Development",
            excerpt: "After years of working with JavaScript and Python, I discovered Rust and it changed everything. Here's my journey and why you might want to consider it too.",
            content: "Full content would be loaded from markdown...",
            date: "2024-01-15",
            read_time: 8,
            tags: vec!["Rust", "WebAssembly", "Web Development"],
            status: PostStatus::Published,
        },
        BlogPost {
            slug: "building-with-dioxus",
            title: "Building a Portfolio with Dioxus",
            excerpt: "A deep dive into building a full-featured portfolio website using Dioxus, Rust's React-like framework. Tips, tricks, and lessons learned.",
            content: "Full content would be loaded from markdown...",
            date: "2024-01-20",
            read_time: 12,
            tags: vec!["Dioxus", "Rust", "Frontend"],
            status: PostStatus::Published,
        },
        BlogPost {
            slug: "cyberpunk-design-system",
            title: "Creating a Cyberpunk Design System",
            excerpt: "How I built a consistent cyberpunk-themed design system with neon colors, glassmorphism, and animated effects using Tailwind CSS.",
            content: "Full content would be loaded from markdown...",
            date: "2024-02-01",
            read_time: 6,
            tags: vec!["CSS", "Design", "Tailwind"],
            status: PostStatus::Published,
        },
        BlogPost {
            slug: "async-rust-patterns",
            title: "Async Rust Patterns for the Web",
            excerpt: "Understanding async/await in Rust and how to apply these patterns effectively in web applications.",
            content: "Full content would be loaded from markdown...",
            date: "2024-02-10",
            read_time: 15,
            tags: vec!["Rust", "Async", "Performance"],
            status: PostStatus::Draft,
        },
    ]
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

/// Blog Section for Home Page (P10-C1)
#[component]
pub fn BlogSection() -> Element {
    let posts = get_published_posts();
    let recent_posts: Vec<_> = posts.into_iter().take(3).collect();

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

/// Blog Post Preview Card (P10-C2)
#[component]
pub fn BlogPostPreview(post: BlogPost) -> Element {
    rsx! {
        Card { hover_effect: true, class: "h-full flex flex-col".to_string(),
            // Date and read time
            div { class: "flex items-center justify-between text-xs text-muted mb-3",
                span { "{post.date}" }
                span { "{post.read_time} min read" }
            }
            
            // Title
            h3 { class: "text-lg font-bold text-white mb-2 group-hover:text-primary transition-colors line-clamp-2",
                "{post.title}"
            }
            
            // Excerpt
            p { class: "text-muted text-sm mb-4 line-clamp-3 flex-grow",
                "{post.excerpt}"
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
                        "{post.title}"
                    }
                    p { class: "text-muted text-sm line-clamp-1", "{post.excerpt}" }
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
