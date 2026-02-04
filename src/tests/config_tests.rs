//! Config Tests (P16-A4)
//! Tests for site configuration and data models

use crate::config::{NAV_LINKS, SITE, SOCIAL_LINKS};

#[test]
fn test_site_config_has_required_fields() {
    // Verify site config is properly configured
    assert!(!SITE.name.is_empty(), "Site name should not be empty");
    assert!(!SITE.title.is_empty(), "Site title should not be empty");
    assert!(
        !SITE.description.is_empty(),
        "Site description should not be empty"
    );
    assert!(!SITE.author.is_empty(), "Site author should not be empty");
    assert!(
        !SITE.base_url.is_empty(),
        "Site base_url should not be empty"
    );
}

#[test]
fn test_site_base_url_is_valid() {
    // Check base URL format
    assert!(
        SITE.base_url.starts_with("https://"),
        "Base URL should use HTTPS"
    );
    assert!(
        !SITE.base_url.ends_with("/"),
        "Base URL should not end with slash"
    );
}

#[test]
fn test_nav_links_exist() {
    // Verify navigation links are configured
    assert!(!NAV_LINKS.is_empty(), "Navigation links should exist");
    assert!(NAV_LINKS.len() >= 3, "Should have at least 3 nav links");
}

#[test]
fn test_nav_links_have_valid_hrefs() {
    for link in NAV_LINKS {
        assert!(!link.label.is_empty(), "Nav link label should not be empty");
        assert!(!link.href.is_empty(), "Nav link href should not be empty");

        if !link.is_external {
            assert!(
                link.href.starts_with("/"),
                "Internal links should start with /"
            );
        }
    }
}

#[test]
fn test_social_links_exist() {
    // Verify social links are configured
    assert!(!SOCIAL_LINKS.is_empty(), "Social links should exist");
}

#[test]
fn test_social_links_have_valid_urls() {
    for link in SOCIAL_LINKS {
        assert!(
            !link.name.is_empty(),
            "Social link name should not be empty"
        );
        assert!(!link.url.is_empty(), "Social link URL should not be empty");
        assert!(
            link.url.starts_with("https://"),
            "Social links should use HTTPS: {}",
            link.name
        );
    }
}
