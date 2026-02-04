//! Skills Section Component
//! Technical skills showcase with categories and progress bars

use dioxus::prelude::*;
use crate::components::molecules::{Card, SectionTitle, ProgressBar};
use crate::components::atoms::Badge;
use crate::components::layout_components::{Container, Section, Grid};

/// Skill Category
#[derive(Clone, PartialEq)]
pub enum SkillCategory {
    Languages,
    Frameworks,
    Tools,
    Concepts,
}

impl SkillCategory {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Languages => "Languages",
            Self::Frameworks => "Frameworks & Libraries",
            Self::Tools => "Tools & Platforms",
            Self::Concepts => "Concepts & Practices",
        }
    }
    
    pub fn color(&self) -> &'static str {
        match self {
            Self::Languages => "cyan",
            Self::Frameworks => "pink",
            Self::Tools => "purple",
            Self::Concepts => "orange",
        }
    }
}

/// Skill data structure (P8-A2)
#[derive(Clone, PartialEq)]
pub struct Skill {
    pub name: &'static str,
    pub level: u8,
    pub category: SkillCategory,
    pub featured: bool,
}

/// Get all skills data
fn get_skills() -> Vec<Skill> {
    vec![
        // Languages (P8-C1: Rust as main)
        Skill { name: "Rust", level: 90, category: SkillCategory::Languages, featured: true },
        Skill { name: "TypeScript", level: 85, category: SkillCategory::Languages, featured: true },
        Skill { name: "Python", level: 80, category: SkillCategory::Languages, featured: false },
        Skill { name: "JavaScript", level: 85, category: SkillCategory::Languages, featured: false },
        Skill { name: "HTML/CSS", level: 90, category: SkillCategory::Languages, featured: false },
        Skill { name: "SQL", level: 75, category: SkillCategory::Languages, featured: false },
        
        // Frameworks (P8-C2)
        Skill { name: "Dioxus", level: 85, category: SkillCategory::Frameworks, featured: true },
        Skill { name: "React/Next.js", level: 80, category: SkillCategory::Frameworks, featured: false },
        Skill { name: "Nuxt/Vue", level: 75, category: SkillCategory::Frameworks, featured: false },
        Skill { name: "Tailwind CSS", level: 90, category: SkillCategory::Frameworks, featured: false },
        Skill { name: "Actix Web", level: 70, category: SkillCategory::Frameworks, featured: false },
        
        // Tools (P8-C3)
        Skill { name: "Git/GitHub", level: 90, category: SkillCategory::Tools, featured: false },
        Skill { name: "Docker", level: 75, category: SkillCategory::Tools, featured: false },
        Skill { name: "Linux", level: 85, category: SkillCategory::Tools, featured: false },
        Skill { name: "VS Code", level: 95, category: SkillCategory::Tools, featured: false },
        Skill { name: "Figma", level: 70, category: SkillCategory::Tools, featured: false },
        
        // Concepts
        Skill { name: "Clean Architecture", level: 85, category: SkillCategory::Concepts, featured: false },
        Skill { name: "UI/UX Design", level: 80, category: SkillCategory::Concepts, featured: false },
        Skill { name: "Agile/Scrum", level: 75, category: SkillCategory::Concepts, featured: false },
    ]
}

/// Skills Section (P8-A1)
#[component]
pub fn SkillsSection() -> Element {
    let skills = get_skills();
    let featured_skills: Vec<_> = skills.iter().filter(|s| s.featured).cloned().collect();
    
    // Group by category
    let languages: Vec<_> = skills.iter().filter(|s| s.category == SkillCategory::Languages).cloned().collect();
    let frameworks: Vec<_> = skills.iter().filter(|s| s.category == SkillCategory::Frameworks).cloned().collect();
    let tools: Vec<_> = skills.iter().filter(|s| s.category == SkillCategory::Tools).cloned().collect();
    let concepts: Vec<_> = skills.iter().filter(|s| s.category == SkillCategory::Concepts).cloned().collect();

    rsx! {
        Section { id: "skills",
            Container {
                SectionTitle { 
                    text: "Skills & Expertise".to_string(), 
                    subtitle: "What I Work With".to_string(),
                    center: true 
                }
                
                // Featured Skills (P8-C1)
                div { class: "mb-16",
                    h3 { class: "text-xl font-bold text-center text-white mb-8",
                        span { class: "text-primary", "▸ " }
                        "Featured Technologies"
                    }
                    
                    div { class: "flex flex-wrap justify-center gap-4",
                        for skill in featured_skills.iter() {
                            FeaturedSkillBadge { skill: skill.clone() }
                        }
                    }
                }
                
                // Skills by Category (P8-A3)
                Grid { cols: 1, md_cols: 2, gap: 8,
                    SkillCategoryCard { 
                        category: SkillCategory::Languages, 
                        skills: languages 
                    }
                    SkillCategoryCard { 
                        category: SkillCategory::Frameworks, 
                        skills: frameworks 
                    }
                    SkillCategoryCard { 
                        category: SkillCategory::Tools, 
                        skills: tools 
                    }
                    SkillCategoryCard { 
                        category: SkillCategory::Concepts, 
                        skills: concepts 
                    }
                }
            }
        }
    }
}

/// Featured Skill Badge (P8-B1)
#[component]
fn FeaturedSkillBadge(skill: Skill) -> Element {
    let color = skill.category.color();
    
    rsx! {
        div { class: "group relative",
            div { class: "px-6 py-3 bg-bg-card border-2 border-primary/30 rounded-lg hover:border-primary hover:shadow-glow-sm transition-all cursor-default",
                span { class: "text-lg font-bold text-white", "{skill.name}" }
                span { class: "ml-2 text-sm text-primary", "{skill.level}%" }
            }
            
            // Hover tooltip (P8-B5)
            div { class: "absolute -bottom-2 left-1/2 -translate-x-1/2 translate-y-full opacity-0 group-hover:opacity-100 transition-opacity z-10 pointer-events-none",
                div { class: "bg-bg-card border border-white/10 rounded px-2 py-1 text-xs text-muted whitespace-nowrap",
                    "{skill.category.label()}"
                }
            }
        }
    }
}

/// Skill Category Card (P8-B1)
#[component]
fn SkillCategoryCard(category: SkillCategory, skills: Vec<Skill>) -> Element {
    let color = category.color();
    
    rsx! {
        Card { class: "animate-fade-in-up".to_string(),
            h4 { class: "text-lg font-bold text-white mb-6 flex items-center gap-2",
                Badge { color: color.to_string(), "{category.label()}" }
            }
            
            div { class: "space-y-4",
                for skill in skills.iter() {
                    ProgressBar { 
                        label: skill.name.to_string(), 
                        percentage: skill.level,
                        color: color.to_string()
                    }
                }
            }
        }
    }
}
