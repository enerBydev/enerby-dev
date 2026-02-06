---
slug: hexagonal-architecture-rust
title: "Arquitectura Hexagonal en Rust: Guía Práctica"
date: 2026-02-04
excerpt: Cómo implementar Clean Architecture con Domain-Driven Design en proyectos Rust. Ports, Adapters y Repository Pattern explicados.
tags:
  - Rust
  - Architecture
  - DDD
  - Clean Code
featured: true
---

# Arquitectura Hexagonal en Rust

La arquitectura hexagonal, o puertos y adaptadores, permite desacoplar tu lógica de negocio de los detalles técnicos.

## El Dominio

En el centro de todo está tu dominio. En Rust, esto se modela típicamente con structs y enums puros.

```rust
pub struct User {
    pub id: UserId,
    pub email: String,
}
```

## Puertos (Traits)

Los puertos definen cómo el mundo exterior interactúa con la aplicación.

## Adaptadores

Los adaptadores implementan los puertos.

> "La dependencia debe apuntar hacia adentro, hacia las reglas de alto nivel."

