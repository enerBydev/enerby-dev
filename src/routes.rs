//! Routes Module - Dioxus Router Configuration
//! Defines all application routes with cyberpunk portfolio structure

use dioxus::prelude::*;

use crate::pages::{
    home::HomePage,
    about::AboutPage,
    projects::{ProjectsPage, ProjectDetailPage},
    blog::{BlogPage, BlogPostPage},
    contact::ContactPage,
    not_found::NotFoundPage,
};
use crate::layouts::root::RootLayout;

/// Main application routes
#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    // All routes wrapped in RootLayout
    #[layout(RootLayout)]
        // Home page
        #[route("/")]
        HomePage {},
        
        // About page
        #[route("/about")]
        AboutPage {},
        
        // Projects section
        #[route("/projects")]
        ProjectsPage {},
        
        #[route("/projects/:slug")]
        ProjectDetailPage { slug: String },
        
        // Blog section
        #[route("/blog")]
        BlogPage {},
        
        #[route("/blog/:slug")]
        BlogPostPage { slug: String },
        
        // Contact page
        #[route("/contact")]
        ContactPage {},
    #[end_layout]
    
    // 404 Not Found (outside layout)
    #[route("/:..route")]
    NotFoundPage { route: Vec<String> },
}
