//! GitHub API Client Module
//!
//! This module provides functionality to fetch repository information from the GitHub API,
//! including the homepage field which can be used as a demo URL for projects.
//!
//! # Architecture
//!
//! Due to WASM limitations (no direct HTTP in browser without special handling),
//! this module uses a **static data approach** where GitHub data is pre-fetched
//! and embedded at build time. This ensures:
//! - Zero runtime HTTP calls from WASM
//! - Instant data availability
//! - Graceful fallback if data is stale
//!
//! # Example
//! ```rust,ignore
//! use crate::utils::github_api::{GitHubRepoInfo, get_repo_homepage};
//!
//! // Get homepage for a known repo
//! let demo_url = get_repo_homepage("enerBydev", "enerby.dev");
//! ```

use serde::{Deserialize, Serialize};
use std::fmt;

// ============================================================================
// CONSTANTS & CONFIGURATION
// ============================================================================

/// Base URL for GitHub REST API v3
pub const GITHUB_API_BASE_URL: &str = "https://api.github.com";

/// User-Agent header required by GitHub API
/// Must be a valid string identifying the application
pub const USER_AGENT: &str = "enerby-dev-portfolio/1.0";

/// Default timeout for API requests in milliseconds
pub const API_TIMEOUT_MS: u64 = 10_000;

/// Configuration for GitHub API client
#[derive(Debug, Clone)]
pub struct GitHubApiConfig {
    /// Base URL for API requests
    pub base_url: String,
    /// User-Agent header value
    pub user_agent: String,
    /// Request timeout in milliseconds
    pub timeout_ms: u64,
}

impl Default for GitHubApiConfig {
    fn default() -> Self {
        Self {
            base_url: GITHUB_API_BASE_URL.to_string(),
            user_agent: USER_AGENT.to_string(),
            timeout_ms: API_TIMEOUT_MS,
        }
    }
}

// ============================================================================
// ERROR TYPES
// ============================================================================

/// Errors that can occur when interacting with the GitHub API
#[derive(Debug, Clone, PartialEq)]
pub enum ApiError {
    /// Network error (connection failed, timeout, etc.)
    NetworkError(String),
    /// Repository not found (404)
    NotFound,
    /// Rate limit exceeded (403 with rate limit headers)
    RateLimited,
    /// Failed to parse JSON response
    ParseError(String),
    /// Repository is private or access denied
    Forbidden,
    /// Generic server error
    ServerError(u16),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            ApiError::NotFound => write!(f, "Repository not found"),
            ApiError::RateLimited => write!(f, "GitHub API rate limit exceeded"),
            ApiError::ParseError(msg) => write!(f, "Failed to parse response: {}", msg),
            ApiError::Forbidden => write!(f, "Access forbidden"),
            ApiError::ServerError(code) => write!(f, "Server error: {}", code),
        }
    }
}

impl std::error::Error for ApiError {}

// ============================================================================
// DATA STRUCTURES
// ============================================================================

/// Represents repository information from GitHub API
///
/// This struct maps to the JSON response from:
/// `GET https://api.github.com/repos/{owner}/{repo}`
///
/// Only includes fields relevant to the portfolio application.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct GitHubRepoInfo {
    /// Repository name (e.g., "enerby.dev")
    pub name: String,

    /// Full name including owner (e.g., "enerBydev/enerby.dev")
    pub full_name: String,

    /// Repository description (can be null)
    #[serde(default)]
    pub description: Option<String>,

    /// Homepage URL configured in repository settings
    /// This is the key field for demo_url auto-detection
    #[serde(default)]
    pub homepage: Option<String>,

    /// Primary programming language (e.g., "Rust")
    #[serde(default)]
    pub language: Option<String>,

    /// Number of stars
    #[serde(default)]
    pub stargazers_count: u32,

    /// Number of forks
    #[serde(default)]
    pub forks_count: u32,

    /// Whether the repository is a fork
    #[serde(default)]
    pub fork: bool,

    /// Whether the repository is archived
    #[serde(default)]
    pub archived: bool,

    /// Repository topics/tags
    #[serde(default)]
    pub topics: Vec<String>,

    /// HTML URL to the repository
    pub html_url: String,
}

