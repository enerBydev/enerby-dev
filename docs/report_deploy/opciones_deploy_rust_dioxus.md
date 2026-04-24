# Reporte: Opciones de Deploy para Rust + Dioxus

> **Fecha**: 2026-04-23
> **Contexto**: Análisis de plataformas cloud gratuitas (sin tarjeta de crédito) para proyectos Rust + Dioxus — CSR, fullstack, SSR, desktop y mobile.
> **Proyecto**: enerby.dev y ecosistema de proyectos en `Rust_Proyects/`

---

## 1. Contexto y Motivación

La estrategia tecnológica se basa en **1 codebase, múltiples targets** usando Dioxus + Rust:

| Target | Cómo |
|---|---|
| **Web (CSR)** | `dx build --features web` → WASM estático |
| **Web (Fullstack/SSR)** | `dx build --features fullstack` → Axum + Tokio nativo |
| **Desktop** | `dx build --features desktop` → Tauri-like |
| **Mobile** | `dx build --features mobile` → iOS/Android |

Cloudflare Pages es excelente para estáticos pero **no soporta Rust nativo en servidor**. Este reporte documenta las alternativas.

---

## 2. Cloudflare Pages — Limitaciones con Rust + Dioxus

### Lo que SÍ funciona

| Feature | Estado | Detalle |
|---|---|---|
| CSR/WASM estático | ✅ Completo | `dx build --release --features web` → HTML + JS + WASM |
| SPA routing | ✅ | Via `_redirects` |
| CDN global | ✅ | Ilimitado, gratis |
| Build command custom | ✅ | Instala Rust + dx en CI (9 min build) |
| Custom domains + SSL | ✅ | Automático y gratis |

### Lo que NO funciona

| Limitante | Tipo | Detalle |
|---|---|---|
| SSR / Hydration | ❌ Sin solución nativa | `dioxus-server` depende de Tokio completo + hyper — no compila a WASM |
| Server Functions | ⚠️ Con workaround | Requiere fork parcheado `dioxus-server-cf` + `unsafe { __wasm_call_ctors() }` |
| Threading | ❌ Imposible | Workers (V8 isolates) no soporta threads. Tokio runtime = imposible |
| Bundle size | ⚠️ Riesgo | 3 MB (free) / 10 MB (paid) — Dioxus server+WASM puede excederlo |
| CPU time | ⚠️ Riesgo | 10ms/request en free plan — SSR puede no alcanzar |
| Build time | ⚠️ Costo | 9+ minutos compilando Rust desde cero en cada deploy |

### Soluciones experimentales (no recomendadas para producción)

| Solución | Estado | Riesgo |
|---|---|---|
| `dioxus-cloudflare` crate (v0.7.3) | Publicada en crates.io | Depende de fork `dioxus-server-cf` que hay que rebasar con cada release de Dioxus |
| `cf-dioxus` ejemplo (jongiddy) | Funciona para server functions | Hydration explícitamente **NO soportada** |
| PR #3840 (axum_core wasm feature) | En revisión upstream | Cuando se mergee, el fork parcheado deja de ser necesario |

---

## 3. Plataformas que Corren Rust Nativo — GRATIS, SIN Tarjeta de Crédito

### 3.1 Shuttle.rs

| Atributo | Detalle |
|---|---|
| **URL** | https://www.shuttle.rs |
| **Free tier** | Community ($0/mes, siempre) |
| **Tarjeta de crédito** | No requerida |
| **RAM** | 0.5 GB |
| **vCPU** | 0.25 |
| **Proyectos** | 1 |
| **Build minutes** | 100/mes |
| **Database** | 0.5 GB PostgreSQL compartida |
| **Egress** | 1 GB/mes |
| **Custom domain** | 1 incluido |
| **SSL** | Automático |
| **Rust nativo** | ✅ Primera clase — `shuttle deploy` compila y despliega |
| **Instancia** | Spot (puede reiniciarse) |
| **Región** | Solo London (eu-west-2) |
| **Dioxus fullstack** | Con workarounds — Shuttle soporta Axum nativo |

