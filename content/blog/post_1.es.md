---
slug: building-portfolio-rust-dioxus
title: "El Futuro del Desarrollo Web: Rust & WASM"
date: 2023-10-27
excerpt: Por qué WebAssembly y Rust están cambiando el panorama del frontend para siempre.
tags:
  - Rust
  - WASM
  - WebDev
featured: true
read_time: 5
---

# Construyendo enerby.dev

Este portfolio no es solo una muestra de mi trabajo, es una **declaración de intenciones**: Rust está listo para el Frontend.

En este post, compartiré las decisiones técnicas, los desafíos superados y las lecciones aprendidas mientras construía este sitio con tecnologías modernas.

## ¿Por qué Dioxus?

Dioxus es un framework portable para construir interfaces de usuario cross-platform. Se siente muy similar a React, pero aprovecha todo el poder del sistema de tipos de Rust.

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

### Mi Perspectiva: Más Nuxt que React

Aunque Dioxus se promociona como "React for Rust", en mi experiencia personal encuentro más similitudes con el ecosistema Vue/Nuxt:

- **Routing declarativo**: Similar a `pages/` de Nuxt
- **Componentes reactivos**: El modelo mental es muy parecido
- **Integración fullstack**: Con Dioxus Fullstack, recuerda a Nitro
- **Hidratación SSR**: Funciona de manera análoga

Esta familiaridad con Nuxt me permitió adaptar rápidamente mis conocimientos al nuevo paradigma.

## Arquitectura del Proyecto

El proyecto sigue una estructura limpia y modular:

```
src/
├── components/     # UI reutilizable (atoms, molecules)
├── pages/          # Vistas principales (Home, Blog, Projects)
├── layouts/        # Estructuras comunes (Header, Footer)
├── routes/         # Definición de rutas
└── utils/          # Helpers y utilidades
```

### Sistema de Componentes

Implementé un sistema de design tokens usando CSS variables:

```css
:root {
  --primary: #00ffff;
  --secondary-purple: #9333ea;
  --bg-main: #030712;
}
```

Esto permite consistencia visual y facilita el theming.

## El Efecto Glitch

Uno de los mayores desafíos fue implementar el efecto glitch del header sin sacrificar accesibilidad ni rendimiento.

### Solución: CSS Puro

Opté por keyframes CSS personalizados en lugar de JavaScript:

```css
@keyframes glitch {
  0%, 100% { clip-path: inset(0 0 0 0); }
  10% { clip-path: inset(10% 0 85% 0); }
  20% { clip-path: inset(80% 0 5% 0); }
  /* ... más frames */
}
```

### Beneficios

1. **Cero JavaScript**: No bloquea el main thread
2. **GPU Accelerated**: Animaciones fluidas a 60fps
3. **Respeta `prefers-reduced-motion`**: Accesible por defecto

## WebAssembly en Producción

Dioxus compila a WASM, lo que significa:

- **Bundle pequeño**: ~300KB gzipped
- **Rendimiento nativo**: Rust optimizado
- **Seguridad de memoria**: Sin memory leaks

El resultado es una SPA ultra-rápida con tiempos de carga mínimos.

## Lecciones Aprendidas

### Lo Bueno
- **Type Safety total**: Los errores se capturan en compile time
- **Hot Reload**: El DX es excelente
- **Comunidad activa**: Discord muy responsive

### Los Retos
- **Ecosistema joven**: Menos componentes listos para usar
- **Debugging WASM**: Requiere herramientas específicas
- **Curva de aprendizaje**: Rust + frontend es intenso

## Conclusión

Construir con Dioxus ha sido una experiencia increíble. La seguridad de tipos de Rust en el frontend elimina una clase entera de bugs que plagan las aplicaciones JavaScript.

**¿Recomiendo Dioxus?** Absolutamente, especialmente si:
- Ya conoces Rust
- Valoras la seguridad de tipos
- Quieres explorar WASM en producción

El futuro del frontend puede tener más Rust de lo que pensamos. 🦀

