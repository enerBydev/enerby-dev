//! Projects Section and Detail Components
//! Project showcase with filtering and detail views

use crate::components::atoms::{Badge, Button, ButtonVariant};
use crate::components::layout_components::{Container, Grid, Section};
use crate::components::molecules::{Card, SectionTitle};
use crate::routes::Route;
use dioxus::prelude::*;

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

/// Get all projects data - Proyectos Reales del Usuario
pub fn get_projects() -> Vec<Project> {
    vec![
        // Featured GitHub Projects
        Project {
            id: "enerby-dev",
            title: "enerby.dev",
            description: "Portfolio personal construido con Rust y Dioxus. Temática cyberpunk con tecnologías web modernas.",
            long_description: "Portfolio full-featured mostrando mi trabajo, skills y blog. Construido enteramente en Rust usando Dioxus, con estética cyberpunk, colores neon, glassmorphism y animaciones suaves.",
            technologies: vec!["Rust", "Dioxus", "WebAssembly", "Tailwind CSS"],
            status: ProjectStatus::Featured,
            github_url: Some("https://github.com/enerBydev/enerby.dev"),
            demo_url: Some("https://enerby.dev"),
            image_placeholder: "🦀",
        },
        Project {
            id: "rust-projects",
            title: "Rust Projects Collection",
            description: "Colección de proyectos Rust incluyendo CLI tools, APIs y experimentos con WebAssembly.",
            long_description: "Repositorio monorepo con múltiples proyectos Rust, desde herramientas CLI de productividad hasta APIs REST con Axum y experimentos con Dioxus y WebAssembly.",
            technologies: vec!["Rust", "Axum", "SQLx", "Tokio", "Dioxus"],
            status: ProjectStatus::Featured,
            github_url: Some("https://github.com/enerBydev/rust_projects"),
            demo_url: None,
            image_placeholder: "⚙️",
        },
        // Featured External Projects (Placeholders)
        Project {
            id: "nuxt-saas-starter",
            title: "Nuxt SaaS Starter",
            description: "Template SaaS completo con Nuxt 3, Supabase Auth, Stripe y Capacitor para apps móviles.",
            long_description: "Starter kit enterprise para construir aplicaciones SaaS con Nuxt 3. Incluye autenticación con Supabase, pagos con Stripe, deployado en Cloudflare y empaquetado móvil con Capacitor.",
            technologies: vec!["Nuxt 3", "Vue.js", "Supabase", "Stripe", "Capacitor"],
            status: ProjectStatus::Featured,
            github_url: None,
            demo_url: Some("https://youtube.com"),
            image_placeholder: "🚀",
        },
        Project {
            id: "onlycar-platform",
            title: "OnlyCar Platform",
            description: "Plataforma web tipo Uber para servicio de transporte privado con arquitectura Hexagonal y DDD.",
            long_description: "Sistema completo de transporte privado con arquitectura Hexagonal, Clean Architecture y DDD. Frontend en Nuxt 3, backend en Rust/Axum, base de datos PostgreSQL con Supabase.",
            technologies: vec!["Nuxt 3", "Rust", "Axum", "Supabase", "Cloudflare"],
            status: ProjectStatus::Featured,
            github_url: None,
            demo_url: Some("https://facebook.com"),
            image_placeholder: "🚗",
        },
        // Active Projects
        Project {
            id: "dioxus-components",
            title: "Dioxus Component Library",
            description: "Librería de componentes UI reutilizables para Dioxus con Atomic Design y Storybook.",
            long_description: "Design System completo para Dioxus siguiendo Atomic Design. Incluye átomos, moléculas y organismos con documentación interactiva y visual testing.",
            technologies: vec!["Rust", "Dioxus", "Tailwind CSS", "Storybook"],
            status: ProjectStatus::Active,
            github_url: Some("https://github.com/enerBydev/dioxus-components"),
            demo_url: None,
            image_placeholder: "🎨",
        },
        Project {
            id: "api-gateway",
            title: "Rust API Gateway",
            description: "Gateway API de alto rendimiento con rate limiting, caching y middleware personalizado.",
            long_description: "API Gateway construido con Axum y Tower, featuring rate limiting inteligente, caching Redis, autenticación JWT y middleware extensible.",
            technologies: vec!["Rust", "Axum", "Tower", "Redis", "JWT"],
            status: ProjectStatus::Active,
            github_url: Some("https://github.com/enerBydev/api-gateway"),
            demo_url: Some("https://google.com"),
            image_placeholder: "🔐",
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
    let featured: Vec<_> = projects
        .iter()
        .filter(|p| p.status == ProjectStatus::Featured)
        .take(3)
        .cloned()
        .collect();
    let active: Vec<_> = projects
        .iter()
        .filter(|p| p.status == ProjectStatus::Active)
        .cloned()
        .collect();

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