**Limitaciones clave**:
- Solo 1 proyecto en free tier
- Spot instance: reinicios ocasionales por mantenimiento
- 1 GB egress/mes — insuficiente para tráfico alto
- 100 build min/mes — ~10 deploys/mes (Rust compila 10+ min)
- 0.5 GB RAM — puede ser justo para Dioxus fullstack con SSR

---

### 3.2 Render

| Atributo | Detalle |
|---|---|
| **URL** | https://render.com |
| **Free tier** | Sí ($0/mes) |
| **Tarjeta de crédito** | No requerida (signup con GitHub) |
| **RAM** | 512 MB |
| **vCPU** | 0.1 |
| **Instance hours** | 750/mes (~1 servicio siempre encendido) |
| **Database** | PostgreSQL free (256 MB, **caduca a los 30 días**) |
| **Egress** | 100 GB/mes |
| **Custom domain** | Incluido |
| **SSL** | Automático (managed TLS) |
| **Rust nativo** | ✅ Primera clase — template Rocket, soporte `RUSTUP_TOOLCHAIN` |
| **Cold start** | Se apaga tras 15 min sin tráfico (~1 min en reactivar) |
| **Filesystem** | Efímero (datos se pierden en restart) |

**Limitaciones clave**:
- 0.1 vCPU — muy lento para SSR rendering
- Cold start de ~1 minuto — visitantes ven loading en primer request
- DB caduca cada 30 días — hay que recrear
- Filesystem efímero — no sirve para uploads o SQLite local

---

### 3.3 Gigalixir

| Atributo | Detalle |
|---|---|
| **URL** | https://gigalixir.com |
| **Free tier** | Sí ("free forever, no credit card required") |
| **Tarjeta de crédito** | No requerida (explícito en su docs) |
| **RAM** | 0.5 GB |
| **vCPU** | ~0.25 |
| **Database** | PostgreSQL free (10,000 filas, 2 conexiones concurrentes) |
| **Custom domain** | Incluido |
| **SSL** | Automático |
| **Rust nativo** | ✅ Build pipeline nativo — compila tu Rust sin Dockerfile |
| **Dioxus fullstack** | Probablemente sí — corre binarios Rust nativos |
| **Inactividad** | Duerme tras 30 días sin deploy |

**Limitaciones clave**:
- 10,000 filas en DB — muy poco para producción
- Solo 2 conexiones DB concurrentes
- Duerme tras 30 días sin deploy (solucionable con cron ping)
- Sin SSH/console en free tier

---

### 3.4 Leapcell

| Atributo | Detalle |
|---|---|
| **URL** | https://leapcell.io |
| **Free tier** | Hobby ($0/mes) |
| **Tarjeta de crédito** | No requerida |
| **Proyectos** | 20 |
| **Runtime** | 3 GB-horas/mes serverless |
| **Build minutes** | 60/mes |
| **Database** | PostgreSQL multi-tenant + Redis |
| **Egress** | 2 GB/mes |
| **Custom domain** | Incluido con TLS |
| **Rust nativo** | ✅ Primera clase — `cargo build --release` nativo |
| **Invocation timeout** | 900 segundos |

**Limitaciones clave**:
- 3 GB-horas/mes de runtime — ~1.8 horas continuas
- 60 build min/mes — solo 4-6 deploys de Rust
- 2 GB egress/mes — muy limitado
- Serverless, no servidor persistente

---

### 3.5 Railway

| Atributo | Detalle |
|---|---|
| **URL** | https://railway.com |
| **Free tier** | Sí ($1/mes en créditos, no acumulables) |
| **Tarjeta de crédito** | No requerida para Free plan |
| **RAM** | 0.5 GB |
| **vCPU** | 1 |
| **Egress** | 10 GB/mes |
| **Build** | Docker o buildpack nativo (Rust soportado) |
| **Custom domain** | Incluido |
| **SSL** | Automático |

