//! Blog Page - Blog listing and post detail

use crate::components::blog::{BlogPostPreview, get_post_by_slug, get_published_posts};
use crate::components::layout_components::{Container, Grid, Section};
use crate::components::molecules::SectionTitle;
use crate::routes::Route;
use dioxus::prelude::*;

use crate::components::atoms::{Button, ButtonVariant};

/// Blog listing page - All published posts
#[component]
pub fn BlogPage() -> Element {
    let posts = get_published_posts();
    let featured: Vec<_> = posts.iter().filter(|p| p.featured).cloned().collect();
    let regular: Vec<_> = posts.iter().filter(|p| !p.featured).cloned().collect();

    rsx! {
        Section { id: "blog-page",
            Container {
                SectionTitle {
                    text: "All Posts".to_string(),
                    subtitle: "Blog Archive".to_string(),
                    center: true
                }

                // Featured Posts
                if !featured.is_empty() {
                    div { class: "mb-12",
                        h3 { class: "text-lg font-bold text-primary mb-6", "⭐ Featured Posts" }
                        Grid { cols: 1, md_cols: 2, gap: 6,
                            for post in featured.iter() {
                                BlogPostPreview { post: post.clone() }
                            }
                        }
                    }
                }

                // All Posts
                if !regular.is_empty() {
                    div { class: "mb-12",
                        h3 { class: "text-lg font-bold text-purple-400 mb-6", "📝 More Posts" }
                        Grid { cols: 1, md_cols: 3, gap: 6,
                            for post in regular.iter() {
                                BlogPostPreview { post: post.clone() }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Blog post detail page
#[component]
pub fn BlogPostPage(slug: String) -> Element {
    let post = get_post_by_slug(&slug);
    let lang = crate::i18n::I18N_CONFIG.read().language;
    use crate::components::blog::{BlogPostHeader, BlogPostContent, BlogPostNavigator};

    rsx! {
        Section { id: "blog-post",
            Container {
                // Atomic Navigator (I20-B)
                BlogPostNavigator {}

                if let Some(p) = post {
                    article { class: "max-w-4xl mx-auto",
                        // Atomic Header (I20-B)
                        BlogPostHeader { post: p.clone(), lang: lang }

                        // Atomic Content (I20-B)
                        BlogPostContent { post: p.clone(), lang: lang }
                    }
                } else {
                    div { class: "text-center py-16",
                        h1 { class: "text-2xl font-bold text-white mb-4", "Post not found" }
                        p { class: "text-muted", "The post \"{slug}\" does not exist." }
                        
                        div { class: "mt-8",
                            crate::components::atoms::Button {
                                variant: crate::components::atoms::ButtonVariant::Primary,
                                to: Route::BlogPage {},
                                "Return to Archive"
                            }
                        }
                    }
                }
            }
        }
    }
}
