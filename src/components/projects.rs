//! Projects Section and Detail Components
//! Project showcase with filtering and detail views

use crate::components::atoms::{Badge, Button, ButtonVariant};
use crate::components::layout_components::{Container, Grid, Section};
use crate::components::molecules::{Card, SectionTitle};
use crate::routes::Route;
use crate::utils::github_api::parse_github_url;
use crate::utils::{ImageSource, get_project_image_url};
use crate::i18n::Language;
use dioxus::prelude::*;

/// Project Status (P9-A4)
#[derive(Clone, PartialEq, Copy, Debug)]
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
/// Updated for i18n (F18-C)
#[derive(Clone, PartialEq)]
pub struct Project {
    pub id: &'static str,
    pub title: &'static str,
    
    // Internal fields for localization - made pub for tests/construction
    pub description_en: &'static str,
    pub description_es: &'static str,
    
    pub long_description_en: &'static str,
    pub long_description_es: &'static str,
    
    pub technologies: Vec<&'static str>,
    pub status: ProjectStatus,
    pub github_url: Option<&'static str>,
    pub demo_url: Option<&'static str>,
    pub image_override: Option<&'static str>,
    pub image_fallback: &'static str,
}

impl Project {
    pub fn description(&self, lang: &Language) -> &'static str {
        match lang {
            Language::EN => self.description_en,
            Language::ES => self.description_es,
        }
    }

    pub fn long_description(&self, lang: &Language) -> &'static str {
        match lang {
            Language::EN => self.long_description_en,
            Language::ES => self.long_description_es,
        }
    }
}

/// Get all projects data - Proyectos Reales del Usuario
pub fn get_projects() -> Vec<Project> {
    vec![
        // Featured - Core Portfolio
        Project {
            id: "enerby-dev",
            title: "enerby.dev",
            
            description_en: "Personal portfolio built with Rust and Dioxus. Cyberpunk themed with modern web technologies.",
            description_es: "Portafolio personal construido con Rust y Dioxus. Temática cyberpunk con tecnologías web modernas.",
            
            long_description_en: "Full-featured portfolio showcasing my work, skills, and blog. Built entirely in Rust using Dioxus, localized with a static-first i18n approach, neon aesthetic, glassmorphism, and smooth animations.",
            long_description_es: "Portafolio completo mostrando mi trabajo, habilidades y blog. Construido enteramente en Rust usando Dioxus, localizado con enfoque i18n estático, estética neón, glassmorphism y animaciones suaves.",
            
            technologies: vec!["Rust", "Dioxus", "WebAssembly", "Tailwind CSS"],
            status: ProjectStatus::Featured,
            github_url: Some("https://github.com/enerBydev/enerby-dev"),
            demo_url: Some("https://enerbydev.pages.dev"),
            image_override: None,
            image_fallback: "🦀",
        },
        // Featured - Rust CLI Tool
        Project {
            id: "oc-diagdoc",
            title: "oc_diagdoc",
            
            description_en: "Advanced CLI for technical documentation. Includes integral verification and real-time dashboard.",
            description_es: "CLI avanzado para documentación técnica. Incluye verificación integral y dashboard en tiempo real.",
            
            long_description_en: "Advanced CLI tool for extensive technical documentation projects. Features integral verification, real-time stats dashboard, hierarchical visualization, dependency analysis, and quantum diagnostics with auto-repair.",
            long_description_es: "Herramienta CLI avanzada para proyectos de documentación técnica extensos. Cuenta con verificación integral, panel de estadísticas en tiempo real, visualización jerárquica, análisis de dependencias y diagnóstico cuántico con auto-reparación.",
            
            technologies: vec!["Rust", "CLI", "WalkDir", "Serde", "Clap"],
            status: ProjectStatus::Featured,
            github_url: Some("https://github.com/enerBydev/oc_diagdoc"),
            demo_url: Some("https://www.google.com"),
            image_override: None,
            image_fallback: "⚛️",
        },
        // Featured - Linux Tool
        Project {
            id: "affinity-legacy-bridge",
            title: "Affinity Legacy Bridge",
            
            description_en: "Bridge to run Affinity on Linux LTS using Bottles/Flatpak. Solution for systems with old GLIBC.",
            description_es: "Puente para ejecutar Affinity en Linux LTS usando Bottles/Flatpak. Solución para sistemas con GLIBC antiguo.",
            
            long_description_en: "Installation kit using Bottles (Flatpak) as a bridge to run Affinity Photo/Designer/Publisher on Linux LTS systems with GLIBC 2.35. Contains isolated modern libraries (GLIBC 2.42+) without breaking the host system.",
            long_description_es: "Kit de instalación que usa Bottles (Flatpak) como puente para ejecutar Affinity Photo/Designer/Publisher en sistemas Linux LTS con GLIBC 2.35. Contiene bibliotecas aisladas modernas (GLIBC 2.42+) sin romper el sistema.",
            
            technologies: vec!["Shell", "Flatpak", "Bottles", "Wine", "Linux"],
            status: ProjectStatus::Featured,
            github_url: Some("https://github.com/enerBydev/Affinity-Legacy-Bridge"),
            demo_url: None,
            image_override: None,
            image_fallback: "🎨",
        },
        // Active - AI Project
        Project {
            id: "videoginiusai",
            title: "VideoGIniusAI",
            
            description_en: "AI-powered video analysis and generation platform. Content automation with Nuxt 3.",
            description_es: "Plataforma de edición de video potenciada por IA. Creación de contenido automatizada con Nuxt 3.",
            
            long_description_en: "Web application for video generation/edition using AI. Built with Nuxt 3, integrates AI models to automate multimedia content creation.",
            long_description_es: "Aplicación web para generación y edición de video usando inteligencia artificial. Construida con Nuxt 3, integra modelos de IA para automatizar la creación de contenido multimedia.",
            
            technologies: vec!["Nuxt 4", "Vue.js", "AI", "TypeScript"],
            status: ProjectStatus::Active,
            github_url: Some("https://github.com/enerBydev/VideoGIniusAI"),
            demo_url: None,
            image_override: None,
            image_fallback: "🎬",
        },
    ]
}

