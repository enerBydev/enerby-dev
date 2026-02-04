//! Data Module - Types and structures for content
//! Defines Project, BlogPost, Skill and other data types

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Project data structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Project {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub long_description: Option<String>,
    pub image: Option<String>,
    pub technologies: Vec<String>,
    pub github_url: Option<String>,
    pub live_url: Option<String>,
    pub featured: bool,
    pub date: String,
    pub status: ProjectStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProjectStatus {
    Completed,
    InProgress,
    Planned,
}

/// Blog post data structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BlogPost {
    pub slug: String,
    pub title: String,
    pub excerpt: String,
    pub content: String,
    pub date: String,
    pub tags: Vec<String>,
    pub reading_time: u32,
    pub published: bool,
}

/// Skill category
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SkillCategory {
    pub name: String,
    pub icon: String,
    pub skills: Vec<Skill>,
}

/// Individual skill
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Skill {
    pub name: String,
    pub level: u8, // 1-100
    pub years: Option<f32>,
}

/// Contact form data
#[derive(Debug, Clone, Default)]
pub struct ContactForm {
    pub name: String,
    pub email: String,
    pub subject: String,
    pub message: String,
}

/// Timeline item for About page
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineItem {
    pub year: String,
    pub title: String,
    pub description: String,
    pub icon: String,
}
