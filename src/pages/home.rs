//! Home Page - Main landing page with all sections

use crate::components::about::AboutSection;
use crate::components::blog::BlogSection;
use crate::components::contact::ContactSection;
use crate::components::hero::HeroSection;
use crate::components::projects::EnrichedProjectsSection;
use crate::components::seo::SeoHead;
use crate::components::skills::SkillsSection;
use dioxus::prelude::*;

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
        EnrichedProjectsSection {}

        // Blog Section (P10)
        BlogSection {}

        // Contact Section (P11)
        ContactSection {}
    }
}