/// Get project by ID
pub fn get_project_by_id(id: &str) -> Option<Project> {
    get_projects().into_iter().find(|p| p.id == id)
}

// ============================================================================
// GITHUB ENRICHMENT (F17 - Phase D)
// ============================================================================

/// Enriched project with dynamic demo_url from GitHub
/// This struct owns its strings for dynamic data and i18n support
#[derive(Clone, PartialEq)]
pub struct EnrichedProject {
    pub id: String,
    pub title: String,
    
    // Localized fields
    pub description_en: String,
    pub description_es: String,
    
    pub long_description_en: String,
    pub long_description_es: String,
    
    pub technologies: Vec<String>,
    pub status: ProjectStatus,
    pub github_url: Option<String>,
    /// Demo URL - either manual or auto-detected from GitHub homepage
    pub demo_url: Option<String>,
    pub image_override: Option<String>,
    pub image_fallback: String,
    /// Source of demo_url for debugging/UI
    pub demo_url_source: DemoUrlSource,
}

impl EnrichedProject {
    pub fn description(&self, lang: &Language) -> &str {
        match lang {
            Language::EN => &self.description_en,
            Language::ES => &self.description_es,
        }
    }

    pub fn long_description(&self, lang: &Language) -> &str {
        match lang {
            Language::EN => &self.long_description_en,
            Language::ES => &self.long_description_es,
        }
    }
}

/// Indicates where the demo_url came from
#[derive(Clone, PartialEq, Debug)]
pub enum DemoUrlSource {
    /// Manually set in project definition
    Manual,
    /// Auto-detected from GitHub repository homepage field
    GitHub,
    /// No demo URL available
    None,
}

