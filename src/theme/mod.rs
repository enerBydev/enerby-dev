//! enerby.dev - Design System Theme
//! Cyberpunk/Neon aesthetic implementation in Rust
//! Reference: design_system.md

// ============================================
// Color Palette
// ============================================

/// Cyberpunk color palette - Primary Cyan Neon
pub mod colors {
    // === Primary Colors - Cyan Neon ===
    pub const PRIMARY: &str = "#00FFFF";
    pub const PRIMARY_DARK: &str = "#00CCCC";
    pub const PRIMARY_LIGHT: &str = "#66FFFF";
    pub const PRIMARY_GLOW: &str = "rgba(0, 255, 255, 0.5)";

    // === Secondary Colors - Accent Neon ===
    pub const SECONDARY_PINK: &str = "#FF00FF";
    pub const SECONDARY_PURPLE: &str = "#9D00FF";
    pub const SECONDARY_ORANGE: &str = "#FF6600";

    // === Background Colors - Dark Mode ===
    pub const BG_PRIMARY: &str = "#0A0A0F";
    pub const BG_SECONDARY: &str = "#12121A";
    pub const BG_TERTIARY: &str = "#1A1A25";
    pub const BG_CARD: &str = "rgba(20, 20, 30, 0.7)";
    pub const BG_ELEVATED: &str = "#1E1E2E";

    // === Text Colors ===
    pub const TEXT_PRIMARY: &str = "#FFFFFF";
    pub const TEXT_SECONDARY: &str = "#B0B0C0";
    pub const TEXT_MUTED: &str = "#6B6B7B";

    // === Status Colors ===
    pub const SUCCESS: &str = "#00FF88";
    pub const WARNING: &str = "#FFB800";
    pub const ERROR: &str = "#FF3366";
}

// ============================================
// Typography
// ============================================

/// Typography configuration
pub mod typography {
    // Font Families
    pub const FONT_DISPLAY: &str = "'Orbitron', sans-serif";
    pub const FONT_BODY: &str = "'Inter', system-ui, -apple-system, sans-serif";
    pub const FONT_MONO: &str = "'JetBrains Mono', 'Fira Code', monospace";

    // Font Sizes (rem)
    pub const SIZE_XS: &str = "0.75rem"; // 12px
    pub const SIZE_SM: &str = "0.875rem"; // 14px
    pub const SIZE_BASE: &str = "1rem"; // 16px
    pub const SIZE_LG: &str = "1.125rem"; // 18px
    pub const SIZE_XL: &str = "1.25rem"; // 20px
    pub const SIZE_2XL: &str = "1.5rem"; // 24px
    pub const SIZE_3XL: &str = "2rem"; // 32px
    pub const SIZE_4XL: &str = "2.5rem"; // 40px
    pub const SIZE_5XL: &str = "3rem"; // 48px
    pub const SIZE_6XL: &str = "4rem"; // 64px

    // Font Weights
    pub const WEIGHT_NORMAL: &str = "400";
    pub const WEIGHT_MEDIUM: &str = "500";
    pub const WEIGHT_SEMIBOLD: &str = "600";
    pub const WEIGHT_BOLD: &str = "700";
}

// ============================================
// Spacing
// ============================================

/// Spacing tokens (4px base)
pub mod spacing {
    pub const SP_0: &str = "0";
    pub const SP_1: &str = "0.25rem"; // 4px
    pub const SP_2: &str = "0.5rem"; // 8px
    pub const SP_3: &str = "0.75rem"; // 12px
    pub const SP_4: &str = "1rem"; // 16px
    pub const SP_5: &str = "1.25rem"; // 20px
    pub const SP_6: &str = "1.5rem"; // 24px
    pub const SP_8: &str = "2rem"; // 32px
    pub const SP_10: &str = "2.5rem"; // 40px
    pub const SP_12: &str = "3rem"; // 48px
    pub const SP_16: &str = "4rem"; // 64px
    pub const SP_20: &str = "5rem"; // 80px
    pub const SP_24: &str = "6rem"; // 96px
}

// ============================================
// Layout
// ============================================

/// Container widths
pub mod container {
    pub const MAX: &str = "1280px";
    pub const NARROW: &str = "768px";
    pub const WIDE: &str = "1536px";
}

/// Breakpoints (min-width)
pub mod breakpoints {
    pub const SM: &str = "640px"; // Mobile landscape
    pub const MD: &str = "768px"; // Tablet portrait
    pub const LG: &str = "1024px"; // Tablet landscape / Small desktop
    pub const XL: &str = "1280px"; // Desktop
    pub const XXL: &str = "1536px"; // Large desktop
}

