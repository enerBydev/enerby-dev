//! About Section Component
//! Biography, timeline, and interests

use crate::components::layout_components::{Container, Section};
use crate::components::molecules::{Card, SectionTitle};
use crate::components::projects::get_projects;
use crate::utils::{format_loc, get_github_stats};
use dioxus::prelude::*;

/// About Section - Biography and experience
#[component]
pub fn AboutSection() -> Element {
    let i18n = crate::i18n::use_i18n(); // Helper hook

    rsx! {
        Section { id: "about", alternate: true,
            Container {
                // Section Title (P7-A2)
                SectionTitle {
                    text: i18n.about.title.to_string(),
                    subtitle: i18n.about.subtitle.to_string()
                }

                div { class: "grid lg:grid-cols-2 gap-12",
                    // Left: Bio (P7-A3)
                    div { class: "space-y-6",
                        p { class: "text-lg text-secondary leading-relaxed",
                            "{i18n.about.bio_p1_start}"
                            span { class: "text-primary font-semibold", "{i18n.about.bio_p1_highlight}" }
                            "{i18n.about.bio_p1_end}"
                        }

                        p { class: "text-muted leading-relaxed",
                            "{i18n.about.bio_p2}"
                        }

                        // Highlights/Facts (P7-A4)
                        div { class: "grid grid-cols-2 gap-4 mt-8",
                            HighlightCard { number: "5+", label: "{i18n.about.stat_experience}" }
                            // Dynamic Living Projects counter
                            DynamicProjectsCounter {}
                            HighlightCard { number: "10+", label: "{i18n.about.stat_technologies}" }
                            // Dynamic Lines of Code counter (F10)
                            DynamicLocCounter {}
                        }
                    }

                    // Right: Timeline (P7-B)
                    div {
                        h3 { class: "text-xl font-bold text-white mb-6 flex items-center gap-2",
                            span { class: "text-primary", "▸" }
                            "{i18n.about.timeline_title}"
                        }

                        Timeline {
                            for item in i18n.about.timeline.iter() {
                                TimelineItem {
                                    year: item.year.to_string(),
                                    title: item.title.to_string(),
                                    company: item.company.to_string(),
                                    description: item.description.to_string()
                                }
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
fn HighlightCard(number: String, label: String) -> Element {
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
    let i18n = crate::i18n::use_i18n();

    rsx! {
        Card {
            div {
                class: "text-center py-2",
                title: "Lines of code across {stats.repos.len()} repositories",
                p { class: "text-3xl font-bold text-primary font-display", "{loc_formatted}" }
                p { class: "text-xs text-muted uppercase tracking-wider mt-1", "{i18n.about.stat_loc}" }
            }
        }
    }
}

/// Dynamic Living Projects Counter
/// Counts projects dynamically from projects.rs
#[component]
fn DynamicProjectsCounter() -> Element {
    let projects = get_projects();
    let count = projects.len();
    let i18n = crate::i18n::use_i18n();

    rsx! {
        Card {
            div {
                class: "text-center py-2",
                title: "Active repositories on GitHub",
                p { class: "text-3xl font-bold text-primary font-display", "{count}" }
                p { class: "text-xs text-muted uppercase tracking-wider mt-1", "{i18n.about.stat_projects}" }
            }
        }
    }
}
