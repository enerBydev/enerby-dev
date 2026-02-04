# enerby.dev 🚀

Personal portfolio and blog built with **Dioxus** and **100% Rust**.

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![WebAssembly](https://img.shields.io/badge/WebAssembly-654FF0?style=for-the-badge&logo=webassembly&logoColor=white)
![Dioxus](https://img.shields.io/badge/Dioxus-0.7-blue?style=for-the-badge)

## 🎯 Overview

A modern, cyberpunk-themed portfolio website showcasing my work, skills, and blog posts. Built entirely in Rust using the Dioxus framework, compiled to WebAssembly for blazing-fast performance.

### Features

- ⚡ **100% Rust** - No JavaScript required
- 🦀 **Dioxus Framework** - React-like DX with Rust performance
- 🌐 **WebAssembly** - Near-native performance in the browser
- 🎨 **Cyberpunk Theme** - Neon colors, glassmorphism, animations
- 📱 **Responsive** - Mobile-first design
- 🔍 **SEO Optimized** - Meta tags, Open Graph, Twitter Cards
- 📝 **Blog System** - Markdown-based posts
- 📂 **Project Showcase** - Filter by status and technologies

## 🛠️ Tech Stack

| Category | Technology |
|----------|------------|
| Language | Rust 2024 Edition |
| Framework | Dioxus 0.7 |
| Styling | Tailwind CSS |
| Build | Cargo + dx CLI |
| Deployment | Cloudflare Pages |

## 🚀 Getting Started

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Dioxus CLI
cargo install dioxus-cli
```

### Development

```bash
# Clone the repository
git clone https://github.com/enerBydev/enerby.dev.git
cd enerby.dev

# Start development server
dx serve --port 8080

# Open http://localhost:8080
```

### Production Build

```bash
# Build for production with SSG
dx build --release

# Or use the build script
./scripts/build.sh
```

## 📁 Project Structure

```
enerby.dev/
├── src/
│   ├── main.rs           # Entry point
│   ├── routes.rs         # Router configuration
│   ├── config.rs         # Site configuration
│   ├── components/       # UI Components
│   │   ├── atoms.rs      # Button, Badge, etc.
│   │   ├── molecules.rs  # Card, SectionTitle, etc.
│   │   ├── hero.rs       # Hero section
│   │   ├── about.rs      # About section
│   │   ├── skills.rs     # Skills section
│   │   ├── projects.rs   # Projects section
│   │   ├── blog.rs       # Blog section
│   │   └── contact.rs    # Contact form
│   ├── layout/           # Layout components
│   └── pages/            # Page components
├── assets/
│   └── main.css          # Tailwind CSS
├── Cargo.toml            # Rust dependencies
├── Dioxus.toml           # Dioxus configuration
└── scripts/
    └── build.sh          # Production build script
```

## 🎨 Design System

The site uses a custom cyberpunk design system with:

- **Primary Color**: Cyan (`#00FFFF`)
- **Secondary**: Purple (`#9D00FF`)
- **Background**: Dark (`#0A0A0F`)
- **Effects**: Glassmorphism, neon glow, glitch animations

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 📧 Contact

- **Website**: [enerby.dev](https://enerby.dev)
- **GitHub**: [@enerBydev](https://github.com/enerBydev)
- **Email**: rjmemdoza.s@gmail.com

---

Built with ❤️ and 🦀 Rust
