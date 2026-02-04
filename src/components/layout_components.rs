//! Layout Components
//! Helpers for positioning and structure

use dioxus::prelude::*;

/// Responsive Container
#[component]
pub fn Container(
    children: Element,
    #[props(default = "max-w-7xl".to_string())] max_width: String,
    #[props(default = "".to_string())] class: String,
) -> Element {
    rsx! {
        div { class: "container mx-auto px-4 sm:px-6 lg:px-8 {max_width} {class}",
            {children}
        }
    }
}

/// Grid System
#[component]
pub fn Grid(
    children: Element,
    #[props(default = 1)] cols: u8,
    #[props(default = 3)] md_cols: u8,
    #[props(default = 4)] gap: u8,
    #[props(default = "".to_string())] class: String,
) -> Element {
    // Tailwind classes mapping simplifed
    let cols_class = match cols {
        1 => "grid-cols-1",
        2 => "grid-cols-2",
        3 => "grid-cols-3",
        4 => "grid-cols-4",
        _ => "grid-cols-1",
    };

    let md_cols_class = match md_cols {
        1 => "md:grid-cols-1",
        2 => "md:grid-cols-2",
        3 => "md:grid-cols-3",
        4 => "md:grid-cols-4",
        _ => "md:grid-cols-3",
    };

    rsx! {
        div { class: "grid {cols_class} {md_cols_class} gap-{gap} {class}",
            {children}
        }
    }
}

/// Section Wrapper with Spacing
#[component]
pub fn Section(
    children: Element,
    #[props(default = false)] alternate: bool,
    #[props(default = "".to_string())] id: String,
    #[props(default = "".to_string())] class: String,
) -> Element {
    let bg_class = if alternate {
        "bg-bg-secondary/30"
    } else {
        "bg-transparent"
    };

    rsx! {
        section {
            id: "{id}",
            class: "py-16 md:py-24 relative overflow-hidden {bg_class} {class}",
            {children}
        }
    }
}

/// Vertical Spacer
#[component]
pub fn Spacer(#[props(default = 8)] h: u8) -> Element {
    rsx! {
        div { class: "h-{h}" }
    }
}