impl GitHubRepoInfo {
    /// Extracts a valid homepage URL if present
    ///
    /// Returns `Some(url)` if:
    /// - homepage is not None
    /// - homepage is not empty
    /// - homepage starts with "http://" or "https://"
    ///
    /// # Example
    /// ```rust,ignore
    /// let repo = GitHubRepoInfo { homepage: Some("https://enerby.dev".to_string()), ... };
    /// assert_eq!(repo.extract_homepage(), Some("https://enerby.dev".to_string()));
    /// ```
    pub fn extract_homepage(&self) -> Option<String> {
        self.homepage.as_ref().and_then(|url| {
            let trimmed = url.trim();
            if trimmed.is_empty() {
                None
            } else if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
                Some(trimmed.to_string())
            } else {
                None
            }
        })
    }

    /// Creates a minimal GitHubRepoInfo for testing or fallback
    pub fn minimal(owner: &str, name: &str) -> Self {
        Self {
            name: name.to_string(),
            full_name: format!("{}/{}", owner, name),
            description: None,
            homepage: None,
            language: None,
            stargazers_count: 0,
            forks_count: 0,
            fork: false,
            archived: false,
            topics: vec![],
            html_url: format!("https://github.com/{}/{}", owner, name),
        }
    }
}

// ============================================================================
// STATIC DATA - Pre-fetched GitHub repository information
// ============================================================================
// This data is embedded at compile time for WASM compatibility.
// Update this data periodically or via build script.

/// Static repository data for enerBydev projects
/// Last updated: 2026-02-05
fn get_static_repo_data() -> Vec<GitHubRepoInfo> {
    vec![
        GitHubRepoInfo {
            name: "enerby.dev".to_string(),
            full_name: "enerBydev/enerby.dev".to_string(),
            description: Some("Personal portfolio website built with Rust + Dioxus".to_string()),
            homepage: Some("https://enerby.dev".to_string()),
            language: Some("Rust".to_string()),
            stargazers_count: 0,
            forks_count: 0,
            fork: false,
            archived: false,
            topics: vec![
                "rust".to_string(),
                "dioxus".to_string(),
                "portfolio".to_string(),
                "wasm".to_string(),
            ],
            html_url: "https://github.com/enerBydev/enerby.dev".to_string(),
        },
        GitHubRepoInfo {
            name: "oc_diagdoc".to_string(),
            full_name: "enerBydev/oc_diagdoc".to_string(),
            description: Some(
                "command-line-utilities, text-processing, development-tools".to_string(),
            ),
            homepage: Some("https://www.google.com".to_string()), // Placeholder confirmed on GitHub
            language: Some("Rust".to_string()),
            stargazers_count: 0,
            forks_count: 0,
            fork: false,
            archived: false,
            topics: vec!["rust".to_string()],
            html_url: "https://github.com/enerBydev/oc_diagdoc".to_string(),
        },
        GitHubRepoInfo {
            name: "nvim-config".to_string(),
            full_name: "enerBydev/nvim-config".to_string(),
            description: Some("Personal Neovim configuration".to_string()),
            homepage: None,
            language: Some("Lua".to_string()),
            stargazers_count: 0,
            forks_count: 0,
            fork: false,
            archived: false,
            topics: vec![
                "neovim".to_string(),
                "lua".to_string(),
                "dotfiles".to_string(),
            ],
            html_url: "https://github.com/enerBydev/nvim-config".to_string(),
        },
        GitHubRepoInfo {
            name: "rust_projects".to_string(),
            full_name: "enerBydev/rust_projects".to_string(),
            description: Some("Collection of Rust learning projects".to_string()),
            homepage: None,
            language: Some("Rust".to_string()),
            stargazers_count: 0,
            forks_count: 0,
            fork: false,
            archived: false,
            topics: vec!["rust".to_string(), "learning".to_string()],
            html_url: "https://github.com/enerBydev/rust_projects".to_string(),
        },
    ]
}

// ============================================================================
// PUBLIC API FUNCTIONS
// ============================================================================

/// Get repository info from static data
///
/// This function searches the pre-loaded static data for a matching repository.
/// It's the primary way to get repo info in WASM context.
///
/// # Arguments
/// * `owner` - Repository owner (e.g., "enerBydev")
/// * `repo` - Repository name (e.g., "enerby.dev")
///
/// # Returns
/// * `Ok(GitHubRepoInfo)` if found
/// * `Err(ApiError::NotFound)` if not in static data
pub fn get_repo_info(owner: &str, repo: &str) -> Result<GitHubRepoInfo, ApiError> {
    let full_name = format!("{}/{}", owner, repo);
    let full_name_lower = full_name.to_lowercase();

    get_static_repo_data()
        .into_iter()
        .find(|r| r.full_name.to_lowercase() == full_name_lower)
        .ok_or(ApiError::NotFound)
}

