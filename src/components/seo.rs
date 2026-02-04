//! SEO Module
//! Components for meta tags, Open Graph, and Twitter Cards

use dioxus::prelude::*;
use crate::config::SITE;

/// Page metadata for SEO
#[derive(Clone, Default)]
pub struct PageMeta {
    pub title: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
    pub url: Option<String>,
    pub page_type: Option<String>,
}

/// Full SEO Head component (P13-A, P13-B, P13-C)
#[component]
pub fn SeoHead(
    #[props(default = None)] title: Option<String>,
    #[props(default = None)] description: Option<String>,
    #[props(default = None)] image: Option<String>,
    #[props(default = None)] canonical: Option<String>,
    #[props(default = "website".to_string())] og_type: String,
) -> Element {
    // Build full title (P13-A1)
    let full_title = match title {
        Some(ref t) => format!("{} | {}", t, SITE.title),
        None => SITE.title.to_string(),
    };
    
    // Use provided or default description (P13-A2)
    let meta_description = description.unwrap_or_else(|| SITE.description.to_string());
    
    // Default OG image
    let og_image = image.unwrap_or_else(|| format!("{}/og-image.png", SITE.base_url));
    
    // Canonical URL (P13-A3)
    let canonical_url = canonical.unwrap_or_else(|| SITE.base_url.to_string());

    rsx! {
        // Basic Meta Tags (P13-A)
        document::Title { "{full_title}" }
        document::Meta { name: "description", content: "{meta_description}" }
        document::Meta { name: "author", content: "{SITE.author}" }
        document::Meta { name: "robots", content: "index, follow" }
        
        // Canonical URL (P13-A3)
        document::Link { rel: "canonical", href: "{canonical_url}" }
        
        // Open Graph Tags (P13-B)
        document::Meta { property: "og:title", content: "{full_title}" }
        document::Meta { property: "og:description", content: "{meta_description}" }
        document::Meta { property: "og:image", content: "{og_image}" }
        document::Meta { property: "og:url", content: "{canonical_url}" }
        document::Meta { property: "og:type", content: "{og_type}" }
        document::Meta { property: "og:site_name", content: "{SITE.name}" }
        document::Meta { property: "og:locale", content: "en_US" }
        
        // Twitter Cards (P13-C)
        document::Meta { name: "twitter:card", content: "summary_large_image" }
        document::Meta { name: "twitter:title", content: "{full_title}" }
        document::Meta { name: "twitter:description", content: "{meta_description}" }
        document::Meta { name: "twitter:image", content: "{og_image}" }
        document::Meta { name: "twitter:creator", content: "@{SITE.twitter}" }
        
        // Theme Color
        document::Meta { name: "theme-color", content: "#00FFFF" }
        
        // Additional SEO
        document::Meta { name: "generator", content: "Dioxus + Rust" }
    }
}

/// Structured Data / JSON-LD for Person (P13-D)
#[component]
pub fn PersonSchema() -> Element {
    let schema = format!(r#"{{
        "@context": "https://schema.org",
        "@type": "Person",
        "name": "{}",
        "url": "{}",
        "email": "{}",
        "sameAs": [
            "https://github.com/{}",
            "https://linkedin.com/in/{}",
            "https://twitter.com/{}"
        ],
        "jobTitle": "Full-Stack Developer",
        "knowsAbout": ["Rust", "Web Development", "Dioxus", "WebAssembly"]
    }}"#, SITE.author, SITE.base_url, SITE.email, SITE.github, SITE.linkedin, SITE.twitter);
    
    rsx! {
        script {
            r#type: "application/ld+json",
            dangerous_inner_html: "{schema}"
        }
    }
}

/// Breadcrumb Schema
#[component]
pub fn BreadcrumbSchema(items: Vec<(String, String)>) -> Element {
    let items_json: Vec<String> = items.iter().enumerate().map(|(i, (name, url))| {
        format!(r#"{{
            "@type": "ListItem",
            "position": {},
            "name": "{}",
            "item": "{}"
        }}"#, i + 1, name, url)
    }).collect();
    
    let schema = format!(r#"{{
        "@context": "https://schema.org",
        "@type": "BreadcrumbList",
        "itemListElement": [{}]
    }}"#, items_json.join(","));
    
    rsx! {
        script {
            r#type: "application/ld+json",
            dangerous_inner_html: "{schema}"
        }
    }
}