**Limitaciones clave**:
- $1/mes en créditos — un servicio Rust consume ~$0.50-1.00/mes, borderline
- Servicios duermen con inactividad en free tier
- Hobby plan ($5/mes) requiere tarjeta de crédito

---

### 3.6 Koyeb

| Atributo | Detalle |
|---|---|
| **URL** | https://koyeb.com |
| **Free tier** | 1 instancia free |
| **Tarjeta de crédito** | No requerida |
| **RAM** | 512 MB |
| **vCPU** | 0.1 |
| **Storage** | 2 GB SSD |
| **Custom domain** | 10 incluidos |
| **SSL** | Automático |
| **Rust** | Via Docker o Git build |

**Limitaciones clave**:
- 0.1 vCPU — muy limitado
- Sin storage persistente en free tier
- Scale-to-zero = cold starts

---

## 4. Plataformas para Estáticos/WASM (Frontend CSR) — GRATIS

| Plataforma | Bandwidth | Builds/mes | Dominio custom | SSL | Rust nativo | Nota |
|---|---|---|---|---|---|---|
| **Cloudflare Pages** | Ilimitado | 500 | ✅ | ✅ | WASM only | Mejor opción — CDN global |
| **GitHub Pages** | 100 GB/mes | 10/hora | ✅ | ✅ | No | Solo repos públicos (free tier) |
| **Netlify** | Variable (créditos) | 300 créditos | ✅ | ✅ | No | Functions solo JS/Go |
| **Vercel** | 100 GB/mes | 6000 min | ✅ | ✅ | WASM en Edge | Hobby = no comercial por ToS |
| **GitLab Pages** | 5 GB storage | 400 CI min | ✅ | ✅ | No | CI más flexible que GitHub |

---

## 5. Bases de Datos — GRATIS, SIN Tarjeta de Crédito

| Plataforma | Storage | Proyectos | Tipo | Scale-to-zero | Auth incluido | Nota |
|---|---|---|---|---|---|---|
| **Neon** | 0.5 GB/proyecto | 100 | PostgreSQL | ✅ (350ms cold start) | No | Mejor opción free — branching, serverless |
| **Supabase** | 500 MB | 2 | PostgreSQL | ✅ | ✅ (50K MAU) | Auth + storage + realtime incluido |
| **Turso** | 5 GB total | 100 DBs | SQLite | ✅ | No | Si prefieres SQLite |
| **CockroachDB** | 10 GB | 1 | PostgreSQL | ✅ | No | Compatible con Postgres |

---

## 6. Plataformas Descartadas

| Plataforma | Razón de descarte |
|---|---|
| **Fly.io** | Requiere tarjeta de crédito. Free trial solo 2 horas/7 días |
| **Oracle Cloud** | Requiere tarjeta de crédito (aunque tiene free tier robusto) |
| **PlanetScale** | Eliminó free tier en abril 2024. Mínimo $5/mes |
| **Northflank** | No tiene free tier permanente. Solo trial 7 días |
| **Deno Deploy** | Solo JS/TS nativo. Rust solo via WASM, no servidor completo |
| **Dioxus Deploy** | No existe como producto. Dioxus Labs no ofrece hosting |
| **WispByte** | Diseñado para bots Discord/servidores de juegos, no web apps |
| **Juno (ICP)** | Basado en blockchain Internet Computer — costos impredecibles |

---

## 7. Arquitectura Recomendada: Enfoque Multi-Plataforma

Ninguna plataforma free cubre todas las necesidades. La estrategia óptima es **combinar fortalezas**:

