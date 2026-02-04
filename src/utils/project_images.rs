//! Project Images Module
//! Generates image URLs for project cards using GitHub OpenGraph and ThumbIO APIs
//!
//! ## Strategy
//! 1. image_override (manual) → Highest priority
//! 2. demo_url → ThumbIO screenshot
//! 3. github_url → GitHub OpenGraph
//! 4. image_fallback → Emoji placeholder

use crate::components::projects::Project;

// =============================================================================
// CONSTANTS
// =============================================================================

/// ThumbIO API base URL (free, no API key required)
const THUMBIO_BASE: &str = "https://image.thum.io/get";

/// GitHub OpenGraph base URL
const GITHUB_OG_BASE: &str = "https://opengraph.githubassets.com";

/// Default viewport dimensions for screenshots
const SCREENSHOT_WIDTH: u32 = 1200;
const SCREENSHOT_HEIGHT: u32 = 630;

// =============================================================================
// TYPES
// =============================================================================

/// Enum representing the source of the project image
#[derive(Debug, Clone, PartialEq)]
pub enum ImageSource {
    /// Manual URL override (highest priority)
    Override(String),
    /// ThumbIO screenshot of demo URL
    DemoScreenshot(String),
    /// GitHub OpenGraph preview image
    GitHubOpenGraph(String),
    /// Emoji fallback (lowest priority)
    Fallback(String),
}

impl std::fmt::Display for ImageSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Override(url) => write!(f, "{}", url),
            Self::DemoScreenshot(url) => write!(f, "{}", url),
            Self::GitHubOpenGraph(url) => write!(f, "{}", url),
            Self::Fallback(emoji) => write!(f, "{}", emoji),
        }
    }
}

impl ImageSource {
    /// Returns true if this is a real image URL (not a fallback emoji)
    pub fn is_real_image(&self) -> bool {
        !matches!(self, Self::Fallback(_))
    }

    /// Returns the URL string if this is a real image, None otherwise
    pub fn url(&self) -> Option<&str> {
        match self {
            Self::Override(url) | Self::DemoScreenshot(url) | Self::GitHubOpenGraph(url) => {
                Some(url)
            }
            Self::Fallback(_) => None,
        }
    }
}

// =============================================================================
// URL GENERATORS
// =============================================================================

/// Parses a GitHub URL to extract owner and repository name
///
/// # Arguments
/// * `url` - GitHub repository URL (e.g., "https://github.com/owner/repo")
///
/// # Returns
/// * `Some((owner, repo))` if parsing succeeds
/// * `None` if URL format is invalid
///
/// # Examples
/// ```
/// let result = parse_github_url("https://github.com/enerBydev/enerby.dev");
/// assert_eq!(result, Some(("enerBydev".to_string(), "enerby.dev".to_string())));
/// ```
pub fn parse_github_url(url: &str) -> Option<(String, String)> {
    // Normalize URL: remove trailing slash and .git suffix
    let normalized = url.trim_end_matches('/').trim_end_matches(".git");

    // Split by '/' and get last two segments
    let parts: Vec<&str> = normalized.split('/').collect();

    // Need at least: https:, "", github.com, owner, repo
    if parts.len() >= 5 {
        let repo = parts[parts.len() - 1].to_string();
        let owner = parts[parts.len() - 2].to_string();

        // Validate non-empty
        if !owner.is_empty() && !repo.is_empty() {
            return Some((owner, repo));
        }
    }

    None
}

/// Generates a GitHub OpenGraph image URL for a repository
///
/// Uses the pattern: https://opengraph.githubassets.com/{hash}/{owner}/{repo}
///
/// # Arguments
/// * `owner` - GitHub username or organization
/// * `repo` - Repository name
///
/// # Returns
/// * Full URL to the OpenGraph image
pub fn generate_github_opengraph_url(owner: &str, repo: &str) -> String {
    // Use "1" as hash for consistent caching
    format!("{}/1/{}/{}", GITHUB_OG_BASE, owner, repo)
}

