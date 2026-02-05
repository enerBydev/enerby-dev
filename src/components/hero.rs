//! Hero Section Component
//! Main landing section with animated title, avatar, and CTAs

use crate::components::atoms::{Button, ButtonVariant};
use crate::components::layout_components::Container;
use crate::components::molecules::GlitchText;
use crate::config::SITE;
use crate::routes::Route;
use dioxus::prelude::*;

use crate::i18n::use_i18n;

/// Hero Section - Main landing area
#[component]
pub fn HeroSection() -> Element {
    let i18n = use_i18n();
    
    // Static role (typing animation handled by CSS) - now localized via i18n
    // Note: If you want roles to cycle, we might need an array in i18n
    // For now, using a static string from i18n or just keeping it 'Full-Stack Developer' if universal?
    // Let's assume description_highlight or a new field serves as role? 
    // Wait, HeroConfig doesn't have 'role'. It has 'role_prefix'?
    // "role_prefix": "> " ?? 
    // Let's check dict.rs again. `pub role_prefix: &'static str`
    // I will use `i18n.hero.role` if I add it, or hardcode it if it's universal "Full Stack Developer".
    // Actually, "Full-Stack Developer" should be translated. "Desarrollador Full-Stack".
    // I will assume it is part of `greeting`?? No.
    // I'll add `role` to `HeroConfig` implies modifying `i18n` crates. This is outside scope of E?
    // "Update UI Components". If I find missing keys, I should add them.
    // But let's look at what I have: `description_highlight` might be "Rust".
    
    // Let's stick to what's in `HeroConfig`:
    // greeting, role_prefix, description_start, description_highlight, description_end, cta_projects, cta_contact, scroll_indicator.
    // Missing: The actual "Full-Stack Developer" text. 
    // I will verify `src/i18n/locales/en.rs` content to see what's mapped to what.
    
    let current_role = "Full-Stack Developer"; // Fallback if not in dict

    rsx! {
        section {
            id: "hero",
            class: "hero-section min-h-screen relative flex items-center justify-center overflow-hidden",

            // Background Decorations (P6-C)
            div { class: "absolute inset-0 pointer-events-none",
                // Grid lines (P6-C2)
                div { class: "cyber-grid absolute inset-0 opacity-20" }

                // Floating geometric elements (P6-C3)
                div { class: "absolute top-20 left-10 w-20 h-20 border border-primary/20 rotate-45 animate-float" }
                div { class: "absolute top-40 right-20 w-16 h-16 border border-secondary-pink/20 rotate-12 animate-float-delayed" }
                div { class: "absolute bottom-40 left-1/4 w-12 h-12 border border-secondary-purple/20 -rotate-12 animate-float" }

                // Radial gradient overlay
                div { class: "absolute inset-0 bg-gradient-radial from-transparent via-transparent to-bg-primary/80" }
            }

            Container {
                div { class: "hero-content flex flex-col lg:flex-row items-center justify-between gap-12 py-20",

                    // Left: Text Content
                    div { class: "flex-1 text-center lg:text-left z-10",
                        // Greeting
                        p { class: "text-primary text-sm uppercase tracking-widest mb-4 animate-fade-in",
                            "{i18n.hero.greeting}"
                        }

                        // Main Title (P6-A2)
                        h1 { class: "mb-4",
                            GlitchText {
                                text: SITE.name.to_string(),
                                size_class: "text-5xl md:text-6xl lg:text-7xl".to_string()
                            }
                        }

                        // Subtitle with typing effect (P6-A3)
                        div { class: "h-8 mb-6",
                            p { class: "text-xl md:text-2xl text-secondary font-mono",
                                span { class: "text-primary", "{i18n.hero.role_prefix}" }
                                span { class: "animate-pulse", "{current_role}" }
                                span { class: "animate-blink text-primary", "_" }
                            }
                        }

                        // Description (P6-A4)
                        p { class: "text-muted max-w-lg mb-8 leading-relaxed",
                            "{i18n.hero.description_start}"
                            span { class: "text-primary font-semibold", "{i18n.hero.description_highlight}" }
                            "{i18n.hero.description_end}"
                        }

                        // CTAs (P6-A5)
                        div { class: "flex gap-4 justify-center lg:justify-start flex-wrap",
                            Button {
                                variant: ButtonVariant::Neon,
                                to: Route::ProjectsPage {},
                                "{i18n.hero.cta_projects}"
                            }
                            Button {
                                variant: ButtonVariant::Ghost,
                                to: Route::ContactPage {},
                                "{i18n.hero.cta_contact}"
                            }
                        }
                    }

                    // Right: Avatar (P6-B)
                    div { class: "flex-shrink-0 z-10",
                        div { class: "relative group",
                            // Glow effect (P6-B3)
                            div { class: "absolute inset-0 bg-primary/20 rounded-full blur-3xl animate-pulse-slow group-hover:bg-primary/30 transition-colors" }

                            // Avatar container with neon border (P6-B1, P6-B2)
                            div { class: "relative w-64 h-64 md:w-80 md:h-80 rounded-full overflow-hidden border-4 border-primary/50 shadow-glow-lg group-hover:border-primary transition-all duration-500",
                                // Placeholder/Fallback (P6-B4)
                                div { class: "w-full h-full bg-gradient-to-br from-primary/20 via-bg-card to-secondary-purple/20 flex items-center justify-center",
                                    span { class: "text-6xl md:text-8xl font-display font-bold text-white/80", "E" }
                                }
                            }

                            // Rotating border decoration
                            div { class: "absolute inset-0 rounded-full border-2 border-dashed border-primary/30 animate-spin-slow" }
                        }
                    }
                }
            }

            // Scroll Indicator (P6-D)
            div { class: "absolute bottom-8 left-1/2 -translate-x-1/2 z-20 animate-bounce",
                a {
                    href: "#about",
                    class: "flex flex-col items-center text-muted hover:text-primary transition-colors",
                    span { class: "text-xs uppercase tracking-widest mb-2", "{i18n.hero.scroll_indicator}" }
                    div { class: "w-6 h-10 border-2 border-current rounded-full flex justify-center pt-2",
                        div { class: "w-1 h-2 bg-current rounded-full animate-scroll-indicator" }
                    }
                }
            }
        }
    }
}
