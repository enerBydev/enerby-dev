//! Configuration Module - Global app settings
//! Contains site metadata and global constants

/// Site metadata and configuration
pub struct SiteConfig {
    pub name: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub author: &'static str,
    pub email: &'static str,
    pub github: &'static str,
    pub linkedin: &'static str,
    pub twitter: &'static str,
    pub base_url: &'static str,
}

/// Global site configuration
pub const SITE: SiteConfig = SiteConfig {
    name: "enerby.dev",
    title: "enerby.dev | Software Developer Portfolio",
    description: "Personal portfolio showcasing web development, Rust, and full-stack projects. Built with Dioxus and 100% Rust.",
    author: "enerBydev",
    email: "rjmemdoza.s@gmail.com",
    github: "https://github.com/enerBydev",
    linkedin: "https://linkedin.com/in/enerbydev",
    twitter: "https://twitter.com/enerbydev",
    base_url: "https://enerby.dev",
};

/// Navigation links
pub struct NavLink {
    pub label: &'static str,
    pub href: &'static str,
    pub is_external: bool,
}

/// Main navigation items
pub const NAV_LINKS: &[NavLink] = &[
    NavLink { label: "Home", href: "/", is_external: false },
    NavLink { label: "About", href: "/about", is_external: false },
    NavLink { label: "Projects", href: "/projects", is_external: false },
    NavLink { label: "Blog", href: "/blog", is_external: false },
    NavLink { label: "Contact", href: "/contact", is_external: false },
];

/// Social links for footer/hero
pub struct SocialLink {
    pub name: &'static str,
    pub url: &'static str,
    pub icon: &'static str,
}

pub const SOCIAL_LINKS: &[SocialLink] = &[
    SocialLink { name: "GitHub", url: "https://github.com/enerBydev", icon: "github" },
    SocialLink { name: "LinkedIn", url: "https://linkedin.com/in/enerbydev", icon: "linkedin" },
    SocialLink { name: "Twitter", url: "https://twitter.com/enerbydev", icon: "twitter" },
];
