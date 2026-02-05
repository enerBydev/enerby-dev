//! GitHub Cache Module
//!
//! This module provides caching functionality for GitHub repository data.
//! It supports both in-memory caching and JSON file persistence.
//!
//! # Architecture
//!
//! The cache uses a simple key-value store where:
//! - Key: "owner/repo" (lowercase)
//! - Value: `CacheEntry<GitHubRepoInfo>` with timestamp
//!
//! # TTL (Time To Live)
//!
//! Default TTL is 1 hour. After expiration, the cache entry is considered stale
//! but can still be used as a fallback if the API is unavailable.
//!
//! # Example
//! ```rust,ignore
//! use crate::utils::github_cache::GitHubCache;
//!
//! let mut cache = GitHubCache::new();
//! cache.set("enerBydev", "enerby.dev", repo_info);
//! let cached = cache.get("enerBydev", "enerby.dev");
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use super::github_api::GitHubRepoInfo;

// ============================================================================
// CONSTANTS
// ============================================================================

/// Default cache TTL in seconds (1 hour)
pub const DEFAULT_TTL_SECONDS: u64 = 3600;

/// Cache file path (relative to project root)
pub const CACHE_FILE_PATH: &str = ".cache/github_repos.json";

// ============================================================================
// DATA STRUCTURES
// ============================================================================

/// A cached entry with timestamp for TTL calculation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CacheEntry<T> {
    /// The cached data
    pub data: T,
    /// Unix timestamp when the entry was cached
    pub cached_at: u64,
    /// TTL in seconds (for this specific entry)
    pub ttl_seconds: u64,
}

impl<T> CacheEntry<T> {
    /// Creates a new cache entry with current timestamp
    pub fn new(data: T, ttl_seconds: u64) -> Self {
        let cached_at = get_now_seconds();

        Self {
            data,
            cached_at,
            ttl_seconds,
        }
    }

    /// Creates a new cache entry with default TTL
    pub fn with_default_ttl(data: T) -> Self {
        Self::new(data, DEFAULT_TTL_SECONDS)
    }

    /// Checks if this cache entry has expired
    pub fn is_expired(&self) -> bool {
        let now = get_now_seconds();

        now > self.cached_at + self.ttl_seconds
    }

    /// Returns the age of this cache entry in seconds
    pub fn age_seconds(&self) -> u64 {
        let now = get_now_seconds();

        now.saturating_sub(self.cached_at)
    }

    /// Returns remaining TTL in seconds (0 if expired)
    pub fn remaining_ttl(&self) -> u64 {
        let expiry = self.cached_at + self.ttl_seconds;
        let now = get_now_seconds();

        expiry.saturating_sub(now)
    }
}

/// In-memory cache for GitHub repository data
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GitHubCache {
    /// Internal storage: key = "owner/repo" (lowercase)
    entries: HashMap<String, CacheEntry<GitHubRepoInfo>>,
    /// Default TTL for new entries
    #[serde(default = "default_ttl")]
    default_ttl: u64,
}

fn default_ttl() -> u64 {
    DEFAULT_TTL_SECONDS
}

