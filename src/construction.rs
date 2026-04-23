//! Construction Overlay Component
//! Full-screen under-construction page for site maintenance mode

use dioxus::prelude::*;

const CONSTRUCTION_CSS: &str = r#"
@keyframes construction-pulse {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.8; }
}

@keyframes construction-glitch {
    0%, 90%, 100% { transform: translate(0); }
    91% { transform: translate(-2px, 0); }
    92% { transform: translate(2px, 0); }
    93% { transform: translate(0); }
    94% { transform: translate(0, 1px); }
    95% { transform: translate(0); }
}

@keyframes construction-scan {
    0% { left: -33%; }
    100% { left: 100%; }
}

@keyframes construction-dot {
    0%, 80%, 100% {
        transform: scale(0.6);
        opacity: 0.5;
    }
    40% {
        transform: scale(1);
        opacity: 1;
    }
}

.glitch-text-construction {
    position: relative;
}

.glitch-text-construction::before,
.glitch-text-construction::after {
    content: attr(data-text);
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
}

.glitch-text-construction::before {
    left: 2px;
    text-shadow: -2px 0 #FF00FF;
    clip: rect(0, 900px, 0, 0);
    animation: construction-glitch-1 2s infinite linear alternate-reverse;
}

.glitch-text-construction::after {
    left: -2px;
    text-shadow: -2px 0 #00FFFF;
    clip: rect(0, 900px, 0, 0);
    animation: construction-glitch-2 3s infinite linear alternate-reverse;
}

@keyframes construction-glitch-1 {
    0% { clip: rect(20px, 9999px, 10px, 0); }
    20% { clip: rect(60px, 9999px, 70px, 0); }
    40% { clip: rect(30px, 9999px, 5px, 0); }
    60% { clip: rect(80px, 9999px, 50px, 0); }
    80% { clip: rect(10px, 9999px, 40px, 0); }
    100% { clip: rect(50px, 9999px, 90px, 0); }
}

@keyframes construction-glitch-2 {
    0% { clip: rect(40px, 9999px, 20px, 0); }
    20% { clip: rect(10px, 9999px, 80px, 0); }
    40% { clip: rect(70px, 9999px, 30px, 0); }
    60% { clip: rect(20px, 9999px, 60px, 0); }
    80% { clip: rect(50px, 9999px, 10px, 0); }
    100% { clip: rect(30px, 9999px, 70px, 0); }
}
"#;

/// Construction Overlay - Blocks entire site with cyberpunk coming-soon screen
#[component]
pub fn ConstructionOverlay() -> Element {
    rsx! {
        // Full-screen fixed overlay
        div {
            class: "fixed inset-0 w-screen h-screen z-50 flex items-center justify-center",
            style: "background-color: #0A0A0F;",

            // Animated grid background
            div {
                class: "absolute inset-0 opacity-10",
                style: r#"
                    background-image:
                        linear-gradient(rgba(0, 255, 255, 0.1) 1px, transparent 1px),
                        linear-gradient(90deg, rgba(0, 255, 255, 0.1) 1px, transparent 1px);
                    background-size: 50px 50px;
                "#
            }

            // Radial glow effect
            div {
                class: "absolute inset-0 pointer-events-none",
                style: r#"
                    background: radial-gradient(ellipse at center, rgba(0, 255, 255, 0.05) 0%, transparent 70%);
                "#
            }

            // Main content container
            div {
                class: "relative z-10 flex flex-col items-center text-center px-6",

                // Site name (subtle, top)
                p {
                    class: "text-xs uppercase tracking-[0.5em] text-white/30 mb-8 font-mono",
                    "enerby.dev"
                }

                // Main heading with glitch class
                h1 {
                    class: "glitch-text-construction text-5xl md:text-7xl lg:text-8xl font-display font-bold mb-6",
                    style: r#"
                        color: #00FFFF;
                        text-shadow: 0 0 20px rgba(0, 255, 255, 0.5), 0 0 40px rgba(0, 255, 255, 0.3);
                        animation: construction-pulse 2s ease-in-out infinite, construction-glitch 3s infinite;
                    "#,
                    "data-text": "EN CONSTRUCCIÓN",
                    "EN CONSTRUCCIÓN"
                }

                // Subtitle
                p {
                    class: "text-2xl md:text-3xl font-light mb-12",
                    style: "color: rgba(255, 255, 255, 0.7);",
                    "Próximamente"
                }

                // Animated scanning line
                div {
                    class: "w-48 md:w-64 h-0.5 mb-12 relative overflow-hidden",
                    style: "background-color: rgba(0, 255, 255, 0.2);",
                    div {
                        class: "absolute inset-y-0 w-1/3",
                        style: r#"
                            background: linear-gradient(90deg, transparent, #00FFFF, transparent);
                            animation: construction-scan 2s linear infinite;
                        "#
                    }
                }

                // Pulsing dots indicator
                div {
                    class: "flex gap-2 mb-8",
                    div {
                        class: "w-3 h-3 rounded-full",
                        style: "background-color: #00FFFF; animation: construction-dot 1.4s ease-in-out infinite;"
                    }
                    div {
                        class: "w-3 h-3 rounded-full",
                        style: "background-color: #00FFFF; animation: construction-dot 1.4s ease-in-out 0.2s infinite;"
                    }
                    div {
                        class: "w-3 h-3 rounded-full",
                        style: "background-color: #00FFFF; animation: construction-dot 1.4s ease-in-out 0.4s infinite;"
                    }
                }

                // Footer text
                p {
                    class: "text-sm font-mono",
                    style: "color: rgba(157, 0, 255, 0.8);",
                    "Reconstruyendo el futuro digital"
                }
            }

            // Corner decorations
            div {
                class: "absolute top-8 left-8 w-16 h-16 border-l-2 border-t-2",
                style: "border-color: rgba(0, 255, 255, 0.3);"
            }
            div {
                class: "absolute top-8 right-8 w-16 h-16 border-r-2 border-t-2",
                style: "border-color: rgba(0, 255, 255, 0.3);"
            }
            div {
                class: "absolute bottom-8 left-8 w-16 h-16 border-l-2 border-b-2",
                style: "border-color: rgba(157, 0, 255, 0.3);"
            }
            div {
                class: "absolute bottom-8 right-8 w-16 h-16 border-r-2 border-b-2",
                style: "border-color: rgba(157, 0, 255, 0.3);"
            }
        }

        // CSS animations inline (since this is a standalone overlay)
        style { dangerous_inner_html: CONSTRUCTION_CSS }
    }
}