/// Border radius tokens
pub mod radius {
    pub const SM: &str = "4px";
    pub const MD: &str = "8px";
    pub const LG: &str = "12px";
    pub const XL: &str = "16px";
    pub const FULL: &str = "9999px";
}

// ============================================
// Animations
// ============================================

/// Animation durations and easings
pub mod animations {
    // Durations
    pub const DURATION_FAST: &str = "150ms";
    pub const DURATION_NORMAL: &str = "300ms";
    pub const DURATION_SLOW: &str = "500ms";
    pub const DURATION_SLOWER: &str = "700ms";

    // Easings
    pub const EASE_OUT: &str = "cubic-bezier(0, 0, 0.2, 1)";
    pub const EASE_IN: &str = "cubic-bezier(0.4, 0, 1, 1)";
    pub const EASE_IN_OUT: &str = "cubic-bezier(0.4, 0, 0.2, 1)";
    pub const EASE_BOUNCE: &str = "cubic-bezier(0.34, 1.56, 0.64, 1)";
}

// ============================================
// Visual Effects
// ============================================

/// Cyberpunk visual effects
pub mod effects {
    // Shadows
    pub const SHADOW_GLOW_SM: &str = "0 0 10px rgba(0, 255, 255, 0.4)";
    pub const SHADOW_GLOW_MD: &str = "0 0 20px rgba(0, 255, 255, 0.5)";
    pub const SHADOW_GLOW_LG: &str =
        "0 0 30px rgba(0, 255, 255, 0.5), 0 0 60px rgba(0, 255, 255, 0.3)";
    pub const SHADOW_NEON: &str = "0 0 10px #00FFFF40, inset 0 0 10px #00FFFF20";

    // Scanlines overlay
    pub const SCANLINES: &str = "repeating-linear-gradient(
        0deg,
        transparent,
        transparent 2px,
        rgba(0, 0, 0, 0.1) 2px,
        rgba(0, 0, 0, 0.1) 4px
    )";

    // Glass effect (CSS)
    pub const GLASS_BG: &str = "rgba(20, 20, 30, 0.7)";
    pub const GLASS_BLUR: &str = "blur(10px)";
    pub const GLASS_BORDER: &str = "1px solid rgba(0, 255, 255, 0.2)";

    // Neon border
    pub const NEON_BORDER: &str = "2px solid #00FFFF";

    // Text glow
    pub const TEXT_GLOW: &str = "0 0 10px #00FFFF, 0 0 20px #00FFFF, 0 0 30px #00FFFF";
}

/// Gradient presets
pub mod gradients {
    // Background gradients
    pub const BG_RADIAL: &str = "radial-gradient(ellipse at center, #12121A 0%, #0A0A0F 100%)";
    pub const BG_DARK: &str = "linear-gradient(180deg, #0A0A0F 0%, #12121A 100%)";

    // Neon gradients
    pub const NEON_CYAN_PINK: &str = "linear-gradient(135deg, #00FFFF 0%, #FF00FF 100%)";
    pub const NEON_CYAN_PURPLE: &str = "linear-gradient(135deg, #00FFFF 0%, #9D00FF 100%)";
    pub const NEON_PINK_ORANGE: &str = "linear-gradient(135deg, #FF00FF 0%, #FF6600 100%)";

    // Underline gradient
    pub const UNDERLINE: &str = "linear-gradient(90deg, #00FFFF, transparent)";
}

// ============================================
// Helpers
// ============================================

/// Generate CSS class string for common patterns
pub mod helpers {
    use super::*;

    /// Get primary color with optional alpha
    pub fn primary_rgba(alpha: f32) -> String {
        format!("rgba(0, 255, 255, {})", alpha)
    }

    /// Get secondary pink with optional alpha
    pub fn pink_rgba(alpha: f32) -> String {
        format!("rgba(255, 0, 255, {})", alpha)
    }

    /// Get secondary purple with optional alpha
    pub fn purple_rgba(alpha: f32) -> String {
        format!("rgba(157, 0, 255, {})", alpha)
    }

    /// Generate box-shadow glow with custom intensity
    pub fn glow_shadow(intensity: u8) -> String {
        let base = intensity as f32 / 100.0;
        format!(
            "0 0 {}px rgba(0, 255, 255, {}), 0 0 {}px rgba(0, 255, 255, {})",
            10 + intensity / 5,
            0.4 + base * 0.2,
            20 + intensity / 3,
            0.2 + base * 0.1
        )
    }
}
