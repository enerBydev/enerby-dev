//! Molecular Components
//! Complex UI elements built from atoms

use dioxus::prelude::*;

/// Card Component with Glassmorphism and Hover Glue
#[component]
pub fn Card(
    children: Element,
    #[props(default = "".to_string())] class: String,
    #[props(default = false)] hover_effect: bool,
    #[props(default = false)] no_padding: bool,
) -> Element {
    let base_class = "relative overflow-hidden rounded-lg bg-bg-card backdrop-blur-md border border-white/5 transition-all duration-300";
    let hover_class = if hover_effect { "hover:border-primary/50 hover:shadow-glow-sm hover:-translate-y-1 group" } else { "" };
    let padding_class = if no_padding { "" } else { "p-6" };
    
    rsx! {
        div { 
            class: "{base_class} {hover_class} {padding_class} {class}",
            {children}
            
            // Corner Accents (Cyberpunk style)
            div { class: "absolute top-0 left-0 w-2 h-2 border-t border-l border-primary/30 rounded-tl-sm" }
            div { class: "absolute top-0 right-0 w-2 h-2 border-t border-r border-primary/30 rounded-tr-sm" }
            div { class: "absolute bottom-0 left-0 w-2 h-2 border-b border-l border-primary/30 rounded-bl-sm" }
            div { class: "absolute bottom-0 right-0 w-2 h-2 border-b border-r border-primary/30 rounded-br-sm" }
        }
    }
}

/// Section Title with Glitch/Neon Effect
#[component]
pub fn SectionTitle(
    text: String,
    #[props(default = "".to_string())] subtitle: String,
    #[props(default = false)] center: bool,
) -> Element {
    let align_class = if center { "text-center items-center" } else { "text-left items-start" };
    
    rsx! {
        div { class: "flex flex-col mb-12 {align_class}",
            h2 { class: "text-3xl md:text-4xl font-display font-bold text-white mb-2 relative inline-block",
                span { class: "relative z-10", "{text}" }
                span { class: "absolute -bottom-2 left-0 w-1/2 h-1 bg-gradient-to-r from-primary to-transparent" }
            }
            if !subtitle.is_empty() {
                p { class: "text-secondary text-sm uppercase tracking-widest mt-2", "{subtitle}" }
            }
        }
    }
}

/// Code Terminal Component
#[component]
pub fn Terminal(
    title: String,
    children: Element,
) -> Element {
    rsx! {
        div { class: "rounded-lg overflow-hidden bg-[#0F0F16] border border-white/10 font-mono text-sm shadow-xl w-full",
            // Terminal Header
            div { class: "bg-[#1A1A24] px-4 py-2 flex items-center gap-2 border-b border-white/5",
                div { class: "flex gap-1.5",
                    div { class: "w-3 h-3 rounded-full bg-red-500/80" }
                    div { class: "w-3 h-3 rounded-full bg-yellow-500/80" }
                    div { class: "w-3 h-3 rounded-full bg-green-500/80" }
                }
                div { class: "ml-2 text-xs text-muted select-none", "{title}" }
            }
            // Terminal Body
            div { class: "p-4 overflow-x-auto text-gray-300",
                {children}
            }
        }
    }
}

/// Glitch Text Component
#[component]
pub fn GlitchText(
    text: String,
    #[props(default = "text-4xl".to_string())] size_class: String,
) -> Element {
    rsx! {
        div { class: "relative inline-block group",
            span { class: "relative z-10 {size_class} font-display font-bold text-white", "{text}" }
            span { class: "absolute top-0 left-0 -z-10 w-full h-full {size_class} font-display font-bold text-primary opacity-70 animate-glitch-1 hidden group-hover:block", "{text}" }
            span { class: "absolute top-0 left-0 -z-10 w-full h-full {size_class} font-display font-bold text-secondary-pink opacity-70 animate-glitch-2 hidden group-hover:block", "{text}" }
        }
    }
}

/// Progress Bar (Skill)
#[component]
pub fn ProgressBar(
    label: String,
    percentage: u8,
    #[props(default = "cyan".to_string())] color: String,
) -> Element {
    let width = format!("{}%", percentage);
    let color_class = match color.as_str() {
        "pink" => "bg-secondary-pink shadow-[0_0_10px_rgba(255,0,255,0.5)]",
        "purple" => "bg-secondary-purple shadow-[0_0_10px_rgba(157,0,255,0.5)]",
        _ => "bg-primary shadow-[0_0_10px_rgba(0,255,255,0.5)]",
    };

    rsx! {
        div { class: "w-full mb-4",
            div { class: "flex justify-between mb-1",
                span { class: "text-sm font-medium text-white", "{label}" }
                span { class: "text-sm font-medium text-muted", "{percentage}%" }
            }
            div { class: "w-full bg-white/5 rounded-full h-2.5 overflow-hidden border border-white/5",
                div { 
                    class: "h-2.5 rounded-full {color_class} transition-all duration-1000 ease-out",
                    style: "width: {width}"
                }
            }
        }
    }
}
