//! About Page - Personal info and timeline

use dioxus::prelude::*;

/// About page component
#[component]
pub fn AboutPage() -> Element {
    rsx! {
        section { class: "section",
            div { class: "container",
                h1 { class: "section-title", "About Me" }

                div { class: "card",
                    p { class: "text-secondary",
                        "Software developer passionate about building high-performance applications with Rust and modern web technologies."
                    }
                }
            }
        }
    }
}
