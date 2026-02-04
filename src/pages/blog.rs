//! Blog Page - Blog listing and post detail

use dioxus::prelude::*;

/// Blog listing page
#[component]
pub fn BlogPage() -> Element {
    rsx! {
        section { class: "section",
            div { class: "container",
                h1 { class: "section-title", "Blog" }
                
                div { class: "blog-grid",
                    // Blog post cards will be loaded from content
                    div { class: "card",
                        div { class: "badge", "Rust" }
                        h3 { "Blog Post Title" }
                        p { class: "text-muted", "Jan 1, 2026 • 5 min read" }
                        p { class: "text-secondary", "Blog post excerpt..." }
                    }
                }
            }
        }
    }
}

/// Blog post detail page
#[component]
pub fn BlogPostPage(slug: String) -> Element {
    rsx! {
        article { class: "section",
            div { class: "container container-narrow",
                header {
                    h1 { class: "section-title", "Blog Post: {slug}" }
                    p { class: "text-muted", "Published on Jan 1, 2026" }
                }
                
                div { class: "card blog-content",
                    p { "Blog post content for: {slug}" }
                }
            }
        }
    }
}
