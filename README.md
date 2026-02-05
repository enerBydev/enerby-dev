# enerby.dev 🚀

> **A "Living" Portfolio built with Rust, Dioxus, and WebAssembly.**

![Rust](https://img.shields.io/badge/Rust-2024-000000?style=for-the-badge&logo=rust&logoColor=white)
![WebAssembly](https://img.shields.io/badge/WebAssembly-Strict-654FF0?style=for-the-badge&logo=webassembly&logoColor=white)
![Dioxus](https://img.shields.io/badge/Dioxus-0.7-blue?style=for-the-badge)
![Status](https://img.shields.io/badge/Status-Production_Ready-success?style=for-the-badge)

## 🎯 Overview

**enerby.dev** is not just a static site; it's a strongly-typed, compiled application running in your browser. It leverages the power of **WebAssembly** to deliver a native-like experience with zero JavaScript bloat.

The site features a **Cyberpunk/Neon aesthetic** with glassmorphism, organic glitch effects, and "living" elements that react to data.

### 🌟 Key Features

#### 🧠 **GitHub Intelligence (Static-First)**

Unlike typical static sites, this portfolio understands its own codebase:

- **Auto-Enrichment**: Project cards automatically pull descriptions, topics, and "Live Demo" links from GitHub repository metadata.
- **Dynamic Stats**: A unified tracking system (`github_stats.rs`) calculates total Lines of Code (LOC) across all maintained projects (currently **35k+ lines**).
- **Architecture**: Implements a **"Static-First" strategy** to bypass WASM `SystemTime` limitations and CORS issues, embedding API data at build time for instant, panic-free loading.

#### 📝 **Advanced Blog System**

- **Markdown Engine**: Custom implementation using `pulldown-cmark` and `gray_matter`.
- **Prose Styling**: Typography optimized for readability with syntax highlighting.
- **Metadata**: Automatic read-time calculation and tag management.

#### 🎨 **Reactive UI/UX**

- **Living Counters**: Numbers tick up dynamically (e.g., LOC stats).
- **Glitch Effects**: Subtle, non-intrusive CSS animations that trigger organically on hover.
- **Navigation**: Client-side routing with Dioxus Router for instant page transitions.

---

## 🛠️ Technical Architecture

This project solves unique challenges of running Rust in the browser:

### The "Static-First" Pattern

Running `std::time::SystemTime` or `reqwest` inside `wasm32-unknown-unknown` often leads to panics or huge binary sizes. We solved this by creating a data abstraction layer:

1. **Source of Truth**: `src/utils/github_api.rs` acts as an in-memory database.
2. **Safety**: The `github_cache` module implements a platform-agnostic TTL system that degrades gracefully in WASM.
3. **Result**: 100% reliability with zero runtime API failures.

### Project Structure

```
enerby.dev/
├── src/
│   ├── components/       # Atomic Design Components
│   │   ├── atoms/        # Badges, Buttons (Pure)
│   │   ├── molecules/    # Cards, SectionTitles
│   │   └── ...           # Organisms (Hero, ProjectsSection)
│   ├── utils/            # Core Logic Modules
│   │   ├── github_api.rs # GitHub Data & Mocking
│   │   ├── github_stats.rs # LOC & Repo Analytics
│   │   └── markdown_loader.rs # Blog Engine
│   ├── routes.rs         # Type-safe Router
│   └── main.rs           # WASM Entry Point
├── content/              # Blog Posts (Markdown)
└── input.css             # Tailwind v4 Configuration
```

---

## 🚀 Getting Started

### Prerequisites

- **Rust** (Latest Stable)
- **Dioxus CLI**: `cargo install dioxus-cli`

### Development

```bash
# Clone the repository
git clone https://github.com/enerBydev/enerby.dev.git
cd enerby.dev

# Run dev server (Hot Reload enabled)
dx serve
```

### Production Build

```bash
# Compile to optimized WASM
dx build --release
```

---

## 📧 Contact

- **Author**: EnerByDev
- **GitHub**: [@enerBydev](https://github.com/enerBydev)
- **Email**: <rjmemdoza.s@gmail.com>

---

_Built with ❤️, 🦀 Rust, and a lot of caffeine._