impl From<Project> for EnrichedProject {
    fn from(p: Project) -> Self {
        Self {
            id: p.id.to_string(),
            title: p.title.to_string(),
            
            description_en: p.description_en.to_string(),
            description_es: p.description_es.to_string(),
            
            long_description_en: p.long_description_en.to_string(),
            long_description_es: p.long_description_es.to_string(),
            
            technologies: p.technologies.iter().map(|t| t.to_string()).collect(),
            status: p.status,
            github_url: p.github_url.map(|s| s.to_string()),
            demo_url: p.demo_url.map(|s| s.to_string()),
            image_override: p.image_override.map(|s| s.to_string()),
            image_fallback: p.image_fallback.to_string(),
            demo_url_source: if p.demo_url.is_some() { DemoUrlSource::Manual } else { DemoUrlSource::None },
        }
    }
}

/// Enriches a project with GitHub data
///
/// If the project has a github_url but no demo_url, attempts to fetch
/// the homepage field from the GitHub repository.
///
/// # Arguments
/// * `project` - The project to enrich
///
/// # Returns
/// An EnrichedProject with demo_url potentially filled from GitHub
pub fn enrich_project_with_github(project: Project) -> EnrichedProject {
    let mut enriched = EnrichedProject::from(project.clone());

    // DEBUG PROBE REMOVED

    // If demo_url already exists (manually set), keep it
    if enriched.demo_url.is_some() {
        return enriched;
    }

    // Try to get homepage from GitHub
    if let Some(github_url) = &project.github_url {
        if let Some((owner, repo)) = parse_github_url(github_url) {
            // Trace logs
            println!("enriching: {}", &project.id);
            if let Ok(repo_info) = crate::utils::github_api::get_repo_info(&owner, &repo) {
                println!("found repo: {}", &repo_info.full_name);

                // Enrich Homepage
                if let Some(homepage) = repo_info.extract_homepage() {
                    println!("updating demo_url: {}", &homepage);
                    enriched.demo_url = Some(homepage);
                    enriched.demo_url_source = DemoUrlSource::GitHub;
                }

                // Enrich Description (if available)
                if let Some(desc) = &repo_info.description {
                    println!("checking description: '{}'", desc);
                    if !desc.is_empty() {
                        println!("updating description");
                        // GitHub descriptions are typically in English (or the repo's primary language)
                        // We update the EN field and optionally sync ES or leave it
                        enriched.description_en = desc.clone();
                        // For now, if we get a better description from GitHub, we might want to use it for ES too 
                        // if the original was empty or placeholder, but to be safe let's just update EN.
                        // Or better: update both if we assume the GitHub desc is the "canonical" one.
                        // Strategy: Update EN, leave ES as is (manual translation preserved).
                    }
                }

                // Enrich Technologies/Topics (if available)
                if !repo_info.topics.is_empty() {
                    println!("merging topics: {:?}", &repo_info.topics);
                    let mut new_techs = repo_info.topics.clone();
                    for tech in &enriched.technologies {
                        // Avoid duplicates (case-insensitive check)
                        if !new_techs.iter().any(|t| t.eq_ignore_ascii_case(tech)) {
                            new_techs.push(tech.clone());
                        }
                    }
                    enriched.technologies = new_techs;
                }
            } else {
                println!("repo not found: {}/{}", owner, repo);
            }
        }
    }

    enriched
}

/// Get all projects enriched with GitHub data
///
/// This function:
/// 1. Loads all static projects
/// 2. For each project without demo_url, checks GitHub for homepage
/// 3. Returns enriched projects with auto-detected demo URLs
pub fn get_projects_enriched() -> Vec<EnrichedProject> {
    get_projects()
        .into_iter()
        .map(enrich_project_with_github)
        .collect()
}

/// Get enriched project by ID
pub fn get_enriched_project_by_id(id: &str) -> Option<EnrichedProject> {
    get_project_by_id(id).map(enrich_project_with_github)
}

