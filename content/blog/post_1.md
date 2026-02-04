---
slug: building-portfolio-rust-dioxus
title: Construyendo un Portfolio con Rust y Dioxus
date: 2026-02-04
excerpt: Deep dive en cómo construí este portfolio usando Dioxus, el framework React-like de Rust. Arquitectura, componentes y lecciones aprendidas.
tags:
  - Rust
  - Dioxus
  - WebAssembly
  - Portfolio
featured: true
read_time: 12
---

# Construyendo enerby.dev

Este portfolio no es solo una muestra de mi trabajo, es una declaración de intenciones: **Rust está listo para el Frontend**.

## ¿Por qué Dioxus?

Dioxus es un framework portable para construir interfaces de usuario cross-platform. Se siente muy similar a React, pero con el poder de Rust. 

```rust
fn app() -> Element {
    rsx! {
        div { "Hello World" }
    }
}
```

Aunque en mi opinion personal es mas parecido a NuxtJS que a React por muchas razones y por eso se me es facil , razonar comprender y encontrar coherencia y logica
### Arquitectura

El proyecto sigue una estructura limpia:
- **components/**: UI reutilizable
- **pages/**: Vistas principales
- **layouts/**: Estructuras comunes (Header, Footer)

## El Efecto Glitch

Uno de los desafíos fue implementar el efecto glitch sin sacrificar accesibilidad o rendimiento. Usé CSS puro con keyframes personalizados.

## Conclusión

Construir con Dioxus ha sido una experiencia increíble. La seguridad de tipos de Rust en el frontend elimina una clase entera de bugs.