impl GitHubCache {
    /// Creates a new empty cache with default TTL
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            default_ttl: DEFAULT_TTL_SECONDS,
        }
    }

    /// Creates a new cache with custom TTL
    pub fn with_ttl(ttl_seconds: u64) -> Self {
        Self {
            entries: HashMap::new(),
            default_ttl: ttl_seconds,
        }
    }

    /// Generates a cache key from owner and repo
    fn make_key(owner: &str, repo: &str) -> String {
        format!("{}/{}", owner.to_lowercase(), repo.to_lowercase())
    }

    /// Gets a cached entry if it exists and is not expired
    ///
    /// Returns `None` if:
    /// - Entry doesn't exist
    /// - Entry has expired
    pub fn get(&self, owner: &str, repo: &str) -> Option<&GitHubRepoInfo> {
        let key = Self::make_key(owner, repo);
        self.entries.get(&key).and_then(|entry| {
            if entry.is_expired() {
                None
            } else {
                Some(&entry.data)
            }
        })
    }

    /// Gets a cached entry even if expired (for fallback use)
    ///
    /// Returns `Some((data, is_stale))` if entry exists
    pub fn get_with_stale(&self, owner: &str, repo: &str) -> Option<(&GitHubRepoInfo, bool)> {
        let key = Self::make_key(owner, repo);
        self.entries
            .get(&key)
            .map(|entry| (&entry.data, entry.is_expired()))
    }

    /// Sets a cache entry with default TTL
    pub fn set(&mut self, owner: &str, repo: &str, data: GitHubRepoInfo) {
        let key = Self::make_key(owner, repo);
        let entry = CacheEntry::new(data, self.default_ttl);
        self.entries.insert(key, entry);
    }

    /// Sets a cache entry with custom TTL
    pub fn set_with_ttl(
        &mut self,
        owner: &str,
        repo: &str,
        data: GitHubRepoInfo,
        ttl_seconds: u64,
    ) {
        let key = Self::make_key(owner, repo);
        let entry = CacheEntry::new(data, ttl_seconds);
        self.entries.insert(key, entry);
    }

    /// Removes an entry from the cache
    pub fn remove(&mut self, owner: &str, repo: &str) -> Option<GitHubRepoInfo> {
        let key = Self::make_key(owner, repo);
        self.entries.remove(&key).map(|e| e.data)
    }

    /// Clears all entries from the cache
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Returns the number of entries in the cache (including expired)
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns true if the cache is empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns the number of non-expired entries
    pub fn len_valid(&self) -> usize {
        self.entries.values().filter(|e| !e.is_expired()).count()
    }

    /// Removes all expired entries from the cache
    pub fn cleanup_expired(&mut self) {
        self.entries.retain(|_, entry| !entry.is_expired());
    }

    /// Checks if a fresh (non-expired) entry exists
    pub fn has_fresh(&self, owner: &str, repo: &str) -> bool {
        self.get(owner, repo).is_some()
    }

    /// Checks if any entry (including stale) exists
    pub fn has_any(&self, owner: &str, repo: &str) -> bool {
        let key = Self::make_key(owner, repo);
        self.entries.contains_key(&key)
    }

    // =========================================================================
    // Persistence Methods
    // =========================================================================

    /// Serializes the cache to JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Deserializes cache from JSON string
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Loads cache from a JSON string, returning empty cache on error
    pub fn load_or_empty(json: &str) -> Self {
        Self::from_json(json).unwrap_or_default()
    }

    /// Merges another cache into this one
    /// Newer entries (by timestamp) take precedence
    pub fn merge(&mut self, other: Self) {
        for (key, entry) in other.entries {
            match self.entries.get(&key) {
                Some(existing) if existing.cached_at >= entry.cached_at => {
                    // Keep existing (newer or same age)
                }
                _ => {
                    // Insert new entry
                    self.entries.insert(key, entry);
                }
            }
        }
    }

    /// Pre-populates cache from static data (used for initialization)
    pub fn populate_from_static(&mut self) {
        use super::github_api::get_all_repos;

        for repo in get_all_repos() {
            // Clone full_name before split to avoid borrow issues
            let full_name = repo.full_name.clone();
            if let Some((owner, name)) = full_name.split_once('/') {
                self.set(owner, name, repo);
            }
        }
    }
}

// ============================================================================
// GLOBAL CACHE INSTANCE (Optional - for simple use cases)
// ============================================================================

use std::sync::OnceLock;

/// Global cache instance (lazy initialization)
static GLOBAL_CACHE: OnceLock<std::sync::RwLock<GitHubCache>> = OnceLock::new();

/// Gets a reference to the global cache
pub fn global_cache() -> &'static std::sync::RwLock<GitHubCache> {
    GLOBAL_CACHE.get_or_init(|| {
        let mut cache = GitHubCache::new();
        cache.populate_from_static();
        std::sync::RwLock::new(cache)
    })
}

/// Gets a cached repo from global cache
pub fn get_cached_repo(owner: &str, repo: &str) -> Option<GitHubRepoInfo> {
    global_cache()
        .read()
        .ok()
        .and_then(|cache| cache.get(owner, repo).cloned())
}

/// Sets a repo in global cache
pub fn set_cached_repo(owner: &str, repo: &str, data: GitHubRepoInfo) {
    if let Ok(mut cache) = global_cache().write() {
        cache.set(owner, repo, data);
    }
}

