//! Utils Tests (P16-A2)
//! Tests for utility functions

use crate::utils::*;

#[test]
fn test_format_date_valid() {
    // Test with ISO date format
    let result = format_date("2024-01-15");
    assert!(!result.is_empty(), "Should format valid date");
}

#[test]
fn test_format_date_invalid() {
    // Test with invalid date - should return original or fallback
    let result = format_date("invalid-date");
    assert!(!result.is_empty(), "Should handle invalid date gracefully");
}

#[test]
fn test_truncate_string_short() {
    let short_text = "Hello";
    let result = truncate_string(short_text, 10);
    assert_eq!(result, "Hello", "Should not truncate short strings");
}

#[test]
fn test_truncate_string_long() {
    let long_text = "This is a very long text that should be truncated";
    let result = truncate_string(long_text, 20);
    assert!(result.len() <= 23, "Should truncate long strings"); // 20 + "..."
    assert!(result.ends_with("..."), "Should add ellipsis");
}

#[test]
fn test_slugify_basic() {
    assert_eq!(slugify("Hello World"), "hello-world");
    assert_eq!(slugify("Test String"), "test-string");
}

#[test]
fn test_slugify_special_chars() {
    assert_eq!(slugify("Hello, World!"), "hello-world");
    // Special chars become hyphens, consecutive hyphens are collapsed
    assert_eq!(slugify("Test@#$String"), "test-string");
}

#[test]
fn test_slugify_multiple_spaces() {
    assert_eq!(slugify("Hello   World"), "hello-world");
}

#[test]
fn test_calculate_read_time_short() {
    let short_content = "This is a short paragraph.";
    let time = calculate_read_time(short_content);
    assert_eq!(time, 1, "Minimum read time should be 1 minute");
}

#[test]
fn test_calculate_read_time_long() {
    // ~1000 words (200 words per minute = 5 minutes)
    let long_content = "word ".repeat(1000);
    let time = calculate_read_time(&long_content);
    assert!(time >= 4 && time <= 6, "Should calculate ~5 minutes for 1000 words");
}

#[test]
fn test_is_external_link() {
    assert!(is_external_link("https://example.com"));
    assert!(is_external_link("http://example.com"));
    assert!(!is_external_link("/about"));
    assert!(!is_external_link("#section"));
}
