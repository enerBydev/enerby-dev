---
slug: building-portfolio-rust-dioxus
title: Building a Portfolio with Rust and Dioxus
date: 2026-02-04
excerpt: Deep dive into how I built this portfolio using Dioxus, the React-like framework for Rust. Architecture, components, and lessons learned.
tags:
  - Rust
  - Dioxus
  - WebAssembly
  - Portfolio
featured: true
---

# Building enerby.dev

This portfolio isn't just a showcase of my work—it's a **statement of intent**: Rust is ready for the Frontend.

In this post, I'll share the technical decisions, challenges overcome, and lessons learned while building this site with modern technologies.

## Why Dioxus?

Dioxus is a portable framework for building cross-platform user interfaces. It feels very similar to React, but leverages the full power of Rust's type system.

```rust
fn app() -> Element {
    rsx! {
        div { class: "container",
            h1 { "Hello from Dioxus" }
            p { "Building UIs with Rust" }
        }
    }
}
```

### My Perspective: More Nuxt than React

Although Dioxus is marketed as "React for Rust", in my personal experience I find more similarities with the Vue/Nuxt ecosystem:

- **Declarative routing**: Similar to Nuxt's `pages/`
- **Reactive components**: The mental model is very similar
- **Fullstack integration**: With Dioxus Fullstack, it reminds me of Nitro
- **SSR hydration**: Works analogously

This familiarity with Nuxt allowed me to quickly adapt my knowledge to the new paradigm.

## Project Architecture

The project follows a clean and modular structure:

```
src/
├── components/     # Reusable UI (atoms, molecules)
├── pages/          # Main views (Home, Blog, Projects)
├── layouts/        # Common structures (Header, Footer)
├── routes/         # Route definitions
└── utils/          # Helpers and utilities
```

### Component System

I implemented a design tokens system using CSS variables:

```css
:root {
  --primary: #00ffff;
  --secondary-purple: #9333ea;
  --bg-main: #030712;
}
```

This allows visual consistency and facilitates theming.

## The Glitch Effect

One of the biggest challenges was implementing the header's glitch effect without sacrificing accessibility or performance.

### Solution: Pure CSS

I opted for custom CSS keyframes instead of JavaScript:

```css
@keyframes glitch {
  0%, 100% { clip-path: inset(0 0 0 0); }
  10% { clip-path: inset(10% 0 85% 0); }
  20% { clip-path: inset(80% 0 5% 0); }
  /* ... more frames */
}
```

### Benefits

1. **Zero JavaScript**: Doesn't block the main thread
2. **GPU Accelerated**: Smooth animations at 60fps
3. **Respects `prefers-reduced-motion`**: Accessible by default

## WebAssembly in Production

Dioxus compiles to WASM, which means:

- **Small bundle**: ~300KB gzipped
- **Native performance**: Optimized Rust
- **Memory safety**: No memory leaks

The result is an ultra-fast SPA with minimal load times.

## Lessons Learned

### The Good
- **Complete Type Safety**: Errors are caught at compile time
- **Hot Reload**: The DX is excellent
- **Active community**: Very responsive Discord

### The Challenges
- **Young ecosystem**: Fewer ready-to-use components
- **WASM debugging**: Requires specific tools
- **Learning curve**: Rust + frontend is intense

## Conclusion

Building with Dioxus has been an incredible experience. Rust's type safety in the frontend eliminates an entire class of bugs that plague JavaScript applications.

**Do I recommend Dioxus?** Absolutely, especially if:
- You already know Rust
- You value type safety
- You want to explore WASM in production

The future of frontend might have more Rust than we think. 🦀
