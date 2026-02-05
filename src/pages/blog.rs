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

    rsx! {
        Section { id: "blog-post",
            Container {
                // Back Button
                div { class: "mb-8 flex justify-center",
                    Button {
                        variant: ButtonVariant::Ghost,
                        to: Route::BlogPage {},
                        "← Back to Blog"
                    }
                }

                if let Some(p) = post {
                    article { class: "max-w-3xl mx-auto",
                        // Header
                        header { class: "mb-8 text-center",
                            h1 { class: "text-3xl font-bold text-white mb-4", "{p.title(&lang)}" }
                            div { class: "flex items-center justify-center gap-4 text-muted",
                                span { "{p.date}" }
                                span { "•" }
                                span { "{p.read_time} min read" }
                            }
                        }

                        // Tags
                        div { class: "flex flex-wrap justify-center gap-2 mb-8",
                            for tag in p.tags.iter() {
                                span { class: "px-3 py-1 bg-primary/20 text-primary rounded-full text-sm", "{tag}" }
                            }
                        }

                        // Content
                        div { class: "prose prose-invert max-w-none bg-bg-card/50 rounded-xl p-8 border border-white/5",
                            div { dangerous_inner_html: "{p.content(&lang)}" }
                        }
                    }
                } else {
                    div { class: "text-center py-16",
                        h1 { class: "text-2xl font-bold text-white mb-4", "Post not found" }
                        p { class: "text-muted", "The post \"{slug}\" does not exist." }
                    }
                }
            }
        }
    }
}
