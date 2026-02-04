//! Projects Page - Portfolio showcase

use dioxus::prelude::*;

/// Projects listing page
#[component]
pub fn ProjectsPage() -> Element {
    rsx! {
        section { class: "section",
            div { class: "container",
                h1 { class: "section-title", "Projects" }
                
                div { class: "projects-grid",
                    // Project cards will be loaded from data
                    div { class: "card",
                        h3 { "Project 1" }
                        p { class: "text-secondary", "Project description placeholder" }
                        div { class: "badge", "Rust" }
                    }
                }
            }
        }
    }
}

/// Project detail page
#[component]
pub fn ProjectDetailPage(slug: String) -> Element {
    rsx! {
        section { class: "section",
            div { class: "container",
                h1 { class: "section-title", "Project: {slug}" }
                
                div { class: "card",
                    p { class: "text-secondary",
                        "Detailed project information for: {slug}"
                    }
                }
            }
        }
    }
}
