//! enerby.dev - Main Application Entry Point
//! A Cyberpunk-styled Portfolio built with Dioxus & Rust

use dioxus::prelude::*;

// Module declarations
mod routes;
mod config;
mod data;
mod utils;
mod theme;
mod pages;
mod layouts;
mod components;

#[cfg(test)]
mod tests;

// Re-exports
use routes::Route;

// Assets
const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

/// Main application component with router
#[component]
fn App() -> Element {
    rsx! {
        // Document head - usando método oficial de Dioxus
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
        
        // Router
        Router::<Route> {}
    }
}