/// Statistics about GitHub enrichment
#[derive(Debug, Clone)]
pub struct EnrichmentStats {
    pub total_projects: usize,
    pub manual_demo_urls: usize,
    pub github_demo_urls: usize,
    pub no_demo_url: usize,
}

/// Get enrichment statistics
pub fn get_enrichment_stats() -> EnrichmentStats {
    let enriched = get_projects_enriched();
    let total = enriched.len();
    let manual = enriched
        .iter()
        .filter(|p| p.demo_url_source == DemoUrlSource::Manual)
        .count();
    let github = enriched
        .iter()
        .filter(|p| p.demo_url_source == DemoUrlSource::GitHub)
        .count();
    let none = enriched
        .iter()
        .filter(|p| p.demo_url_source == DemoUrlSource::None)
        .count();

    EnrichmentStats {
        total_projects: total,
        manual_demo_urls: manual,
        github_demo_urls: github,
        no_demo_url: none,
    }
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
    let i18n = crate::i18n::use_i18n();

    rsx! {
        Section { id: "projects", alternate: true,
            Container {
                SectionTitle {
                    text: i18n.projects.title.to_string(),
                    subtitle: i18n.projects.subtitle.to_string(),
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
                        "{i18n.projects.view_all}"
                    }
                }
            }
        }
    }
}

use crate::i18n::I18N_CONFIG;

/// Project Card (P9-C2)
#[component]
pub fn ProjectCard(project: Project, #[props(default = false)] featured: bool) -> Element {
    let lang = I18N_CONFIG.read().language;
    let i18n = crate::i18n::use_i18n();
    let card_class = if featured {
        "mb-8".to_string()
    } else {
        "h-full".to_string()
    };

    // Get the best available image source for this project
    let image_source = get_project_image_url(&project);
    let fallback_emoji = project.image_fallback.to_string();

    rsx! {
        Card { hover_effect: true, class: card_class,
            div { class: if featured { "lg:flex gap-8" } else { "" },
                // Project Image - dynamic loading with fallback
                div { class: if featured { "lg:w-1/3 mb-4 lg:mb-0" } else { "mb-4" },
                    div { class: "aspect-video bg-gradient-to-br from-primary/20 via-bg-element to-secondary-purple/20 rounded-lg flex items-center justify-center border border-white/5 overflow-hidden relative",
                        match &image_source {
                            ImageSource::Fallback(emoji) => rsx! {
                                span { class: "text-4xl font-display font-bold text-white/60", "{emoji}" }
                            },
                            _ => rsx! {
                                // Fallback emoji shown behind image (visible if image fails to load)
                                span {
                                    class: "absolute inset-0 flex items-center justify-center text-4xl font-display font-bold text-white/60",
                                    "{fallback_emoji}"
                                }
                                img {
                                    src: "{image_source}",
                                    alt: "{project.title}",
                                    class: "relative w-full h-full object-cover",
                                    loading: "lazy",
                                }
                            }
                        }
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
                        "{project.description(&lang)}"
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
                                "{i18n.projects.btn_github}"
                            }
                        }
                        if let Some(demo) = project.demo_url {
                            Button {
                                variant: ButtonVariant::Neon,
                                href: Some(demo.to_string()),
                                new_tab: true,
                                "{i18n.projects.btn_demo}"
                            }
                        }
                    }
                }
            }
        }
    }
}

// ============================================================================
// ENRICHED PROJECT CARD (F17 - Phase F)
// ============================================================================

