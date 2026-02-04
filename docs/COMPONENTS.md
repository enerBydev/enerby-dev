# enerby.dev - Component Guide

## Atoms (Basic Building Blocks)

### Button
```rust
Button {
    text: "Click me",
    variant: ButtonVariant::Primary,  // Primary, Secondary, Ghost, Neon
    size: "md",                       // sm, md, lg
    href: Some("/about".to_string()), // Optional link
    onclick: |_| {}                   // Optional handler
}
```

### Badge
```rust
Badge {
    text: "Rust",
    variant: BadgeVariant::Default
}
```

### Icon
```rust
Icon { name: "github", size: 24 }
```

## Molecules (Composite Components)

### Card
```rust
Card {
    title: Some("Project Title"),
    hoverable: true,
    children: rsx! { ... }
}
```

### SectionTitle
```rust
SectionTitle {
    title: "About Me",
    subtitle: Some("Learn more about my journey"),
    centered: true
}
```

## Sections

### HeroSection
Main landing section with name, title, and CTAs.

### AboutSection
Biography with timeline and interests.

### SkillsSection
Skills organized by category with progress bars.
- Categories: Languages, Frameworks, Tools, Concepts

### ProjectsSection
Project cards with filtering by status.

### BlogSection
Blog post previews with tags and read time.

### ContactSection
Contact form with validation + alternative contact methods.

## Layouts

### RootLayout
Wraps all pages with Header and Footer.

### Container
Centers content with max-width.

### Section
Adds padding and gap for sections.

### Grid
Responsive grid layout.

## Animation Components

### AnimateOnScroll
Reveals content when scrolled into view.
```rust
AnimateOnScroll {
    animation: "fade-up",
    children: rsx! { ... }
}
```

### GlitchEffect
Cyberpunk glitch text effect (for headings).

### NeonFlicker
Neon light flicker effect.

## SEO Components

### SeoHead
Adds meta tags, Open Graph, and Twitter Cards.
```rust
SeoHead {
    title: "Page Title",
    description: "Page description",
    keywords: Some("rust, portfolio"),
    og_image: Some("/images/og.png"),
    canonical: Some("https://enerby.dev/about")
}
```
