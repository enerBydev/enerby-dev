# enerby.dev - Architecture

## Overview

This portfolio is built with **Dioxus 0.7**, a Rust framework for building web applications using WebAssembly. The architecture follows a component-based pattern inspired by React.

## Tech Stack

| Layer | Technology |
|-------|------------|
| Language | Rust 2024 Edition |
| Framework | Dioxus 0.7 |
| Styling | Tailwind CSS |
| Deployment | Cloudflare Pages |

## Directory Structure

```
enerby.dev/
├── src/
│   ├── main.rs           # Entry point, App component
│   ├── routes.rs         # Router configuration
│   ├── config.rs         # Site metadata & configuration
│   ├── data.rs           # Static data (placeholder)
│   ├── utils.rs          # Helper functions
│   ├── theme/            # Design tokens & theming
│   ├── components/       # UI Components
│   │   ├── atoms.rs      # Button, Badge, Icon
│   │   ├── molecules.rs  # Card, SectionTitle
│   │   ├── hero.rs       # Hero section
│   │   ├── about.rs      # About section
│   │   ├── skills.rs     # Skills grid
│   │   ├── projects.rs   # Project showcase
│   │   ├── blog.rs       # Blog posts
│   │   ├── contact.rs    # Contact form
│   │   ├── animations.rs # Animation utilities
│   │   └── seo.rs        # SEO components
│   ├── layouts/          # Layout wrappers
│   │   ├── root.rs       # Root layout
│   │   ├── header.rs     # Navigation
│   │   └── footer.rs     # Footer
│   ├── pages/            # Route pages
│   │   ├── home.rs       # Landing page
│   │   ├── about.rs      # About page
│   │   ├── projects.rs   # Projects page
│   │   ├── blog.rs       # Blog page
│   │   └── contact.rs    # Contact page
│   └── tests/            # Unit tests
├── assets/
│   └── main.css          # Tailwind CSS
├── Cargo.toml            # Rust dependencies
├── Dioxus.toml           # Dioxus configuration
└── .github/workflows/    # CI/CD
```

## Component Hierarchy

```
App
└── Router
    └── RootLayout
        ├── Header (Navigation)
        ├── Page Content
        │   ├── HomePage
        │   │   ├── SeoHead
        │   │   ├── HeroSection
        │   │   ├── AboutSection
        │   │   ├── SkillsSection
        │   │   ├── ProjectsSection
        │   │   ├── BlogSection
        │   │   └── ContactSection
        │   ├── AboutPage
        │   ├── ProjectsPage
        │   ├── BlogPage
        │   └── ContactPage
        └── Footer
```

## Data Flow

1. **Static Data**: Projects, blog posts, and skills are defined in component files
2. **Props**: Data flows down through component props
3. **Signals**: Local state managed with `use_signal()` hooks
4. **No Server State**: Pure client-side rendering

## Styling System

- **Colors**: Cyberpunk palette (cyan, purple, dark backgrounds)
- **Effects**: Glassmorphism, neon glow, glitch animations
- **Responsive**: Mobile-first with Tailwind breakpoints

## Build Process

```bash
# Development
dx serve --port 8080

# Production
dx build --release
```

## Testing

```bash
# Run all tests
cargo test

# Run with verbose output
cargo test -- --nocapture
```