```
┌──────────────────────────────────────────────────────────┐
│                    FRONTEND (estático/WASM)               │
│            Cloudflare Pages — GRATIS, ilimitado           │
│         CDN global, custom domains, SSL automático        │
└────────────────────────┬─────────────────────────────────┘
                         │ API calls / Server Functions
                         ▼
┌──────────────────────────────────────────────────────────┐
│                 BACKEND (Rust nativo)                     │
│  Proyecto 1 → Shuttle.rs    (0.5GB RAM, Axum nativo)     │
│  Proyecto 2 → Render Free   (0.5GB RAM, Rust nativo)     │
│  Proyecto 3 → Gigalixir Free (0.5GB RAM, Rust nativo)    │
│  Proyecto 4 → Railway Free  (Docker, $1/mes crédito)     │
│  Proyecto 5 → Koyeb Free    (Docker, 0.1 vCPU)           │
└────────────────────────┬─────────────────────────────────┘
                         │
                         ▼
┌──────────────────────────────────────────────────────────┐
│                    BASES DE DATOS                         │
│  Neon — 100 proyectos, 0.5GB cada uno, PostgreSQL        │
│  Supabase — 2 proyectos con Auth + Storage + Realtime     │
│  Turso — 100 DBs SQLite, 5GB total                       │
└──────────────────────────────────────────────────────────┘
```

### Mapeo por tipo de proyecto

| Tipo de proyecto | Frontend | Backend | Base de datos |
|---|---|---|---|
| Portfolio / marketing (solo CSR) | CF Pages | No necesita | No necesita |
| App web fullstack #1 | CF Pages | Shuttle | Neon |
| App web fullstack #2 | CF Pages | Render Free | Neon |
| App web fullstack #3 | CF Pages | Gigalixir Free | Neon |
| App web fullstack #4 | CF Pages | Railway Free | Neon |
| Desktop app (Dioxus) | N/A (local) | Shuttle o N/A | Neon |
| Mobile app (Dioxus) | N/A (APK/IPA) | Shuttle o Render | Neon o Supabase |

---

## 8. Limitaciones Globales del Stack Gratuito

### Lo que NO puedes hacer gratis

| ❌ No puedes | Por qué |
|---|---|
| Correr 5+ proyectos Rust backend simultáneos | Cada plataforma free limita a 1 proyecto |
| SSR sin cold starts | Render/Gigalixir duermen sin tráfico (~1 min reactivar) |
| Tener DB >0.5 GB gratis | Neon y Shuttle limitan storage |
| WebSocket persistente 24/7 | Los servicios que se apagan rompen conexiones |
| Build ilimitado de Rust en CI | 60-100 min/mes en Shuttle/Leapcell |
| Más de 1-2 GB egress/mes en backend | Shuttle y Leapcell limitan duro |
| Auto-scaling | Requiere planes pagos ($20-250/mes) |
| Zero-downtime deploys | Requiere Growth tier en Shuttle |

### Riesgos del modelo free

| Riesgo | Mitigación |
|---|---|
| Plataformas reducen/eliminan free tiers (ej: PlanetScale, Fly.io) | No depender de una sola plataforma; tener migración lista |
| Cold starts cascading (app + DB ambos dormidos) | Usar cron ping cada 5 min para mantener caliente |
| Créditos de build agotados (Rust compila lento) | Build local + push Docker image donde sea posible |
| Spot instance restarts (Shuttle) | Diseñar app stateless; usar DB externa (Neon) |
| DB expira cada 30 días (Render) | Usar Neon/SUPABASE como DB principal, no Render DB |

---

## 9. La Alternativa "Casi Gratis": VPS + Coolify

Si se dispone de **$3.50/mes** (un Hetzner CX22):