/// Get homepage URL for a repository
///
/// Convenience function that combines `get_repo_info` and `extract_homepage`.
///
/// # Arguments
/// * `owner` - Repository owner
/// * `repo` - Repository name
///
/// # Returns
/// * `Some(url)` if repository has a valid homepage
/// * `None` if not found or no homepage configured
pub fn get_repo_homepage(owner: &str, repo: &str) -> Option<String> {
    get_repo_info(owner, repo)
        .ok()
        .and_then(|info| info.extract_homepage())
}

/// Get all static repository info
///
/// Returns all pre-loaded repository data.
pub fn get_all_repos() -> Vec<GitHubRepoInfo> {
    get_static_repo_data()
}

/// Fetch multiple repositories (sync version using static data)
///
/// # Arguments
/// * `repos` - Slice of (owner, repo) tuples
///
/// # Returns
/// * Vector of Results for each requested repo
pub fn fetch_multiple_repos(repos: &[(&str, &str)]) -> Vec<Result<GitHubRepoInfo, ApiError>> {
    repos
        .iter()
        .map(|(owner, repo)| get_repo_info(owner, repo))
        .collect()
}

/// Parse owner and repo from a GitHub URL
///
/// # Arguments
/// * `github_url` - Full GitHub URL (e.g., "https://github.com/owner/repo")
///
/// # Returns
/// * `Some((owner, repo))` if valid GitHub URL
/// * `None` if invalid format
pub fn parse_github_url(github_url: &str) -> Option<(String, String)> {
    let url = github_url.trim();

    // Handle different URL formats
    let path = if url.starts_with("https://github.com/") {
        url.strip_prefix("https://github.com/")
    } else if url.starts_with("http://github.com/") {
        url.strip_prefix("http://github.com/")
    } else if url.starts_with("github.com/") {
        url.strip_prefix("github.com/")
    } else {
        None
    }?;

    // Split path into owner/repo
    let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();

    if parts.len() >= 2 {
        Some((parts[0].to_string(), parts[1].to_string()))
    } else {
        None
    }
}