/// Gets GitHub OpenGraph URL from a github_url if valid
pub fn get_github_preview_from_url(github_url: &str) -> Option<String> {
    parse_github_url(github_url).map(|(owner, repo)| generate_github_opengraph_url(&owner, &repo))
}

/// Generates a ThumbIO screenshot URL for any website
///
/// Uses ThumbIO's free API (no API key required)
/// Pattern: https://image.thum.io/get/width/{width}/crop/{height}/{url}
///
/// # Arguments
/// * `demo_url` - URL of the website to screenshot
///
/// # Returns
/// * Full ThumbIO URL that will return a screenshot
pub fn generate_thumbio_url(demo_url: &str) -> String {
    format!(
        "{}/width/{}/crop/{}/{}",
        THUMBIO_BASE, SCREENSHOT_WIDTH, SCREENSHOT_HEIGHT, demo_url
    )
}

/// Generates ThumbIO URL with custom dimensions
#[allow(dead_code)]
pub fn generate_thumbio_url_custom(demo_url: &str, width: u32, height: u32) -> String {
    format!("{}/width/{}/crop/{}/{}", THUMBIO_BASE, width, height, demo_url)
}

// =============================================================================
// MAIN API
// =============================================================================

/// Returns the best available image source for a project
///
/// # Priority Order:
/// 1. `image_override` - Manual override (highest priority)
/// 2. `demo_url` - ThumbIO screenshot of live demo
/// 3. `github_url` - GitHub OpenGraph preview
/// 4. `image_fallback` - Emoji placeholder (lowest priority)
///
/// # Arguments
/// * `project` - Reference to the Project struct
///
/// # Returns
/// * `ImageSource` enum containing the URL or fallback
pub fn get_project_image_url(project: &Project) -> ImageSource {
    // Priority 1: Manual override
    if let Some(override_url) = &project.image_override {
        return ImageSource::Override(override_url.to_string());
    }

    // Priority 2: Demo URL → ThumbIO screenshot
    if let Some(demo) = &project.demo_url {
        return ImageSource::DemoScreenshot(generate_thumbio_url(demo));
    }

    // Priority 3: GitHub URL → OpenGraph
    if let Some(github) = &project.github_url {
        if let Some(og_url) = get_github_preview_from_url(github) {
            return ImageSource::GitHubOpenGraph(og_url);
        }
    }

    // Priority 4: Fallback emoji
    ImageSource::Fallback(project.image_fallback.to_string())
}

/// Returns image URL as String (for use in RSX)
pub fn get_project_image_url_string(project: &Project) -> String {
    get_project_image_url(project).to_string()
}