// Helper for getting current time in seconds
// In WASM, SystemTime is not supported without bindings, so we return a dummy value (non-zero)
// This effectively disables TTL/expiration logic in WASM usage.
#[cfg(not(target_arch = "wasm32"))]
fn get_now_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(target_arch = "wasm32")]
fn get_now_seconds() -> u64 {
    // Return a constant starting point.
    // Since cached_at will also use this, expired checks (now > cached_at + ttl)
    // will be: 1000 > 1000 + 3600 -> false (not expired).
    1000
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_repo(name: &str) -> GitHubRepoInfo {
        GitHubRepoInfo {
            name: name.to_string(),
            full_name: format!("owner/{}", name),
            description: Some("Test repo".to_string()),
            homepage: Some("https://example.com".to_string()),
            language: Some("Rust".to_string()),
            stargazers_count: 10,
            forks_count: 2,
            fork: false,
            archived: false,
            topics: vec!["test".to_string()],
            html_url: format!("https://github.com/owner/{}", name),
        }
    }

    // -------------------------
    // CacheEntry Tests
    // -------------------------

    #[test]
    fn test_cache_entry_new() {
        let repo = create_test_repo("test");
        let entry = CacheEntry::new(repo.clone(), 3600);

        assert_eq!(entry.data.name, "test");
        assert_eq!(entry.ttl_seconds, 3600);
        assert!(entry.cached_at > 0);
    }

    #[test]
    fn test_cache_entry_with_default_ttl() {
        let repo = create_test_repo("test");
        let entry = CacheEntry::with_default_ttl(repo);

        assert_eq!(entry.ttl_seconds, DEFAULT_TTL_SECONDS);
    }

    #[test]
    fn test_cache_entry_not_expired() {
        let repo = create_test_repo("test");
        let entry = CacheEntry::new(repo, 3600); // 1 hour TTL

        assert!(!entry.is_expired());
    }

    #[test]
    fn test_cache_entry_expired() {
        let repo = create_test_repo("test");
        let mut entry = CacheEntry::new(repo, 1); // 1 second TTL

        // Force expiration by setting cached_at to the past
        entry.cached_at = entry.cached_at.saturating_sub(10);

        assert!(entry.is_expired());
    }

    #[test]
    fn test_cache_entry_age() {
        let repo = create_test_repo("test");
        let entry = CacheEntry::new(repo, 3600);

        // Age should be very small (just created)
        assert!(entry.age_seconds() < 2);
    }

    #[test]
    fn test_cache_entry_remaining_ttl() {
        let repo = create_test_repo("test");
        let entry = CacheEntry::new(repo, 3600);

        // Remaining TTL should be close to 3600
        let remaining = entry.remaining_ttl();
        assert!(remaining > 3590 && remaining <= 3600);
    }

    // -------------------------
    // GitHubCache Tests
    // -------------------------

    #[test]
    fn test_cache_new() {
        let cache = GitHubCache::new();
        assert!(cache.is_empty());
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn test_cache_with_custom_ttl() {
        let cache = GitHubCache::with_ttl(7200);
        assert_eq!(cache.default_ttl, 7200);
    }

    #[test]
    fn test_cache_set_get() {
        let mut cache = GitHubCache::new();
        let repo = create_test_repo("myrepo");

        cache.set("owner", "myrepo", repo.clone());

        let cached = cache.get("owner", "myrepo");
        assert!(cached.is_some());
        assert_eq!(cached.unwrap().name, "myrepo");
    }

    #[test]
    fn test_cache_case_insensitive() {
        let mut cache = GitHubCache::new();
        let repo = create_test_repo("MyRepo");

        cache.set("Owner", "MyRepo", repo);

        // Should find with different case
        assert!(cache.get("owner", "myrepo").is_some());
        assert!(cache.get("OWNER", "MYREPO").is_some());
    }

    #[test]
    fn test_cache_get_nonexistent() {
        let cache = GitHubCache::new();
        assert!(cache.get("owner", "nonexistent").is_none());
    }

    #[test]
    fn test_cache_get_with_stale() {
        let mut cache = GitHubCache::new();
        let repo = create_test_repo("test");

        cache.set("owner", "test", repo);

        let result = cache.get_with_stale("owner", "test");
        assert!(result.is_some());

        let (data, is_stale) = result.unwrap();
        assert_eq!(data.name, "test");
        assert!(!is_stale); // Just created, should not be stale
    }

    #[test]
    fn test_cache_remove() {
        let mut cache = GitHubCache::new();
        let repo = create_test_repo("test");

        cache.set("owner", "test", repo);
        assert!(cache.get("owner", "test").is_some());

        let removed = cache.remove("owner", "test");
        assert!(removed.is_some());
        assert!(cache.get("owner", "test").is_none());
    }

    #[test]
    fn test_cache_clear() {
        let mut cache = GitHubCache::new();
        cache.set("owner", "repo1", create_test_repo("repo1"));
        cache.set("owner", "repo2", create_test_repo("repo2"));

        assert_eq!(cache.len(), 2);

        cache.clear();
        assert!(cache.is_empty());
    }

    #[test]
    fn test_cache_len_valid() {
        let mut cache = GitHubCache::new();
        cache.set("owner", "repo1", create_test_repo("repo1"));
        cache.set_with_ttl("owner", "repo2", create_test_repo("repo2"), 1);

        // Force repo2 to expire
        if let Some(entry) = cache.entries.get_mut("owner/repo2") {
            entry.cached_at = entry.cached_at.saturating_sub(10);
        }

        assert_eq!(cache.len(), 2);
        assert_eq!(cache.len_valid(), 1);
    }

    #[test]
    fn test_cache_cleanup_expired() {
        let mut cache = GitHubCache::new();
        cache.set("owner", "fresh", create_test_repo("fresh"));
        cache.set_with_ttl("owner", "stale", create_test_repo("stale"), 1);

        // Force stale to expire
        if let Some(entry) = cache.entries.get_mut("owner/stale") {
            entry.cached_at = entry.cached_at.saturating_sub(10);
        }

        assert_eq!(cache.len(), 2);
        cache.cleanup_expired();
        assert_eq!(cache.len(), 1);
        assert!(cache.get("owner", "fresh").is_some());
    }

    #[test]
    fn test_cache_has_fresh_and_has_any() {
        let mut cache = GitHubCache::new();
        cache.set_with_ttl("owner", "test", create_test_repo("test"), 1);

        // Fresh entry
        assert!(cache.has_fresh("owner", "test"));
        assert!(cache.has_any("owner", "test"));

        // Force expiration
        if let Some(entry) = cache.entries.get_mut("owner/test") {
            entry.cached_at = entry.cached_at.saturating_sub(10);
        }

        // Stale entry
        assert!(!cache.has_fresh("owner", "test"));
        assert!(cache.has_any("owner", "test"));
    }

    // -------------------------
    // Persistence Tests
    // -------------------------

    #[test]
    fn test_cache_to_json() {
        let mut cache = GitHubCache::new();
        cache.set("owner", "test", create_test_repo("test"));

        let json = cache.to_json();
        assert!(json.is_ok());

        let json_str = json.unwrap();
        assert!(json_str.contains("owner/test"));
        assert!(json_str.contains("\"name\": \"test\""));
    }

    #[test]
    fn test_cache_from_json() {
        let mut original = GitHubCache::new();
        original.set("owner", "test", create_test_repo("test"));

        let json = original.to_json().unwrap();
        let restored = GitHubCache::from_json(&json);

        assert!(restored.is_ok());
        let restored = restored.unwrap();
        assert!(restored.get("owner", "test").is_some());
    }

    #[test]
    fn test_cache_load_or_empty_valid() {
        let mut original = GitHubCache::new();
        original.set("owner", "test", create_test_repo("test"));

        let json = original.to_json().unwrap();
        let loaded = GitHubCache::load_or_empty(&json);

        assert!(loaded.get("owner", "test").is_some());
    }

    #[test]
    fn test_cache_load_or_empty_invalid() {
        let loaded = GitHubCache::load_or_empty("invalid json");
        assert!(loaded.is_empty());
    }

    #[test]
    fn test_cache_merge() {
        let mut cache1 = GitHubCache::new();
        cache1.set("owner", "repo1", create_test_repo("repo1"));

        let mut cache2 = GitHubCache::new();
        cache2.set("owner", "repo2", create_test_repo("repo2"));

        cache1.merge(cache2);

        assert!(cache1.get("owner", "repo1").is_some());
        assert!(cache1.get("owner", "repo2").is_some());
    }

    #[test]
    fn test_cache_populate_from_static() {
        let mut cache = GitHubCache::new();
        cache.populate_from_static();

        // Should have the static repos from github_api
        assert!(cache.len() >= 4);
        assert!(cache.get("enerBydev", "enerby.dev").is_some());
    }

    // -------------------------
    // Global Cache Tests
    // -------------------------

    #[test]
    fn test_global_cache_get() {
        // Global cache should be populated with static data
        let repo = get_cached_repo("enerBydev", "enerby.dev");
        assert!(repo.is_some());
        assert_eq!(
            repo.unwrap().homepage,
            Some("https://enerby.dev".to_string())
        );
    }
}