/// Enriched Project Card with GitHub demo_url source indicator
///
/// This component uses EnrichedProject and shows:
/// - Visual indicator (GitHub icon) if demo_url came from GitHub
/// - Tooltip explaining the source
#[component]
pub fn EnrichedProjectCard(
    project: EnrichedProject,
    #[props(default = false)] featured: bool,
) -> Element {
    let lang = I18N_CONFIG.read().language;
    let i18n = crate::i18n::use_i18n();
    let card_class = if featured {
        "mb-8".to_string()
    } else {
        "h-full".to_string()
    };

    // Create a temporary Project for image lookup
    let temp_project = Project {
        id: Box::leak(project.id.clone().into_boxed_str()),
        title: Box::leak(project.title.clone().into_boxed_str()),
        
        description_en: Box::leak(project.description_en.clone().into_boxed_str()),
        description_es: Box::leak(project.description_es.clone().into_boxed_str()),
        
        long_description_en: Box::leak(project.long_description_en.clone().into_boxed_str()),
        long_description_es: Box::leak(project.long_description_es.clone().into_boxed_str()),
        
        technologies: project
            .technologies
            .iter()
            .map(|s| -> &'static str { Box::leak(s.clone().into_boxed_str()) })
            .collect(),
        status: project.status,
        github_url: project
            .github_url
            .clone()
            .map(|s| -> &'static str { Box::leak(s.into_boxed_str()) }),
        demo_url: project
            .demo_url
            .clone()
            .map(|s| -> &'static str { Box::leak(s.into_boxed_str()) }),
        image_override: project
            .image_override
            .clone()
            .map(|s| -> &'static str { Box::leak(s.into_boxed_str()) }),
        image_fallback: Box::leak(project.image_fallback.clone().into_boxed_str()),
    };

    let image_source = get_project_image_url(&temp_project);
    let fallback_emoji = project.image_fallback.to_string();

    // Demo URL button label and tooltip based on source
    // Demo URL button label and tooltip based on source
    let (demo_label, demo_tooltip): (String, Option<&'static str>) = match project.demo_url_source {
        DemoUrlSource::GitHub => (format!("{} 🔗", i18n.projects.btn_demo), Some(i18n.projects.tooltip_auto)),
        DemoUrlSource::Manual => (i18n.projects.btn_demo.to_string(), None),
        DemoUrlSource::None => ("".to_string(), None),
    };

    rsx! {
        Card { hover_effect: true, class: card_class,
            div { class: if featured { "lg:flex gap-8" } else { "" },
                // Project Image - dynamic loading with fallback
                div { class: if featured { "lg:w-1/3 mb-4 lg:mb-0" } else { "mb-4" },
                    div { class: "aspect-video bg-gradient-to-br from-primary/20 via-bg-element to-secondary-purple/20 rounded-lg flex items-center justify-center border border-white/5 overflow-hidden relative",
                        match &image_source {
                            ImageSource::Fallback(emoji) => rsx! {
                                span { class: "text-4xl font-display font-bold text-white/60", "{emoji}" }
                            },
                            _ => rsx! {
                                // Fallback emoji shown behind image (visible if image fails to load)
                                span {
                                    class: "absolute inset-0 flex items-center justify-center text-4xl font-display font-bold text-white/60",
                                    "{fallback_emoji}"
                                }
                                img {
                                    src: "{image_source}",
                                    alt: "{project.title}",
                                    class: "relative w-full h-full object-cover",
                                    loading: "lazy",
                                }
                            }
                        }
                        // Demo Source Indicator
                        if project.demo_url_source == DemoUrlSource::GitHub {
                            div {
                                class: "absolute top-2 right-2 bg-black/80 text-white text-xs px-2 py-1 rounded flex items-center gap-1 border border-white/10 shadow-lg",
                                title: "{i18n.projects.tooltip_auto}",
                                span { "🔍" }
                                "{i18n.projects.btn_demo_github}"
                            }
                        }
                    }
                }

                // Content
                div { class: if featured { "lg:w-2/3" } else { "" },
                    // Status badge + GitHub indicator
                    div { class: "flex items-center gap-2 mb-2",
                        Badge { color: project.status.color().to_string(), "{project.status.label()}" }
                        // GitHub auto-detection indicator
                        if project.demo_url_source == DemoUrlSource::GitHub {
                            span {
                                class: "text-xs px-2 py-0.5 bg-green-500/20 text-green-400 rounded-full flex items-center gap-1",
                                title: "{i18n.projects.tooltip_auto}",
                                "🔗 {i18n.projects.badge_auto}"
                            }
                        }
                    }

                    // Title
                    h3 { class: "text-xl font-bold text-white mb-2 group-hover:text-primary transition-colors",
                        "{project.title}"
                    }

                    // Description
                    p { class: "text-muted text-sm mb-4 line-clamp-2",
                        "{project.description(&lang)}"
                    }

                    // Technologies
                    div { class: "flex flex-wrap gap-2 mb-4",
                        for tech in project.technologies.iter() {
                            span { class: "text-xs px-2 py-1 bg-white/5 rounded text-secondary", "{tech}" }
                        }
                    }

                    // Links
                    div { class: "flex gap-3 items-center",
                        if let Some(github) = &project.github_url {
                            Button {
                                variant: ButtonVariant::Ghost,
                                href: Some(github.clone()),
                                new_tab: true,
                                "{i18n.projects.btn_github}"
                            }
                        }
                        if let Some(demo) = &project.demo_url {
                            div {
                                class: "relative group/demo",
                                title: demo_tooltip.map(|s| s.to_string()).unwrap_or("".to_string()),
                                Button {
                                    variant: ButtonVariant::Neon,
                                    href: Some(demo.clone()),
                                    new_tab: true,
                                    "{demo_label}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Projects Section using Enriched Projects (F17)
///
/// Uses EnrichedProjectCard to show GitHub-sourced demo URLs with visual indicator
#[component]
pub fn EnrichedProjectsSection() -> Element {
    let projects = get_projects_enriched();
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
    let i18n = crate::i18n::use_i18n();

    rsx! {
        Section { id: "projects", alternate: true,
            Container {
                SectionTitle {
                    text: i18n.projects.title.to_string(),
                    subtitle: i18n.projects.subtitle.to_string(),
                    center: true
                }

                // Featured Projects
                if !featured.is_empty() {
                    div { class: "mb-12",
                        h3 { class: "text-lg font-semibold text-primary mb-6", "{i18n.projects.section_featured}" }
                        div { class: "space-y-6",
                            for project in featured {
                                EnrichedProjectCard { project: project.clone(), featured: true }
                            }
                        }
                    }
                }

                // Active Projects Grid
                if !active.is_empty() {
                    div {
                        h3 { class: "text-lg font-semibold text-secondary mb-6", "{i18n.projects.section_active}" }
                        Grid { cols: 2, gap: 6,
                            for project in active {
                                EnrichedProjectCard { project: project.clone() }
                            }
                        }
                    }
                }

                // View all projects link
                div { class: "mt-8 text-center",
                    Link { to: Route::ProjectsPage {},
                        Button { variant: ButtonVariant::Secondary, "{i18n.projects.view_all}" }
                    }
                }
            }
        }
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enriched_project_from_project() {
        let project = Project {
            id: "test",
            title: "Test Project",
            description_en: "A test",
            description_es: "A test",
            long_description_en: "A longer test",
            long_description_es: "A longer test",
            technologies: vec!["Rust"],
            status: ProjectStatus::Active,
            github_url: Some("https://github.com/owner/repo"),
            demo_url: Some("https://demo.com"),
            image_override: None,
            image_fallback: "🧪",
        };

        let enriched = EnrichedProject::from(project);

        assert_eq!(enriched.id, "test");
        assert_eq!(enriched.title, "Test Project");
        assert_eq!(enriched.demo_url, Some("https://demo.com".to_string()));
        assert_eq!(enriched.demo_url_source, DemoUrlSource::Manual);
    }

    #[test]
    fn test_enriched_project_no_demo_url() {
        let project = Project {
            id: "test",
            title: "Test",
            description_en: "Test",
            description_es: "Test",
            long_description_en: "Test",
            long_description_es: "Test",
            technologies: vec![],
            status: ProjectStatus::Active,
            github_url: Some("https://github.com/owner/repo"),
            demo_url: None,
            image_override: None,
            image_fallback: "🧪",
        };

        let enriched = EnrichedProject::from(project);

        assert_eq!(enriched.demo_url, None);
        assert_eq!(enriched.demo_url_source, DemoUrlSource::None);
    }

    #[test]
    fn test_enrich_project_keeps_manual_demo_url() {
        let project = Project {
            id: "test",
            title: "Test",
            description_en: "Test",
            description_es: "Test",
            long_description_en: "Test",
            long_description_es: "Test",
            technologies: vec![],
            status: ProjectStatus::Active,
            github_url: Some("https://github.com/enerBydev/enerby.dev"),
            demo_url: Some("https://manual.demo"),
            image_override: None,
            image_fallback: "🧪",
        };

        let enriched = enrich_project_with_github(project);

        // Should keep manual demo_url, not replace with GitHub
        assert_eq!(enriched.demo_url, Some("https://manual.demo".to_string()));
        assert_eq!(enriched.demo_url_source, DemoUrlSource::Manual);
    }

    #[test]
    fn test_enrich_project_gets_github_homepage() {
        // enerby.dev has homepage in static data
        let project = Project {
            id: "test",
            title: "Test",
            description_en: "Test",
            description_es: "Test",
            long_description_en: "Test",
            long_description_es: "Test",
            technologies: vec![],
            status: ProjectStatus::Active,
            github_url: Some("https://github.com/enerBydev/enerby.dev"),
            demo_url: None, // No manual demo_url
            image_override: None,
            image_fallback: "🧪",
        };

        let enriched = enrich_project_with_github(project);

        // Should get demo_url from GitHub static data
        assert_eq!(enriched.demo_url, Some("https://enerby.dev".to_string()));
        assert_eq!(enriched.demo_url_source, DemoUrlSource::GitHub);
    }

    #[test]
    fn test_enrich_project_no_homepage_in_github() {
        // Case: Project has github_url, but repo has NO homepage (nvim-config)
        let project = Project {
            id: "test",
            title: "Test",
            description_en: "Test",
            description_es: "Test",
            long_description_en: "Test",
            long_description_es: "Test",
            technologies: vec![],
            status: ProjectStatus::Active,
            github_url: Some("https://github.com/enerBydev/nvim-config"),
            demo_url: None,
            image_override: None,
            image_fallback: "🧪",
        };

        let enriched = enrich_project_with_github(project);

        // Should remain None since nvim-config has no homepage
        assert_eq!(enriched.demo_url, None);
        assert_eq!(enriched.demo_url_source, DemoUrlSource::None);
    }

    #[test]
    fn test_get_projects_enriched() {
        let enriched = get_projects_enriched();

        // Should have same count as regular projects
        assert_eq!(enriched.len(), get_projects().len());

        // All should be EnrichedProject
        for p in &enriched {
            assert!(!p.id.is_empty());
            assert!(!p.title.is_empty());
        }
    }

    #[test]
    fn test_get_enriched_project_by_id() {
        let result = get_enriched_project_by_id("enerby-dev");
        assert!(result.is_some());

        let project = result.unwrap();
        assert_eq!(project.id, "enerby-dev");
    }

    #[test]
    fn test_get_enriched_project_by_id_not_found() {
        let result = get_enriched_project_by_id("nonexistent");
        assert!(result.is_none());
    }

    #[test]
    fn test_enrichment_stats() {
        let stats = get_enrichment_stats();

        assert!(stats.total_projects > 0);
        assert_eq!(
            stats.total_projects,
            stats.manual_demo_urls + stats.github_demo_urls + stats.no_demo_url
        );
    }

    #[test]
    fn test_demo_url_source_equality() {
        assert_eq!(DemoUrlSource::Manual, DemoUrlSource::Manual);
        assert_ne!(DemoUrlSource::Manual, DemoUrlSource::GitHub);
        assert_ne!(DemoUrlSource::GitHub, DemoUrlSource::None);
    }

    // ==========================================================================
    // INTEGRATION TESTS (G2) - End-to-end enrichment with real repos
    // ==========================================================================

    #[test]
    fn test_integration_enerby_dev_has_demo_url() {
        // enerby-dev project should have demo_url (either manual or from GitHub)
        let enriched = get_enriched_project_by_id("enerby-dev");
        assert!(enriched.is_some(), "enerby-dev project should exist");

        let project = enriched.unwrap();
        assert!(
            project.demo_url.is_some(),
            "enerby-dev should have a demo_url"
        );

        // The demo URL should be the actual website
        let demo = project.demo_url.unwrap();
        assert!(
            demo.contains("enerby") || demo.contains("pages.dev"),
            "Demo URL should be enerby.dev or pages.dev: {}",
            demo
        );
    }

    #[test]
    fn test_integration_enrichment_flow_complete() {
        // Test the complete enrichment flow from static projects to enriched
        let static_projects = get_projects();
        let enriched_projects = get_projects_enriched();

        // Count should match
        assert_eq!(static_projects.len(), enriched_projects.len());

        // Each project should have preserved its original data
        for (static_p, enriched_p) in static_projects.iter().zip(enriched_projects.iter()) {
            assert_eq!(static_p.id, enriched_p.id);
            assert_eq!(static_p.title, enriched_p.title);
            assert_eq!(static_p.status, enriched_p.status);
        }
    }

    #[test]
    fn test_integration_manual_demo_priority() {
        // Verify that manual demo_url is never overwritten by GitHub
        let enriched = get_projects_enriched();

        for project in &enriched {
            if project.demo_url_source == DemoUrlSource::Manual {
                // Found a project with manual demo - verify it wasn't replaced
                let original = get_project_by_id(&project.id);
                if let Some(orig) = original {
                    if let Some(orig_demo) = orig.demo_url {
                        assert_eq!(
                            project.demo_url,
                            Some(orig_demo.to_string()),
                            "Manual demo_url should be preserved for {}",
                            project.id
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn test_integration_no_orphan_github_sources() {
        // A GitHub source should only exist if there's actually a demo_url
        let enriched = get_projects_enriched();

        for project in &enriched {
            if project.demo_url_source == DemoUrlSource::GitHub {
                assert!(
                    project.demo_url.is_some(),
                    "GitHub source but no demo_url for {}",
                    project.id
                );
            }
            if project.demo_url_source == DemoUrlSource::None {
                assert!(
                    project.demo_url.is_none(),
                    "None source but has demo_url for {}",
                    project.id
                );
            }
        }
    }

    #[test]
    fn test_integration_all_projects_have_valid_structure() {
        let enriched = get_projects_enriched();
        use crate::i18n::Language;

        for project in &enriched {
            // Basic validation
            assert!(!project.id.is_empty(), "Project ID should not be empty");
            assert!(
                !project.title.is_empty(),
                "Project title should not be empty"
            );
            assert!(
                !project.description(&Language::EN).is_empty(),
                "Project description (EN) should not be empty"
            );
            assert!(
                !project.image_fallback.is_empty(),
                "Fallback emoji should exist"
            );

            // If has github_url, should be valid
            if let Some(ref url) = project.github_url {
                assert!(
                    url.starts_with("https://github.com/"),
                    "GitHub URL invalid for {}",
                    project.id
                );
            }

            // If has demo_url, should be valid URL
            if let Some(ref url) = project.demo_url {
                assert!(
                    url.starts_with("http://") || url.starts_with("https://"),
                    "Invalid demo URL: {}",
                    url
                );
            }
        }
    }
}
