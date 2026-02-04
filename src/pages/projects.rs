//! Projects Page - Portfolio showcase

use crate::components::layout_components::{Container, Grid, Section};
use crate::components::molecules::SectionTitle;
use crate::components::projects::{ProjectCard, ProjectStatus, get_projects};
use dioxus::prelude::*;

/// Projects listing page - All projects with dynamic data
#[component]
pub fn ProjectsPage() -> Element {
    let projects = get_projects();
    let featured: Vec<_> = projects
        .iter()
        .filter(|p| p.status == ProjectStatus::Featured)
        .cloned()
        .collect();
    let active: Vec<_> = projects
        .iter()
        .filter(|p| p.status == ProjectStatus::Active)
        .cloned()
        .collect();
    let archived: Vec<_> = projects
        .iter()
        .filter(|p| p.status == ProjectStatus::Archived)
        .cloned()
        .collect();

    rsx! {
        Section { id: "projects-page",
            Container {
                SectionTitle {
                    text: "All Projects".to_string(),
                    subtitle: "Complete Portfolio".to_string(),
                    center: true
                }

                // Featured Projects
                if !featured.is_empty() {
                    div { class: "mb-12",
                        h3 { class: "text-lg font-bold text-primary mb-6", "⭐ Featured Projects" }
                        Grid { cols: 1, md_cols: 2, gap: 6,
                            for project in featured.iter() {
                                ProjectCard { project: project.clone(), featured: true }
                            }
                        }
                    }
                }

                // Active Projects
                if !active.is_empty() {
                    div { class: "mb-12",
                        h3 { class: "text-lg font-bold text-purple-400 mb-6", "🚀 Active Projects" }
                        Grid { cols: 1, md_cols: 2, gap: 6,
                            for project in active.iter() {
                                ProjectCard { project: project.clone(), featured: false }
                            }
                        }
                    }
                }

                // Archived Projects
                if !archived.is_empty() {
                    div { class: "mb-12",
                        h3 { class: "text-lg font-bold text-orange-400 mb-6", "📦 Archived Projects" }
                        Grid { cols: 1, md_cols: 2, gap: 6,
                            for project in archived.iter() {
                                ProjectCard { project: project.clone(), featured: false }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Project detail page
#[component]
pub fn ProjectDetailPage(slug: String) -> Element {
    use crate::components::projects::get_project_by_id;

    let project = get_project_by_id(&slug);

    rsx! {
        Section { id: "project-detail",
            Container {
                if let Some(p) = project {
                    div { class: "max-w-4xl mx-auto",
                        // Header
                        div { class: "mb-8",
                            h1 { class: "text-3xl font-bold text-white mb-4", "{p.title}" }
                            p { class: "text-muted text-lg", "{p.long_description}" }
                        }

                        // Technologies
                        div { class: "flex flex-wrap gap-2 mb-8",
                            for tech in p.technologies.iter() {
                                span { class: "px-3 py-1 bg-primary/20 text-primary rounded-full text-sm", "{tech}" }
                            }
                        }

                        // Links
                        div { class: "flex gap-4",
                            if let Some(github) = p.github_url {
                                a {
                                    href: "{github}",
                                    target: "_blank",
                                    class: "px-6 py-3 bg-bg-card border border-white/10 rounded-lg hover:border-primary transition-colors",
                                    "View on GitHub"
                                }
                            }
                            if let Some(demo) = p.demo_url {
                                a {
                                    href: "{demo}",
                                    target: "_blank",
                                    class: "px-6 py-3 bg-primary text-bg-dark rounded-lg hover:bg-primary/80 transition-colors",
                                    "Live Demo"
                                }
                            }
                        }
                    }
                } else {
                    div { class: "text-center py-16",
                        h1 { class: "text-2xl font-bold text-white mb-4", "Project not found" }
                        p { class: "text-muted", "The project \"{slug}\" does not exist." }
                    }
                }
            }
        }
    }
}
