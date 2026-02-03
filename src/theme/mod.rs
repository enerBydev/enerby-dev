//! enerby.dev - Design System Theme
//! Cyberpunk/Neon aesthetic implementation

/// Cyberpunk color palette
pub mod colors {
    // Primary Colors
    pub const CYAN_NEON: &str = "#00FFFF";
    pub const CYAN_DARK: &str = "#00B8B8";
    pub const CYAN_LIGHT: &str = "#7FFFFF";
    
    // Secondary Colors  
    pub const PINK_NEON: &str = "#FF00FF";
    pub const PURPLE_ELECTRIC: &str = "#9D00FF";
    pub const ORANGE_SUNSET: &str = "#FF6B00";
    
    // Background Colors
    pub const BG_DARK: &str = "#0A0A0F";
    pub const BG_DARKER: &str = "#050508";
    pub const BG_SURFACE: &str = "#14141F";
    pub const BG_ELEVATED: &str = "#1E1E2E";
    
    // Text Colors
    pub const TEXT_PRIMARY: &str = "#FFFFFF";
    pub const TEXT_SECONDARY: &str = "#B0B0C0";
    pub const TEXT_MUTED: &str = "#6B6B80";
    
    // Status Colors
    pub const SUCCESS: &str = "#00FF88";
    pub const WARNING: &str = "#FFB800";
    pub const ERROR: &str = "#FF3366";
}

/// Typography configuration
pub mod typography {
    pub const FONT_DISPLAY: &str = "'Orbitron', sans-serif";
    pub const FONT_BODY: &str = "'Inter', system-ui, sans-serif";
    pub const FONT_MONO: &str = "'JetBrains Mono', monospace";
    
    // Font sizes (rem)
    pub const SIZE_XS: &str = "0.75rem";   // 12px
    pub const SIZE_SM: &str = "0.875rem";  // 14px
    pub const SIZE_BASE: &str = "1rem";    // 16px
    pub const SIZE_LG: &str = "1.125rem";  // 18px
    pub const SIZE_XL: &str = "1.25rem";   // 20px
    pub const SIZE_2XL: &str = "1.5rem";   // 24px
    pub const SIZE_3XL: &str = "2rem";     // 32px
    pub const SIZE_4XL: &str = "2.5rem";   // 40px
    pub const SIZE_5XL: &str = "3rem";     // 48px
}

/// Spacing tokens (4px base)
pub mod spacing {
    pub const SP_0: &str = "0";
    pub const SP_1: &str = "0.25rem";  // 4px
    pub const SP_2: &str = "0.5rem";   // 8px
    pub const SP_3: &str = "0.75rem";  // 12px
    pub const SP_4: &str = "1rem";     // 16px
    pub const SP_5: &str = "1.25rem";  // 20px
    pub const SP_6: &str = "1.5rem";   // 24px
    pub const SP_8: &str = "2rem";     // 32px
    pub const SP_10: &str = "2.5rem";  // 40px
    pub const SP_12: &str = "3rem";    // 48px
    pub const SP_16: &str = "4rem";    // 64px
}

/// Breakpoints (min-width)
pub mod breakpoints {
    pub const SM: &str = "640px";   // Mobile landscape
    pub const MD: &str = "768px";   // Tablet portrait
    pub const LG: &str = "1024px";  // Tablet landscape / Small desktop
    pub const XL: &str = "1280px";  // Desktop
    pub const XXL: &str = "1536px"; // Large desktop
}

/// Animation durations and easings
pub mod animations {
    pub const DURATION_FAST: &str = "150ms";
    pub const DURATION_NORMAL: &str = "300ms";
    pub const DURATION_SLOW: &str = "500ms";
    pub const DURATION_SLOWER: &str = "700ms";
    
    pub const EASE_OUT: &str = "cubic-bezier(0, 0, 0.2, 1)";
    pub const EASE_IN: &str = "cubic-bezier(0.4, 0, 1, 1)";
    pub const EASE_IN_OUT: &str = "cubic-bezier(0.4, 0, 0.2, 1)";
    pub const EASE_BOUNCE: &str = "cubic-bezier(0.34, 1.56, 0.64, 1)";
}

/// Cyberpunk visual effects
pub mod effects {
    pub const SCANLINES: &str = "repeating-linear-gradient(
        0deg,
        transparent,
        transparent 2px,
        rgba(0, 0, 0, 0.1) 2px,
        rgba(0, 0, 0, 0.1) 4px
    )";
    
    pub const GLASS: &str = "
        background: rgba(20, 20, 30, 0.7);
        backdrop-filter: blur(10px);
        border: 1px solid rgba(0, 255, 255, 0.2);
    ";
    
    pub const NEON_BORDER: &str = "
        border: 2px solid #00FFFF;
        box-shadow: 0 0 10px #00FFFF40, inset 0 0 10px #00FFFF20;
    ";
    
    pub const GLOW_TEXT: &str = "
        text-shadow: 0 0 10px #00FFFF, 0 0 20px #00FFFF, 0 0 30px #00FFFF;
    ";
}