| Recurso | Lo que obtienes |
|---|---|
| **vCPU** | 2 (comparado con 0.1-0.25 en free tiers) |
| **RAM** | 4 GB (comparado con 0.5 GB en free tiers) |
| **SSD** | 40 GB |
| **Proyectos** | Ilimitados |
| **Cold starts** | Cero — siempre encendido |
| **DB** | Ilimitadas (Postgres, SQLite, lo que sea) |
| **Bandwidth** | 20 TB/mes |
| **Coolify** | Gratis, self-hosted — deploy con git push, SSL automático |

Esto equivale a tener **Cloudflare Pages + Fly.io + Neon combinados** por $3.50/mes.

---

## 10. Tabla Comparativa Final

| Criterio | CF Pages | Shuttle | Render | Gigalixir | Leapcell | Railway | Koyeb |
|---|---|---|---|---|---|---|---|
| **Gratis** | ✅ | ✅ | ✅ | ✅ | ✅ | ✅* | ✅ |
| **Sin tarjeta** | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Rust nativo** | WASM | ✅ | ✅ | ✅ | ✅ | Docker | Docker |
| **Dioxus fullstack** | No | workaround | probable | probable | limitado | Docker | Docker |
| **Proyectos free** | ∞ | 1 | 1 | 1 | 20 | 1 | 1 |
| **RAM** | N/A | 0.5GB | 0.5GB | 0.5GB | variable | 0.5GB | 0.5GB |
| **Cold start** | No | No† | 15min | 30 días | serverless | sí | sí |
| **DB incluida** | No | Postgres | Postgres (30d) | Postgres | Postgres+Redis | integrada | No |
| **Custom domain** | ✅ | 1 | ✅ | ✅ | ✅ | ✅ | 10 |
| **Build min/mes** | 500 | 100 | ∞ | ∞ | 60 | ~60 | ∞ |

*Railway: $1/mes en créditos (no acumulables)
†Shuttle: spot instances, reinicios ocasionales

---

## 11. Pendientes y Preguntas Abiertas

- [ ] ¿Se necesita WebSocket/LiveView? (impacta elección — cold starts rompen conexiones)
- [ ] ¿Cuál es el tráfico esperado por proyecto? (define si egress limits son suficientes)
- [ ] ¿Es tolerable un cold start de ~1 min? (afecta UX en primer visitante)
- [ ] ¿Se necesita filesystem persistente? (uploads, SQLite local)
- [ ] ¿Región geográfica del público? (Shuttle solo London, CF Pages global)
- [ ] ¿Frecuencia de deploys esperada? (100 build min/mes = ~10 deploys Rust)
- [ ] ¿Se consideraría $3.50/mes VPS + Coolify como alternativa?

---

## Fuentes

- Cloudflare Pages docs: https://developers.cloudflare.com/pages/
- Cloudflare Workers limits: https://developers.cloudflare.com/workers/platform/limits/
- Shuttle pricing: https://docs.shuttle.dev/pricing/overview
- Shuttle pricing update (Mar 2025): https://www.shuttle.dev/blog/2025/03/19/pricing-update
- Render free tier: https://render.com/docs/free
- Gigalixir free tier: https://gigalixir.com/docs/pricing
- Leapcell Rust hosting: https://leapcell.io
- Railway pricing: https://railway.com/pricing
- Koyeb pricing: https://koyeb.com/pricing
- Neon free tier: https://neon.tech/pricing
- Supabase free tier: https://supabase.com/pricing
- Turso free tier: https://turso.tech/pricing
- Dioxus deploy docs: https://dioxuslabs.com/learn/0.7/tutorial/deploy/
- `dioxus-cloudflare` crate: https://crates.io/crates/dioxus-cloudflare
- `cf-dioxus` examples: https://github.com/jongiddy/cf-dioxus
- Dioxus issue #3275 (WASM fullstack): https://github.com/DioxusLabs/dioxus/issues/3275
- Dioxus PR #3840 (axum_core wasm): https://github.com/DioxusLabs/dioxus/pull/3840
- Coolify: https://coolify.io
- Hetzner CX22: https://www.hetzner.com/cloud
