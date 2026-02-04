//! Data Tests (P16-A3, P16-A4)
//! Tests for data models: Projects, Blog Posts, Skills

use crate::components::blog::{PostStatus, get_blog_posts, get_published_posts};
use crate::components::projects::{ProjectStatus, get_projects};
use crate::components::skills::SkillCategory;

// =============================================================================
// Project Tests (P16-C2)
// =============================================================================

#[test]
fn test_projects_exist() {
    let projects = get_projects();
    assert!(!projects.is_empty(), "Should have at least one project");
}

#[test]
fn test_projects_have_required_fields() {
    let projects = get_projects();

    for project in &projects {
        assert!(!project.id.is_empty(), "Project ID should not be empty");
        assert!(
            !project.title.is_empty(),
            "Project title should not be empty"
        );
        assert!(
            !project.description.is_empty(),
            "Project description should not be empty"
        );
        assert!(
            !project.technologies.is_empty(),
            "Project should have technologies"
        );
    }
}

#[test]
fn test_projects_have_unique_ids() {
    let projects = get_projects();
    let mut ids: Vec<&str> = projects.iter().map(|p| p.id).collect();
    let original_len = ids.len();
    ids.sort();
    ids.dedup();

    assert_eq!(ids.len(), original_len, "Project IDs should be unique");
}

#[test]
fn test_has_featured_project() {
    let projects = get_projects();
    let featured_count = projects
        .iter()
        .filter(|p| p.status == ProjectStatus::Featured)
        .count();

    assert!(
        featured_count >= 1,
        "Should have at least one featured project"
    );
}

// =============================================================================
// Blog Post Tests (P16-C3)
// =============================================================================

#[test]
fn test_blog_posts_exist() {
    let posts = get_blog_posts();
    assert!(!posts.is_empty(), "Should have at least one blog post");
}

#[test]
fn test_blog_posts_have_required_fields() {
    let posts = get_blog_posts();

    for post in &posts {
        assert!(!post.slug.is_empty(), "Post slug should not be empty");
        assert!(!post.title.is_empty(), "Post title should not be empty");
        assert!(!post.excerpt.is_empty(), "Post excerpt should not be empty");
        assert!(!post.date.is_empty(), "Post date should not be empty");
        assert!(post.read_time > 0, "Read time should be positive");
    }
}

#[test]
fn test_blog_posts_have_unique_slugs() {
    let posts = get_blog_posts();
    let mut slugs: Vec<String> = posts.iter().map(|p| p.slug.clone()).collect();
    let original_len = slugs.len();
    slugs.sort();
    slugs.dedup();

    assert_eq!(slugs.len(), original_len, "Post slugs should be unique");
}

#[test]
fn test_published_posts_filter() {
    let all_posts = get_blog_posts();
    let published = get_published_posts();

    // All published posts should have Published status
    for post in &published {
        assert_eq!(
            post.status,
            PostStatus::Published,
            "Filtered posts should be published"
        );
    }

    // Published count should be less than or equal to all posts
    assert!(published.len() <= all_posts.len(), "Published <= All posts");
}

#[test]
fn test_blog_posts_have_tags() {
    let posts = get_blog_posts();

    for post in &posts {
        assert!(
            !post.tags.is_empty(),
            "Post '{}' should have at least one tag",
            post.title
        );
    }
}

// =============================================================================
// Skills Category Tests
// =============================================================================

#[test]
fn test_skill_category_labels() {
    assert_eq!(SkillCategory::Languages.label(), "Languages");
    assert_eq!(SkillCategory::Frameworks.label(), "Frameworks & Libraries");
    assert_eq!(SkillCategory::Tools.label(), "Tools & Platforms");
    assert_eq!(SkillCategory::Concepts.label(), "Concepts & Practices");
}

#[test]
fn test_skill_category_colors() {
    // Verify each category has a color defined
    assert!(!SkillCategory::Languages.color().is_empty());
    assert!(!SkillCategory::Frameworks.color().is_empty());
    assert!(!SkillCategory::Tools.color().is_empty());
    assert!(!SkillCategory::Concepts.color().is_empty());
}
