//! Hero Section Component
//! Main landing section with animated title, avatar, and CTAs

use crate::components::atoms::{Button, ButtonVariant};
use crate::components::layout_components::Container;
use crate::components::molecules::GlitchText;
use crate::config::SITE;
use crate::routes::Route;
use dioxus::prelude::*;

/// Hero Section - Main landing area
#[component]
pub fn HeroSection() -> Element {
    // Static role (typing animation handled by CSS)
    let current_role = "Full-Stack Developer";

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
                            "👋 Hello, I'm"
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
                                span { class: "text-primary", "> " }
                                span { class: "animate-pulse", "{current_role}" }
                                span { class: "animate-blink text-primary", "_" }
                            }
                        }

                        // Description (P6-A4)
                        p { class: "text-muted max-w-lg mb-8 leading-relaxed",
                            "Crafting high-performance, beautiful digital experiences with "
                            span { class: "text-primary font-semibold", "Rust" }
                            " and modern web technologies. Passionate about clean code, cyberpunk aesthetics, and pushing the boundaries of what's possible on the web."
                        }

                        // CTAs (P6-A5)
                        div { class: "flex gap-4 justify-center lg:justify-start flex-wrap",
                            Button {
                                variant: ButtonVariant::Neon,
                                to: Route::ProjectsPage {},
                                "View My Work"
                            }
                            Button {
                                variant: ButtonVariant::Ghost,
                                to: Route::ContactPage {},
                                "Get In Touch"
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
                    span { class: "text-xs uppercase tracking-widest mb-2", "Scroll" }
                    div { class: "w-6 h-10 border-2 border-current rounded-full flex justify-center pt-2",
                        div { class: "w-1 h-2 bg-current rounded-full animate-scroll-indicator" }
                    }
                }
            }
        }
    }
}
