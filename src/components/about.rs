//! About Section Component
//! Biography, timeline, and interests

use crate::components::layout_components::{Container, Section};
use crate::components::molecules::{Card, SectionTitle};
use crate::utils::{format_loc, get_github_stats};
use dioxus::prelude::*;

/// About Section - Biography and experience
#[component]
pub fn AboutSection() -> Element {
    rsx! {
        Section { id: "about", alternate: true,
            Container {
                // Section Title (P7-A2)
                SectionTitle {
                    text: "About Me".to_string(),
                    subtitle: "Who I Am".to_string()
                }

                div { class: "grid lg:grid-cols-2 gap-12",
                    // Left: Bio (P7-A3)
                    div { class: "space-y-6",
                        p { class: "text-lg text-secondary leading-relaxed",
                            "I'm a passionate software developer based in Mexico, specializing in "
                            span { class: "text-primary font-semibold", "Rust" }
                            " and modern web technologies. With a deep love for clean architecture, performance optimization, and cyberpunk aesthetics, I craft digital experiences that are both beautiful and blazingly fast."
                        }

                        p { class: "text-muted leading-relaxed",
                            "My journey in tech started with curiosity and evolved into a passion for building tools that make a difference. I believe in the power of open source, continuous learning, and pushing the boundaries of what's possible."
                        }

                        // Highlights/Facts (P7-A4)
                        div { class: "grid grid-cols-2 gap-4 mt-8",
                            HighlightCard { number: "5+", label: "Years Experience" }
                            HighlightCard { number: "50+", label: "Projects Completed" }
                            HighlightCard { number: "10+", label: "Technologies" }
                            // Dynamic Lines of Code counter (F10)
                            DynamicLocCounter {}
                        }
                    }

                    // Right: Timeline (P7-B)
                    div {
                        h3 { class: "text-xl font-bold text-white mb-6 flex items-center gap-2",
                            span { class: "text-primary", "▸" }
                            "Experience Timeline"
                        }

                        Timeline {
                            TimelineItem {
                                year: "2024 - Present".to_string(),
                                title: "Full-Stack Developer".to_string(),
                                company: "Freelance".to_string(),
                                description: "Building high-performance web applications with Rust, Dioxus, and modern frontend technologies.".to_string()
                            }
                            TimelineItem {
                                year: "2022 - 2024".to_string(),
                                title: "Software Engineer".to_string(),
                                company: "Tech Startup".to_string(),
                                description: "Led development of real-time systems and contributed to core architecture decisions.".to_string()
                            }
                            TimelineItem {
                                year: "2020 - 2022".to_string(),
                                title: "Junior Developer".to_string(),
                                company: "Digital Agency".to_string(),
                                description: "Started journey with web development, learning fundamentals and best practices.".to_string()
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Timeline Container (P7-B1)
#[component]
pub fn Timeline(children: Element) -> Element {
    rsx! {
        div { class: "relative pl-8 border-l-2 border-primary/30 space-y-8",
            {children}
        }
    }
}

/// Timeline Item (P7-B2)
#[component]
pub fn TimelineItem(year: String, title: String, company: String, description: String) -> Element {
    rsx! {
        div { class: "relative group animate-fade-in-up",
            // Timeline dot
            div { class: "absolute -left-[2.56rem] top-0 w-4 h-4 rounded-full border-2 border-primary bg-bg-primary group-hover:bg-primary transition-colors" }

            // Content
            div { class: "bg-bg-card/50 p-4 rounded-lg border border-white/5 group-hover:border-primary/30 transition-all",
                span { class: "text-xs text-primary font-mono", "{year}" }
                h4 { class: "text-lg font-bold text-white mt-1", "{title}" }
                p { class: "text-secondary text-sm", "{company}" }
                p { class: "text-muted text-sm mt-2", "{description}" }
            }
        }
    }
}

/// Highlight Card for stats
#[component]
fn HighlightCard(number: &'static str, label: &'static str) -> Element {
    rsx! {
        Card {
            div { class: "text-center py-2",
                p { class: "text-3xl font-bold text-primary font-display", "{number}" }
                p { class: "text-xs text-muted uppercase tracking-wider mt-1", "{label}" }
            }
        }
    }
}

/// Dynamic Lines of Code Counter (F10)
/// Displays real LOC from github_stats module
#[component]
fn DynamicLocCounter() -> Element {
    let stats = get_github_stats();
    let loc_formatted = format_loc(stats.total_loc);
    
    rsx! {
        Card {
            div { 
                class: "text-center py-2",
                title: "Lines of code across {stats.repos.len()} repositories",
                p { class: "text-3xl font-bold text-primary font-display", "{loc_formatted}" }
                p { class: "text-xs text-muted uppercase tracking-wider mt-1", "Lines of Code" }
            }
        }
    }
}

// InterestCard component removed - Beyond Code section eliminated

