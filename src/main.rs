//! enerby.dev - Main Application Entry Point
//! A Cyberpunk-styled Portfolio built with Dioxus & Rust

use dioxus::prelude::*;

// Module declarations
mod components;
mod config;
mod data;
mod layouts;
mod pages;
mod routes;
mod theme;
mod utils;
mod i18n;

#[cfg(test)]
mod tests;

// Construction overlay — remove "construction" from default features in Cargo.toml to disable
#[cfg(feature = "construction")]
mod construction;

// Re-exports
#[cfg(not(feature = "construction"))]
use routes::Route;

// Assets
const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

/// Main application component
#[component]
#[cfg(feature = "construction")]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Stylesheet { href: TAILWIND_CSS }
        document::Title { "{config::SITE.title}" }
        document::Meta {
            name: "description",
            content: "{config::SITE.description}"
        }
        document::Meta {
            name: "viewport",
            content: "width=device-width, initial-scale=1.0"
        }

        construction::ConstructionOverlay {}
    }
}

/// Main application component with router
#[component]
#[cfg(not(feature = "construction"))]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Stylesheet { href: TAILWIND_CSS }
        document::Title { "{config::SITE.title}" }
        document::Meta {
            name: "description",
            content: "{config::SITE.description}"
        }
        document::Meta {
            name: "viewport",
            content: "width=device-width, initial-scale=1.0"
        }

        Router::<Route> {}
    }
}
