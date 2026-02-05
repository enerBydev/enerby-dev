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
    let hover_class = if hover_effect {
        "hover:border-primary/50 hover:shadow-glow-sm hover:-translate-y-1 group"
    } else {
        ""
    };
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
    let align_class = if center {
        "text-center items-center"
    } else {
        "text-left items-start"
    };

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
pub fn Terminal(title: String, children: Element) -> Element {
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

/// Glitch Text Component - Cyberpunk glitch effect on hover
/// Usa CSS ::before/::after con data-text para el efecto glitch
#[component]
pub fn GlitchText(
    text: String,
    #[props(default = "text-4xl".to_string())] size_class: String,
) -> Element {
    rsx! {
        span {
            class: "glitch-text relative inline-block {size_class} font-display font-bold text-white",
            "data-text": "{text}",
            "{text}"
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

/// Language Switcher Component
/// Toggles between available languages
#[component]
pub fn LanguageToggle() -> Element {
    // Read global state directly to get current enum value
    let i18n_state = crate::i18n::I18N_CONFIG.read();
    let current_lang = i18n_state.language;

    rsx! {
        button {
            class: "language-toggle flex items-center gap-2 px-3 py-1.5 rounded-full border border-white/10 bg-black/20 hover:border-primary/50 transition-all cursor-pointer group backdrop-blur-sm",
            onclick: move |_| crate::i18n::toggle_language(),
            title: "Switch Language / Cambiar Idioma",

            // EN Segment
            span {
                class: if current_lang == crate::i18n::Language::EN { "text-primary font-bold font-mono text-xs shadow-glow-sm" } else { "text-muted font-mono text-xs group-hover:text-white transition-colors" },
                "EN"
            }

            // Separator
            span { class: "text-white/10 text-xs", "|" }

            // ES Segment
            span {
                class: if current_lang == crate::i18n::Language::ES { "text-primary font-bold font-mono text-xs shadow-glow-sm" } else { "text-muted font-mono text-xs group-hover:text-white transition-colors" },
                "ES"
            }
        }
    }
}