/// Get homepage from a GitHub URL string
///
/// Parses the URL, looks up the repo, and extracts homepage if available.
///
/// # Arguments
/// * `github_url` - Full GitHub URL
///
/// # Returns
/// * `Some(homepage_url)` if found and valid
/// * `None` otherwise
pub fn get_homepage_from_url(github_url: &str) -> Option<String> {
    parse_github_url(github_url).and_then(|(owner, repo)| get_repo_homepage(&owner, &repo))
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------
    // GitHubRepoInfo Tests
    // -------------------------

    /// Test: Basic JSON deserialization with all fields
    #[test]
    fn test_deserialize_github_repo_info_full() {
        let json = r#"{
            "name": "enerby.dev",
            "full_name": "enerBydev/enerby.dev",
            "description": "Personal portfolio website",
            "homepage": "https://enerby.dev",
            "language": "Rust",
            "stargazers_count": 42,
            "forks_count": 5,
            "fork": false,
            "archived": false,
            "topics": ["rust", "dioxus", "portfolio"],
            "html_url": "https://github.com/enerBydev/enerby.dev"
        }"#;

        let repo: GitHubRepoInfo = serde_json::from_str(json).expect("Failed to parse JSON");

        assert_eq!(repo.name, "enerby.dev");
        assert_eq!(repo.full_name, "enerBydev/enerby.dev");
        assert_eq!(
            repo.description,
            Some("Personal portfolio website".to_string())
        );
        assert_eq!(repo.homepage, Some("https://enerby.dev".to_string()));
        assert_eq!(repo.language, Some("Rust".to_string()));
        assert_eq!(repo.stargazers_count, 42);
        assert_eq!(repo.forks_count, 5);
        assert!(!repo.fork);
        assert!(!repo.archived);
        assert_eq!(repo.topics, vec!["rust", "dioxus", "portfolio"]);
        assert_eq!(repo.html_url, "https://github.com/enerBydev/enerby.dev");
    }

    /// Test: JSON deserialization with null/missing optional fields
    #[test]
    fn test_deserialize_github_repo_info_minimal() {
        let json = r#"{
            "name": "test-repo",
            "full_name": "owner/test-repo",
            "html_url": "https://github.com/owner/test-repo"
        }"#;

        let repo: GitHubRepoInfo = serde_json::from_str(json).expect("Failed to parse JSON");

        assert_eq!(repo.name, "test-repo");
        assert_eq!(repo.description, None);
        assert_eq!(repo.homepage, None);
        assert_eq!(repo.language, None);
        assert_eq!(repo.stargazers_count, 0);
        assert!(repo.topics.is_empty());
    }

    /// Test: JSON deserialization with homepage = null
    #[test]
    fn test_deserialize_homepage_null() {
        let json = r#"{
            "name": "test",
            "full_name": "owner/test",
            "homepage": null,
            "html_url": "https://github.com/owner/test"
        }"#;

        let repo: GitHubRepoInfo = serde_json::from_str(json).expect("Failed to parse JSON");
        assert_eq!(repo.homepage, None);
    }

    /// Test: extract_homepage with valid URL
    #[test]
    fn test_extract_homepage_valid() {
        let repo = GitHubRepoInfo {
            name: "test".to_string(),
            full_name: "owner/test".to_string(),
            description: None,
            homepage: Some("https://example.com".to_string()),
            language: None,
            stargazers_count: 0,
            forks_count: 0,
            fork: false,
            archived: false,
            topics: vec![],
            html_url: "https://github.com/owner/test".to_string(),
        };

        assert_eq!(
            repo.extract_homepage(),
            Some("https://example.com".to_string())
        );
    }

    /// Test: extract_homepage with empty string
    #[test]
    fn test_extract_homepage_empty() {
        let repo = GitHubRepoInfo {
            name: "test".to_string(),
            full_name: "owner/test".to_string(),
            description: None,
            homepage: Some("".to_string()),
            language: None,
            stargazers_count: 0,
            forks_count: 0,
            fork: false,
            archived: false,
            topics: vec![],
            html_url: "https://github.com/owner/test".to_string(),
        };

        assert_eq!(repo.extract_homepage(), None);
    }

    /// Test: extract_homepage with None
    #[test]
    fn test_extract_homepage_none() {
        let repo = GitHubRepoInfo {
            name: "test".to_string(),
            full_name: "owner/test".to_string(),
            description: None,
            homepage: None,
            language: None,
            stargazers_count: 0,
            forks_count: 0,
            fork: false,
            archived: false,
            topics: vec![],
            html_url: "https://github.com/owner/test".to_string(),
        };

        assert_eq!(repo.extract_homepage(), None);
    }

    /// Test: extract_homepage with invalid URL (no http prefix)
    #[test]
    fn test_extract_homepage_invalid_url() {
        let repo = GitHubRepoInfo {
            name: "test".to_string(),
            full_name: "owner/test".to_string(),
            description: None,
            homepage: Some("not-a-url".to_string()),
            language: None,
            stargazers_count: 0,
            forks_count: 0,
            fork: false,
            archived: false,
            topics: vec![],
            html_url: "https://github.com/owner/test".to_string(),
        };

        assert_eq!(repo.extract_homepage(), None);
    }

    /// Test: extract_homepage with whitespace
    #[test]
    fn test_extract_homepage_whitespace() {
        let repo = GitHubRepoInfo {
            name: "test".to_string(),
            full_name: "owner/test".to_string(),
            description: None,
            homepage: Some("  https://example.com  ".to_string()),
            language: None,
            stargazers_count: 0,
            forks_count: 0,
            fork: false,
            archived: false,
            topics: vec![],
            html_url: "https://github.com/owner/test".to_string(),
        };

        assert_eq!(
            repo.extract_homepage(),
            Some("https://example.com".to_string())
        );
    }

    /// Test: GitHubApiConfig default values
    #[test]
    fn test_api_config_default() {
        let config = GitHubApiConfig::default();

        assert_eq!(config.base_url, GITHUB_API_BASE_URL);
        assert_eq!(config.user_agent, USER_AGENT);
        assert_eq!(config.timeout_ms, API_TIMEOUT_MS);
    }

    // -------------------------
    // ApiError Tests
    // -------------------------

    #[test]
    fn test_api_error_display() {
        assert_eq!(format!("{}", ApiError::NotFound), "Repository not found");
        assert_eq!(
            format!("{}", ApiError::RateLimited),
            "GitHub API rate limit exceeded"
        );
        assert_eq!(
            format!("{}", ApiError::NetworkError("timeout".to_string())),
            "Network error: timeout"
        );
        assert_eq!(
            format!("{}", ApiError::ParseError("invalid json".to_string())),
            "Failed to parse response: invalid json"
        );
        assert_eq!(
            format!("{}", ApiError::ServerError(500)),
            "Server error: 500"
        );
    }

    // -------------------------
    // Static Data Tests
    // -------------------------

    #[test]
    fn test_get_repo_info_found() {
        let result = get_repo_info("enerBydev", "enerby.dev");
        assert!(result.is_ok());
        let repo = result.unwrap();
        assert_eq!(repo.name, "enerby.dev");
        assert_eq!(repo.homepage, Some("https://enerby.dev".to_string()));
    }

    #[test]
    fn test_get_repo_info_case_insensitive() {
        let result = get_repo_info("ENERBYDEV", "ENERBY.DEV");
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_repo_info_not_found() {
        let result = get_repo_info("nonexistent", "repo");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ApiError::NotFound);
    }

    #[test]
    fn test_get_repo_homepage_found() {
        let result = get_repo_homepage("enerBydev", "enerby.dev");
        assert_eq!(result, Some("https://enerby.dev".to_string()));
    }

    #[test]
    fn test_get_repo_homepage_no_homepage() {
        let result = get_repo_homepage("enerBydev", "nvim-config");
        assert_eq!(result, None);
    }

    #[test]
    fn test_get_repo_homepage_not_found() {
        let result = get_repo_homepage("nonexistent", "repo");
        assert_eq!(result, None);
    }

    #[test]
    fn test_get_all_repos() {
        let repos = get_all_repos();
        assert!(repos.len() >= 4);
        assert!(repos.iter().any(|r| r.name == "enerby.dev"));
    }

    // -------------------------
    // URL Parsing Tests
    // -------------------------

    #[test]
    fn test_parse_github_url_https() {
        let result = parse_github_url("https://github.com/owner/repo");
        assert_eq!(result, Some(("owner".to_string(), "repo".to_string())));
    }

    #[test]
    fn test_parse_github_url_http() {
        let result = parse_github_url("http://github.com/owner/repo");
        assert_eq!(result, Some(("owner".to_string(), "repo".to_string())));
    }

    #[test]
    fn test_parse_github_url_no_protocol() {
        let result = parse_github_url("github.com/owner/repo");
        assert_eq!(result, Some(("owner".to_string(), "repo".to_string())));
    }

    #[test]
    fn test_parse_github_url_with_trailing_slash() {
        let result = parse_github_url("https://github.com/owner/repo/");
        assert_eq!(result, Some(("owner".to_string(), "repo".to_string())));
    }

    #[test]
    fn test_parse_github_url_invalid() {
        assert_eq!(parse_github_url("https://gitlab.com/owner/repo"), None);
        assert_eq!(parse_github_url("not-a-url"), None);
        assert_eq!(parse_github_url("https://github.com/"), None);
        assert_eq!(parse_github_url("https://github.com/owner"), None);
    }

    #[test]
    fn test_get_homepage_from_url() {
        let result = get_homepage_from_url("https://github.com/enerBydev/enerby.dev");
        assert_eq!(result, Some("https://enerby.dev".to_string()));
    }

    #[test]
    fn test_get_homepage_from_url_no_homepage() {
        let result = get_homepage_from_url("https://github.com/enerBydev/nvim-config");
        assert_eq!(result, None);
    }

    // -------------------------
    // Batch Fetch Tests
    // -------------------------

    #[test]
    fn test_fetch_multiple_repos() {
        let repos = &[
            ("enerBydev", "enerby.dev"),
            ("enerBydev", "oc_diagdoc"),
            ("nonexistent", "repo"),
        ];

        let results = fetch_multiple_repos(repos);

        assert_eq!(results.len(), 3);
        assert!(results[0].is_ok());
        assert!(results[1].is_ok());
        assert!(results[2].is_err());
    }

    // -------------------------
    // Minimal Constructor Test
    // -------------------------

    #[test]
    fn test_minimal_constructor() {
        let repo = GitHubRepoInfo::minimal("owner", "repo");
        assert_eq!(repo.name, "repo");
        assert_eq!(repo.full_name, "owner/repo");
        assert_eq!(repo.html_url, "https://github.com/owner/repo");
        assert!(repo.homepage.is_none());
    }
}
