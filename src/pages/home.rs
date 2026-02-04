//! Home Page - Main landing page with all sections

use dioxus::prelude::*;
use crate::components::hero::HeroSection;
use crate::components::about::AboutSection;
use crate::components::skills::SkillsSection;
use crate::components::projects::ProjectsSection;
use crate::components::blog::BlogSection;
use crate::components::contact::ContactSection;
use crate::components::seo::SeoHead;

/// Home page component
#[component]
pub fn HomePage() -> Element {
    rsx! {
        // SEO Tags (P13)
        SeoHead {}
        
        // Hero Section (P6)
        HeroSection {}
        
        // About Section (P7)
        AboutSection {}
        
        // Skills Section (P8)
        SkillsSection {}
        
        // Projects Section (P9)
        ProjectsSection {}
        
        // Blog Section (P10)
        BlogSection {}
        
        // Contact Section (P11)
        ContactSection {}
    }
}
