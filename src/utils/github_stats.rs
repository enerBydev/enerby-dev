//! GitHub Statistics Module
//! Provides Lines of Code statistics for the portfolio
//!
//! Current implementation: Static mock with realistic values
//! Future: GitHub API integration for real-time stats

/// Statistics from GitHub repositories
#[derive(Debug, Clone)]
pub struct GitHubStats {
    /// Total lines of code across all tracked repos
    pub total_loc: u64,
    /// List of tracked repository names
    pub repos: Vec<String>,
}

/// Configuration for which repos to track
pub const TRACKED_REPOS: &[&str] = &[
    "enerBydev/enerby-dev",
    "enerBydev/oc_diagdoc",
    "enerBydev/Affinity-Legacy-Bridge",
    "enerBydev/VideoGIniusAI",
];

/// Get current GitHub statistics
///
/// # Current Implementation
/// Returns mock data with realistic estimates.
///
/// # Future Implementation
/// Will use GitHub API to calculate real LOC:
/// - Fetch repo stats via REST API
/// - Cache results to avoid rate limiting
/// - Fallback to static values on error
pub fn get_github_stats() -> GitHubStats {
    // Mock values based on realistic estimates:
    // - enerby-dev: ~8,000 LOC (Rust/CSS portfolio)
    // - oc_diagdoc: ~12,000 LOC (Rust CLI tool)
    // - Affinity-Legacy-Bridge: ~500 LOC (Shell scripts)
    // - VideoGIniusAI: ~15,000 LOC (Nuxt 3/Vue)
    // Total: ~35,500 LOC

    GitHubStats {
        total_loc: 35_847, // Realistic mock value based on real repos
        repos: TRACKED_REPOS.iter().map(|s| s.to_string()).collect(),
    }
}

/// Format LOC with thousand separators
/// e.g., 42847 -> "42,847"
pub fn format_loc(loc: u64) -> String {
    let s = loc.to_string();
    let chars: Vec<char> = s.chars().rev().collect();
    let formatted: String = chars
        .chunks(3)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join(",");
    formatted.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_loc() {
        assert_eq!(format_loc(1000), "1,000");
        assert_eq!(format_loc(42847), "42,847");
        assert_eq!(format_loc(1234567), "1,234,567");
        assert_eq!(format_loc(100), "100");
    }

    #[test]
    fn test_get_github_stats() {
        let stats = get_github_stats();
        assert!(stats.total_loc > 0);
        assert!(!stats.repos.is_empty());
    }
}
