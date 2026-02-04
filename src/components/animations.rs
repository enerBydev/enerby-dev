//! Animations Module
//! CSS animation utilities and effects for cyberpunk theme

use dioxus::prelude::*;

/// Animation wrapper component for scroll-triggered fade-in (P12-A1)
#[component]
pub fn AnimateOnScroll(
    children: Element,
    #[props(default = "fade-in".to_string())] animation: String,
    #[props(default = 0)] delay_ms: u32,
) -> Element {
    // Note: Full intersection observer would require JS interop
    // For now, we use CSS animations that trigger on page load
    let delay_style = if delay_ms > 0 {
        format!("animation-delay: {}ms;", delay_ms)
    } else {
        String::new()
    };

    let animation_class = match animation.as_str() {
        "slide-up" => "animate-fade-in-up",
        "slide-left" => "animate-slide-left",
        "slide-right" => "animate-slide-right",
        "scale" => "animate-scale-in",
        _ => "animate-fade-in",
    };

    rsx! {
        div {
            class: "{animation_class}",
            style: "{delay_style}",
            {children}
        }
    }
}

/// Glitch effect wrapper (P12-B1)
#[component]
pub fn GlitchEffect(children: Element, #[props(default = false)] continuous: bool) -> Element {
    let class = if continuous {
        "glitch-continuous"
    } else {
        "glitch-hover"
    };

    rsx! {
        div { class: "relative inline-block {class}",
            {children}
        }
    }
}

/// Neon flicker effect (P12-B3)
#[component]
pub fn NeonFlicker(
    children: Element,
    #[props(default = "cyan".to_string())] color: String,
) -> Element {
    let color_class = match color.as_str() {
        "pink" => "neon-flicker-pink",
        "purple" => "neon-flicker-purple",
        _ => "neon-flicker-cyan",
    };

    rsx! {
        span { class: "{color_class}",
            {children}
        }
    }
}

/// Staggered animation container for lists (P12-A3)
#[component]
pub fn StaggerContainer(children: Element, #[props(default = 100)] stagger_ms: u32) -> Element {
    // Each child would get increasing delay
    // Implementation note: Full implementation would require enumerating children
    rsx! {
        div { class: "stagger-container",
            style: "--stagger-delay: {stagger_ms}ms;",
            {children}
        }
    }
}

/// Floating animation (P12-D3)
#[component]
pub fn FloatingElement(children: Element, #[props(default = 0)] offset: i32) -> Element {
    let delay = format!("animation-delay: {}ms;", offset * 200);

    rsx! {
        div {
            class: "animate-float",
            style: "{delay}",
            {children}
        }
    }
}

/// Pulse glow effect
#[component]
pub fn PulseGlow(
    children: Element,
    #[props(default = "cyan".to_string())] color: String,
) -> Element {
    let shadow_color = match color.as_str() {
        "pink" => "rgba(255, 0, 255, 0.5)",
        "purple" => "rgba(157, 0, 255, 0.5)",
        _ => "rgba(0, 255, 255, 0.5)",
    };

    rsx! {
        div {
            class: "animate-pulse-glow",
            style: "--glow-color: {shadow_color};",
            {children}
        }
    }
}

/// RGB Split effect on hover (P12-B5)
#[component]
pub fn RgbSplit(children: Element) -> Element {
    rsx! {
        div { class: "rgb-split-hover",
            {children}
        }
    }
}

/// Page transition wrapper (P12-C5)
#[component]
pub fn PageTransition(children: Element) -> Element {
    rsx! {
        div { class: "animate-page-enter",
            {children}
        }
    }
}

// Additional CSS that should be added to main.css for these animations:
/*
@keyframes fade-in-up {
    from { opacity: 0; transform: translateY(20px); }
    to { opacity: 1; transform: translateY(0); }
}

@keyframes float {
    0%, 100% { transform: translateY(0); }
    50% { transform: translateY(-10px); }
}

@keyframes pulse-glow {
    0%, 100% { box-shadow: 0 0 5px var(--glow-color); }
    50% { box-shadow: 0 0 20px var(--glow-color), 0 0 30px var(--glow-color); }
}

@keyframes glitch-1 {
    0%, 100% { transform: translate(0); }
    20% { transform: translate(-2px, 2px); }
    40% { transform: translate(2px, -2px); }
    60% { transform: translate(-2px, -2px); }
    80% { transform: translate(2px, 2px); }
}

.animate-fade-in-up { animation: fade-in-up 0.6s ease-out forwards; }
.animate-float { animation: float 3s ease-in-out infinite; }
.animate-pulse-glow { animation: pulse-glow 2s ease-in-out infinite; }
*/
