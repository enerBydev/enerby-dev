//! Dictionary Types
//! Defines the shape of the translation data.
//! This ensures that all languages implement the exact same fields.

#[derive(Debug, Clone, PartialEq)]
pub struct Dictionary {
    pub nav: NavConfig,
    pub hero: HeroConfig,
    pub about: AboutConfig,
    pub footer: FooterConfig,
    pub projects: ProjectsConfig,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NavConfig {
    pub home: &'static str,
    pub projects: &'static str,
    pub blog: &'static str,
    pub about: &'static str,
    pub contact: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HeroConfig {
    pub greeting: &'static str,
    pub role_prefix: &'static str,
    pub description_start: &'static str,
    pub description_highlight: &'static str,
    pub description_end: &'static str,
    pub cta_projects: &'static str,
    pub cta_contact: &'static str,
    pub scroll_indicator: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AboutConfig {
    pub title: &'static str,
    pub subtitle: &'static str,
    pub bio_p1_start: &'static str,
    pub bio_p1_highlight: &'static str,
    pub bio_p1_end: &'static str,
    pub bio_p2: &'static str,
    pub stat_experience: &'static str,
    pub stat_projects: &'static str,
    pub stat_technologies: &'static str,
    pub stat_loc: &'static str,
    pub timeline_title: &'static str,
    pub timeline: &'static [TimelineItemData],
}

#[derive(Debug, Clone, PartialEq)]
pub struct TimelineItemData {
    pub year: &'static str,
    pub title: &'static str,
    pub company: &'static str,
    pub description: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FooterConfig {
    pub built_with: &'static str,
    pub and: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProjectsConfig {
    pub title: &'static str,
    pub subtitle: &'static str,
    pub section_featured: &'static str,
    pub section_active: &'static str,
    pub view_all: &'static str,
    pub btn_github: &'static str,
    pub btn_demo: &'static str,
    pub btn_demo_github: &'static str, // "GitHub Demo"
    pub badge_auto: &'static str, // "Auto"
    pub tooltip_auto: &'static str, 
}
