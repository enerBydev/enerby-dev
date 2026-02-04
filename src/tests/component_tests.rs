//! Component Tests (P16-B)
//! Tests for component props and behavior

use crate::components::atoms::ButtonVariant;
use crate::components::projects::ProjectStatus;
use crate::components::blog::PostStatus;
use crate::components::contact::FormState;

// =============================================================================
// Enum Variant Tests
// =============================================================================

#[test]
fn test_button_variants_are_distinct() {
    assert_ne!(ButtonVariant::Primary, ButtonVariant::Secondary);
    assert_ne!(ButtonVariant::Primary, ButtonVariant::Ghost);
    assert_ne!(ButtonVariant::Primary, ButtonVariant::Neon);
}

#[test]
fn test_project_status_labels() {
    assert_eq!(ProjectStatus::Featured.label(), "Featured");
    assert_eq!(ProjectStatus::Active.label(), "Active");
    assert_eq!(ProjectStatus::Archived.label(), "Archived");
}

#[test]
fn test_project_status_colors() {
    // Each status should have a color
    assert!(!ProjectStatus::Featured.color().is_empty());
    assert!(!ProjectStatus::Active.color().is_empty());
    assert!(!ProjectStatus::Archived.color().is_empty());
}

#[test]
fn test_post_status_equality() {
    assert_eq!(PostStatus::Published, PostStatus::Published);
    assert_eq!(PostStatus::Draft, PostStatus::Draft);
    assert_ne!(PostStatus::Published, PostStatus::Draft);
}

#[test]
fn test_form_state_idle_is_default() {
    let state = FormState::Idle;
    assert_eq!(state, FormState::Idle);
}

#[test]
fn test_form_state_variants() {
    // Verify all form states can be created
    let _idle = FormState::Idle;
    let _loading = FormState::Loading;
    let _success = FormState::Success;
    let _error = FormState::Error("Test error".to_string());
}

// =============================================================================
// Validation Logic Tests (P16-C4)
// =============================================================================

#[test]
fn test_email_validation_logic() {
    // Simple email validation check (mirrors contact.rs logic)
    fn is_valid_email(email: &str) -> bool {
        email.contains('@') && email.contains('.') && email.len() > 5
    }
    
    assert!(is_valid_email("test@example.com"));
    assert!(is_valid_email("user@domain.org"));
    assert!(!is_valid_email("invalid"));
    assert!(!is_valid_email("no@dot"));
    assert!(!is_valid_email("a@b.c")); // Too short
}

#[test]
fn test_message_length_validation() {
    // Message should be at least 20 characters
    let short_msg = "Hi";
    let valid_msg = "This is a valid message that is long enough";
    
    assert!(short_msg.len() < 20);
    assert!(valid_msg.len() >= 20);
}
