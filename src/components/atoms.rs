//! Atomic Components
//! Basic building blocks for the UI

use dioxus::prelude::*;
use crate::routes::Route;

/// Button Variants
#[derive(PartialEq, Clone, Copy)]
#[derive(Debug)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Ghost,
    Neon,
}

/// Button Component
#[component]
pub fn Button(
    children: Element,
    #[props(default = ButtonVariant::Primary)] variant: ButtonVariant,
    #[props(default = "".to_string())] class: String,
    to: Option<Route>,
    href: Option<String>,
    #[props(default = false)] new_tab: bool,
    onclick: Option<EventHandler<MouseEvent>>,
) -> Element {
    let base_class = "btn transition-all duration-300 font-bold uppercase tracking-wider text-sm py-3 px-6 rounded-sm relative overflow-hidden group";
    
    let variant_class = match variant {
        ButtonVariant::Primary => "btn-primary bg-primary text-bg-primary hover:bg-primary-dark hover:shadow-glow-md",
        ButtonVariant::Secondary => "bg-bg-element text-white hover:bg-bg-element-light border border-white/10",
        ButtonVariant::Ghost => "bg-transparent text-primary hover:bg-primary/10 border border-transparent hover:border-primary/30",
        ButtonVariant::Neon => "bg-transparent text-primary border border-primary shadow-glow-sm hover:shadow-glow-lg hover:bg-primary/10",
    };

    let full_class = format!("{} {} {}", base_class, variant_class, class);

    if let Some(route) = to {
        rsx! {
            Link {
                to: route,
                class: "{full_class}",
                {children}
            }
        }
    } else if let Some(url) = href {
        rsx! {
            a {
                href: "{url}",
                class: "{full_class}",
                target: if new_tab { "_blank" } else { "_self" },
                rel: if new_tab { "noopener noreferrer" } else { "" },
                {children}
            }
        }
    } else {
        rsx! {
            button {
                class: "{full_class}",
                onclick: move |evt| if let Some(handler) = onclick { handler.call(evt) },
                {children}
                
                // Scanline effect on hover for Neon variant
                if variant == ButtonVariant::Neon {
                    span { class: "absolute inset-0 bg-white/20 transform -translate-x-full group-hover:animate-shine pointer-events-none" }
                }
            }
        }
    }
}

/// Badge Component
#[component]
pub fn Badge(
    children: Element,
    #[props(default = "cyan".to_string())] color: String, // cyan, pink, purple
) -> Element {
    let color_class = match color.as_str() {
        "pink" => "bg-secondary-pink/10 text-secondary-pink border-secondary-pink/50",
        "purple" => "bg-secondary-purple/10 text-secondary-purple border-secondary-purple/50",
        "orange" => "bg-secondary-orange/10 text-secondary-orange border-secondary-orange/50",
        _ => "bg-primary/10 text-primary border-primary/50", // Default Cyan
    };

    rsx! {
        span {
            class: "inline-flex items-center px-2.5 py-0.5 rounded text-xs font-medium border {color_class}",
            {children}
        }
    }
}

/// Divider Component
#[component]
pub fn Divider(
    #[props(default = false)] vertical: bool,
    #[props(default = false)] neon: bool,
) -> Element {
    let base = if vertical { "w-px h-full mx-4" } else { "h-px w-full my-8" };
    let style = if neon { "bg-gradient-to-r from-transparent via-primary to-transparent opacity-50 shadow-glow-sm" } else { "bg-white/10" };
    
    rsx! {
        div { class: "{base} {style}" }
    }
}

/// Spinner Component (Cyberpunk Loading)
#[component]
pub fn Spinner(#[props(default = "cyan".to_string())] color: String) -> Element {
    let border_color = match color.as_str() {
        "pink" => "border-secondary-pink",
        _ => "border-primary",
    };
    
    rsx! {
        div { class: "relative w-12 h-12",
            div { class: "absolute inset-0 border-2 {border_color} opacity-20 rounded-full" }
            div { class: "absolute inset-0 border-t-2 {border_color} rounded-full animate-spin shadow-glow-sm" }
            div { class: "absolute inset-4 bg-{color} opacity-10 rounded-full animate-pulse" }
        }
    }
}

/// Icon Wrapper (Placeholder for now, usually would handle SVG paths)
#[component]
pub fn Icon(
    name: String,
    #[props(default = 24)] size: u32,
    #[props(default = "".to_string())] class: String
) -> Element {
    // In a real app, this would match `name` to SVG paths
    // For now, it wraps a generic container
    rsx! {
        div { 
            class: "inline-block {class}",
            style: "width: {size}px; height: {size}px;",
            // Placeholder content
            span { class: "text-xs", "?" }
        }
    }
}