/// Returns true if the project has a real image URL (not fallback emoji)
pub fn has_real_image(project: &Project) -> bool {
    get_project_image_url(project).is_real_image()
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::projects::ProjectStatus;

    /// Helper to create mock projects for testing
    fn create_mock_project(
        github_url: Option<&'static str>,
        demo_url: Option<&'static str>,
        image_override: Option<&'static str>,
    ) -> Project {
        Project {
            id: "test-project",
            title: "Test Project",
            description: "Test description",
            long_description: "Test long description",
            technologies: vec!["Rust"],
            status: ProjectStatus::Active,
            github_url,
            demo_url,
            image_override,
            image_fallback: "🧪",
        }
    }

    // =========================================================================
    // GitHub URL Parser Tests
    // =========================================================================

    #[test]
    fn test_parse_github_url_basic() {
        let result = parse_github_url("https://github.com/enerBydev/enerby.dev");
        assert_eq!(
            result,
            Some(("enerBydev".to_string(), "enerby.dev".to_string()))
        );
    }

    #[test]
    fn test_parse_github_url_with_trailing_slash() {
        let result = parse_github_url("https://github.com/owner/repo/");
        assert_eq!(result, Some(("owner".to_string(), "repo".to_string())));
    }

    #[test]
    fn test_parse_github_url_with_git_extension() {
        let result = parse_github_url("https://github.com/owner/repo.git");
        assert_eq!(result, Some(("owner".to_string(), "repo".to_string())));
    }

    #[test]
    fn test_parse_github_url_invalid() {
        assert!(parse_github_url("not-a-url").is_none());
        assert!(parse_github_url("https://example.com").is_none());
        assert!(parse_github_url("").is_none());
    }

    // =========================================================================
    // OpenGraph URL Generator Tests
    // =========================================================================

    #[test]
    fn test_generate_github_opengraph_url() {
        let url = generate_github_opengraph_url("enerBydev", "enerby.dev");
        assert_eq!(
            url,
            "https://opengraph.githubassets.com/1/enerBydev/enerby.dev"
        );
    }

    #[test]
    fn test_get_github_preview_from_url() {
        let result = get_github_preview_from_url("https://github.com/owner/repo");
        assert!(result.is_some());
        assert!(result.unwrap().contains("owner/repo"));
    }

    // =========================================================================
    // ThumbIO URL Generator Tests
    // =========================================================================

    #[test]
    fn test_generate_thumbio_url() {
        let url = generate_thumbio_url("https://enerby.dev");
        assert!(url.starts_with("https://image.thum.io/get/"));
        assert!(url.contains("width/1200"));
        assert!(url.contains("crop/630"));
        assert!(url.ends_with("https://enerby.dev"));
    }

    #[test]
    fn test_generate_thumbio_url_custom() {
        let url = generate_thumbio_url_custom("https://example.com", 800, 600);
        assert!(url.contains("width/800"));
        assert!(url.contains("crop/600"));
    }

    // =========================================================================
    // Priority Tests
    // =========================================================================

    #[test]
    fn test_priority_override() {
        let project = create_mock_project(
            Some("https://github.com/owner/repo"),
            Some("https://demo.com"),
            Some("https://custom.com/image.png"),
        );
        let result = get_project_image_url(&project);
        assert!(matches!(result, ImageSource::Override(_)));
        assert_eq!(result.to_string(), "https://custom.com/image.png");
    }

    #[test]
    fn test_priority_demo_url() {
        let project = create_mock_project(
            Some("https://github.com/owner/repo"),
            Some("https://demo.com"),
            None,
        );
        let result = get_project_image_url(&project);
        assert!(matches!(result, ImageSource::DemoScreenshot(_)));
        assert!(result.to_string().contains("image.thum.io"));
    }

    #[test]
    fn test_priority_github_url() {
        let project = create_mock_project(Some("https://github.com/owner/repo"), None, None);
        let result = get_project_image_url(&project);
        assert!(matches!(result, ImageSource::GitHubOpenGraph(_)));
        assert!(result.to_string().contains("opengraph.githubassets.com"));
    }

    #[test]
    fn test_priority_fallback() {
        let project = create_mock_project(None, None, None);
        let result = get_project_image_url(&project);
        assert!(matches!(result, ImageSource::Fallback(_)));
        assert_eq!(result.to_string(), "🧪");
    }

    // =========================================================================
    // Helper Function Tests
    // =========================================================================

    #[test]
    fn test_has_real_image_true() {
        let project = create_mock_project(Some("https://github.com/owner/repo"), None, None);
        assert!(has_real_image(&project));
    }

    #[test]
    fn test_has_real_image_false() {
        let project = create_mock_project(None, None, None);
        assert!(!has_real_image(&project));
    }

    #[test]
    fn test_image_source_is_real_image() {
        assert!(ImageSource::Override("url".to_string()).is_real_image());
        assert!(ImageSource::DemoScreenshot("url".to_string()).is_real_image());
        assert!(ImageSource::GitHubOpenGraph("url".to_string()).is_real_image());
        assert!(!ImageSource::Fallback("🧪".to_string()).is_real_image());
    }
}
