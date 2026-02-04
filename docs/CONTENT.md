# enerby.dev - Content Guide

## Adding a New Project

Edit `src/components/projects.rs` and add to the `get_projects()` function:

```rust
Project {
    id: "my-project",
    title: "My New Project",
    description: "A brief description of the project.",
    image: Some("/assets/projects/my-project.png"),
    technologies: vec!["Rust", "WASM", "Dioxus"],
    github_url: Some("https://github.com/user/project"),
    live_url: Some("https://myproject.com"),
    status: ProjectStatus::Active,  // Featured, Active, or Archived
}
```

## Adding a Blog Post

Edit `src/components/blog.rs` and add to the `get_blog_posts()` function:

```rust
BlogPost {
    slug: "my-new-post",
    title: "My New Blog Post",
    excerpt: "A brief summary of the post content...",
    content: "Full markdown content goes here...",
    date: "2024-02-01",
    read_time: 5,
    tags: vec!["Rust", "Tutorial"],
    status: PostStatus::Published,  // Published or Draft
}
```

## Modifying Site Configuration

Edit `src/config.rs`:

```rust
pub static SITE: SiteConfig = SiteConfig {
    name: "enerby.dev",
    title: "enerby.dev | Full-Stack Developer",
    description: "Personal portfolio...",
    author: "enerBydev",
    email: "your@email.com",
    base_url: "https://enerby.dev",
};
```

## Adding Navigation Links

Edit `src/config.rs`:

```rust
pub static NAV_LINKS: &[NavLink] = &[
    NavLink { label: "Home", href: "/", is_external: false },
    NavLink { label: "About", href: "/about", is_external: false },
    // Add more...
];
```

## Adding Social Links

```rust
pub static SOCIAL_LINKS: &[SocialLink] = &[
    SocialLink { name: "GitHub", url: "https://github.com/user", icon: "github" },
    SocialLink { name: "Twitter", url: "https://twitter.com/user", icon: "twitter" },
];
```

## Deployment

### Automatic (GitHub Actions)
Push to `main` branch triggers automatic deployment to Cloudflare Pages.

### Manual
```bash
# Build
dx build --release

# Output in dist/ folder
# Upload to Cloudflare Pages dashboard
```

## Environment Setup

### Prerequisites
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Dioxus CLI
cargo install dioxus-cli
```

### Development
```bash
# Start dev server
dx serve --port 8080

# Run tests
cargo test

# Check code
cargo clippy
```
