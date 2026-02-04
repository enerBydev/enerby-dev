//! Projects Section and Detail Components
//! Project showcase with filtering and detail views

use dioxus::prelude::*;
use crate::routes::Route;
use crate::components::molecules::{Card, SectionTitle};
use crate::components::atoms::{Button, ButtonVariant, Badge};
use crate::components::layout_components::{Container, Section, Grid};

/// Project Status (P9-A4)
#[derive(Clone, PartialEq, Copy)]
pub enum ProjectStatus {
    Featured,
    Active,
    Archived,
}

impl ProjectStatus {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Featured => "Featured",
            Self::Active => "Active",
            Self::Archived => "Archived",
        }
    }
    
    pub fn color(&self) -> &'static str {
        match self {
            Self::Featured => "cyan",
            Self::Active => "purple",
            Self::Archived => "orange",
        }
    }
}

/// Project data structure (P9-A1, P9-A2)
#[derive(Clone, PartialEq)]
pub struct Project {
    pub id: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub long_description: &'static str,
    pub technologies: Vec<&'static str>,
    pub status: ProjectStatus,
    pub github_url: Option<&'static str>,
    pub demo_url: Option<&'static str>,
    pub image_placeholder: &'static str,
}

/// Get all projects data (P9-B1 - hardcoded for now, can be moved to TOML later)
pub fn get_projects() -> Vec<Project> {
    vec![
        Project {
            id: "enerby-dev",
            title: "enerby.dev",
            description: "Personal portfolio website built with Rust and Dioxus. Cyberpunk-themed with modern web technologies.",
            long_description: "A fully-featured portfolio website showcasing my work, skills, and blog posts. Built entirely in Rust using the Dioxus framework, featuring a cyberpunk aesthetic with neon colors, glassmorphism, and smooth animations.",
            technologies: vec!["Rust", "Dioxus", "WebAssembly", "Tailwind CSS"],
            status: ProjectStatus::Featured,
            github_url: Some("https://github.com/enerbydev/enerby.dev"),
            demo_url: Some("https://enerby.dev"),
            image_placeholder: "E",
        },
        Project {
            id: "rust-cli-tools",
            title: "Rust CLI Tools",
            description: "Collection of command-line utilities written in Rust for productivity and automation.",
            long_description: "A suite of high-performance CLI tools built in Rust, including file processors, data transformers, and automation scripts. Designed with a focus on speed, reliability, and ergonomic APIs.",
            technologies: vec!["Rust", "Clap", "Tokio", "Serde"],
            status: ProjectStatus::Active,
            github_url: Some("https://github.com/enerbydev/rust-cli-tools"),
            demo_url: None,
            image_placeholder: "R",
        },
        Project {
            id: "web-crawler",
            title: "Async Web Crawler",
            description: "High-performance asynchronous web crawler with configurable depth and rate limiting.",
            long_description: "An async web crawler built with Tokio and reqwest, featuring configurable crawl depth, rate limiting, robots.txt compliance, and structured data extraction.",
            technologies: vec!["Rust", "Tokio", "Reqwest", "Scraper"],
            status: ProjectStatus::Active,
            github_url: Some("https://github.com/enerbydev/web-crawler"),
            demo_url: None,
            image_placeholder: "W",
        },
        Project {
            id: "legacy-project",
            title: "Legacy Dashboard",
            description: "Admin dashboard built with Vue.js and Node.js. No longer actively maintained.",
            long_description: "A full-stack admin dashboard application featuring real-time analytics, user management, and reporting tools. Built during my earlier career with Vue.js frontend and Node.js backend.",
            technologies: vec!["Vue.js", "Node.js", "PostgreSQL", "Docker"],
            status: ProjectStatus::Archived,
            github_url: None,
            demo_url: None,
            image_placeholder: "L",
        },
    ]
}

/// Get project by ID
pub fn get_project_by_id(id: &str) -> Option<Project> {
    get_projects().into_iter().find(|p| p.id == id)
}

/// Projects Section (P9-C1)
#[component]
pub fn ProjectsSection() -> Element {
    let projects = get_projects();
    let featured: Vec<_> = projects.iter().filter(|p| p.status == ProjectStatus::Featured).cloned().collect();
    let active: Vec<_> = projects.iter().filter(|p| p.status == ProjectStatus::Active).cloned().collect();

    rsx! {
        Section { id: "projects", alternate: true,
            Container {
                SectionTitle { 
                    text: "Projects".to_string(), 
                    subtitle: "What I've Built".to_string(),
                    center: true 
                }
                
                // Featured Projects
                if !featured.is_empty() {
                    div { class: "mb-12",
                        for project in featured.iter() {
                            ProjectCard { project: project.clone(), featured: true }
                        }
                    }
                }
                
                // Active Projects Grid (P9-C3)
                Grid { cols: 1, md_cols: 2, gap: 6,
                    for project in active.iter() {
                        ProjectCard { project: project.clone(), featured: false }
                    }
                }
                
                // View All Button
                div { class: "text-center mt-12",
                    Button { 
                        variant: ButtonVariant::Ghost,
                        to: Route::ProjectsPage {},
                        "View All Projects →"
                    }
                }
            }
        }
    }
}

/// Project Card (P9-C2)
#[component]
pub fn ProjectCard(project: Project, #[props(default = false)] featured: bool) -> Element {
    let card_class = if featured { 
        "mb-8".to_string() 
    } else { 
        "h-full".to_string() 
    };
    
    rsx! {
        Card { hover_effect: true, class: card_class,
            div { class: if featured { "lg:flex gap-8" } else { "" },
                // Image placeholder
                div { class: if featured { "lg:w-1/3 mb-4 lg:mb-0" } else { "mb-4" },
                    div { class: "aspect-video bg-gradient-to-br from-primary/20 via-bg-element to-secondary-purple/20 rounded-lg flex items-center justify-center border border-white/5",
                        span { class: "text-4xl font-display font-bold text-white/60", "{project.image_placeholder}" }
                    }
                }
                
                // Content
                div { class: if featured { "lg:w-2/3" } else { "" },
                    // Status badge
                    div { class: "flex items-center gap-2 mb-2",
                        Badge { color: project.status.color().to_string(), "{project.status.label()}" }
                    }
                    
                    // Title
                    h3 { class: "text-xl font-bold text-white mb-2 group-hover:text-primary transition-colors",
                        "{project.title}"
                    }
                    
                    // Description
                    p { class: "text-muted text-sm mb-4 line-clamp-2",
                        "{project.description}"
                    }
                    
                    // Technologies
                    div { class: "flex flex-wrap gap-2 mb-4",
                        for tech in project.technologies.iter() {
                            span { class: "text-xs px-2 py-1 bg-white/5 rounded text-secondary", "{tech}" }
                        }
                    }
                    
                    // Links
                    div { class: "flex gap-3",
                        if let Some(github) = project.github_url {
                            Button { 
                                variant: ButtonVariant::Ghost,
                                href: Some(github.to_string()),
                                new_tab: true,
                                "GitHub"
                            }
                        }
                        if let Some(demo) = project.demo_url {
                            Button { 
                                variant: ButtonVariant::Neon,
                                href: Some(demo.to_string()),
                                new_tab: true,
                                "Live Demo"
                            }
                        }
                    }
                }
            }
        }
    }
}
