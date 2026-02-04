//! Contact Section Component
//! Contact form with validation and alternative contact info

use dioxus::prelude::*;
use crate::components::molecules::{Card, SectionTitle};
use crate::components::atoms::{Button, ButtonVariant, Badge};
use crate::components::layout_components::{Container, Section, Grid};

/// Form State (P11-C)
#[derive(Clone, PartialEq)]
pub enum FormState {
    Idle,
    Loading,
    Success,
    Error(String),
}

/// Contact Form Data
#[derive(Clone, Default)]
struct ContactFormData {
    name: String,
    email: String,
    subject: String,
    message: String,
}

/// Validation Errors
#[derive(Clone, Default)]
struct ValidationErrors {
    name: Option<String>,
    email: Option<String>,
    subject: Option<String>,
    message: Option<String>,
}

impl ValidationErrors {
    fn has_errors(&self) -> bool {
        self.name.is_some() || self.email.is_some() || self.subject.is_some() || self.message.is_some()
    }
}

/// Validate email format (P11-B2)
fn is_valid_email(email: &str) -> bool {
    email.contains('@') && email.contains('.') && email.len() > 5
}

/// Validate form (P11-B1, P11-B2, P11-B3)
fn validate_form(data: &ContactFormData) -> ValidationErrors {
    let mut errors = ValidationErrors::default();
    
    if data.name.trim().is_empty() {
        errors.name = Some("Name is required".to_string());
    }
    
    if data.email.trim().is_empty() {
        errors.email = Some("Email is required".to_string());
    } else if !is_valid_email(&data.email) {
        errors.email = Some("Please enter a valid email".to_string());
    }
    
    if data.subject.trim().is_empty() {
        errors.subject = Some("Subject is required".to_string());
    }
    
    if data.message.trim().is_empty() {
        errors.message = Some("Message is required".to_string());
    } else if data.message.len() < 20 {
        errors.message = Some("Message must be at least 20 characters".to_string());
    }
    
    errors
}

/// Contact Section (P11-A1)
#[component]
pub fn ContactSection() -> Element {
    rsx! {
        Section { id: "contact", alternate: true,
            Container {
                SectionTitle { 
                    text: "Get In Touch".to_string(), 
                    subtitle: "Let's Work Together".to_string(),
                    center: true 
                }
                
                div { class: "grid lg:grid-cols-2 gap-12 max-w-5xl mx-auto",
                    // Contact Form
                    div {
                        ContactForm {}
                    }
                    
                    // Contact Info (P11-E)
                    div { class: "space-y-6",
                        ContactInfoCard {}
                        SocialLinks {}
                    }
                }
            }
        }
    }
}

/// Contact Form Component (P11-A2)
#[component]
fn ContactForm() -> Element {
    let mut form_data = use_signal(ContactFormData::default);
    let mut errors = use_signal(ValidationErrors::default);
    let mut form_state = use_signal(|| FormState::Idle);
    
    let on_submit = move |evt: FormEvent| {
        evt.prevent_default();
        
        // Validate
        let validation_errors = validate_form(&form_data());
        errors.set(validation_errors.clone());
        
        if validation_errors.has_errors() {
            return;
        }
        
        // Simulate submission (P11-D1 placeholder)
        form_state.set(FormState::Loading);
        
        // In a real app, this would be an async call
        // For now, simulate success after "submission"
        spawn(async move {
            // Simulated delay would go here
            form_state.set(FormState::Success);
        });
    };
    
    // Reset form (P11-C4)
    let reset_form = move |_| {
        form_data.set(ContactFormData::default());
        errors.set(ValidationErrors::default());
        form_state.set(FormState::Idle);
    };

    rsx! {
        Card {
            match form_state() {
                FormState::Success => rsx! {
                    div { class: "text-center py-8",
                        div { class: "text-5xl mb-4", "✅" }
                        h3 { class: "text-xl font-bold text-primary mb-2", "Message Sent!" }
                        p { class: "text-muted mb-4", "Thank you for reaching out. I'll get back to you soon." }
                        Button { 
                            variant: ButtonVariant::Ghost,
                            onclick: reset_form,
                            "Send Another Message"
                        }
                    }
                },
                FormState::Error(ref msg) => rsx! {
                    div { class: "text-center py-8",
                        div { class: "text-5xl mb-4", "❌" }
                        h3 { class: "text-xl font-bold text-red-400 mb-2", "Something went wrong" }
                        p { class: "text-muted mb-4", "{msg}" }
                        Button { 
                            variant: ButtonVariant::Ghost,
                            onclick: reset_form,
                            "Try Again"
                        }
                    }
                },
                _ => rsx! {
                    form { 
                        onsubmit: on_submit,
                        class: "space-y-4",
                        
                        // Name Field (P11-A3)
                        FormField {
                            label: "Name".to_string(),
                            field_type: "text".to_string(),
                            placeholder: "Your Name".to_string(),
                            value: form_data().name,
                            error: errors().name,
                            oninput: move |evt: FormEvent| {
                                form_data.write().name = evt.value();
                            }
                        }
                        
                        // Email Field (P11-A4)
                        FormField {
                            label: "Email".to_string(),
                            field_type: "email".to_string(),
                            placeholder: "your@email.com".to_string(),
                            value: form_data().email,
                            error: errors().email,
                            oninput: move |evt: FormEvent| {
                                form_data.write().email = evt.value();
                            }
                        }
                        
                        // Subject Field (P11-A5)
                        FormField {
                            label: "Subject".to_string(),
                            field_type: "text".to_string(),
                            placeholder: "What's this about?".to_string(),
                            value: form_data().subject,
                            error: errors().subject,
                            oninput: move |evt: FormEvent| {
                                form_data.write().subject = evt.value();
                            }
                        }
                        
                        // Message Field (P11-A6)
                        div { class: "space-y-1",
                            label { class: "text-sm font-medium text-secondary", "Message" }
                            textarea {
                                class: "w-full px-4 py-3 bg-bg-element border border-white/10 rounded-lg text-white placeholder-muted focus:border-primary focus:ring-1 focus:ring-primary outline-none transition-all resize-none",
                                placeholder: "Your message (min 20 characters)...",
                                rows: "5",
                                value: "{form_data().message}",
                                oninput: move |evt: FormEvent| {
                                    form_data.write().message = evt.value();
                                }
                            }
                            if let Some(error) = errors().message {
                                p { class: "text-xs text-red-400", "{error}" }
                            }
                        }
                        
                        // Submit Button (P11-A7)
                        div { class: "pt-4",
                            Button { 
                                variant: ButtonVariant::Neon,
                                class: "w-full".to_string(),
                                if matches!(form_state(), FormState::Loading) {
                                    "Sending..."
                                } else {
                                    "Send Message"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Form Field Component
#[component]
fn FormField(
    label: String,
    field_type: String,
    placeholder: String,
    value: String,
    error: Option<String>,
    oninput: EventHandler<FormEvent>,
) -> Element {
    let border_class = if error.is_some() { "border-red-400" } else { "border-white/10" };
    
    rsx! {
        div { class: "space-y-1",
            label { class: "text-sm font-medium text-secondary", "{label}" }
            input {
                r#type: "{field_type}",
                class: "w-full px-4 py-3 bg-bg-element border {border_class} rounded-lg text-white placeholder-muted focus:border-primary focus:ring-1 focus:ring-primary outline-none transition-all",
                placeholder: "{placeholder}",
                value: "{value}",
                oninput: move |evt| oninput.call(evt)
            }
            if let Some(err) = error {
                p { class: "text-xs text-red-400", "{err}" }
            }
        }
    }
}

/// Contact Info Card (P11-E1, P11-E3)
#[component]
fn ContactInfoCard() -> Element {
    rsx! {
        Card {
            h3 { class: "text-lg font-bold text-white mb-4", "Contact Info" }
            
            div { class: "space-y-4",
                // Email
                div { class: "flex items-center gap-3",
                    span { class: "text-2xl", "📧" }
                    div {
                        p { class: "text-xs text-muted", "Email" }
                        a { 
                            class: "text-primary hover:underline",
                            href: "mailto:hello@enerby.dev",
                            "hello@enerby.dev"
                        }
                    }
                }
                
                // Location
                div { class: "flex items-center gap-3",
                    span { class: "text-2xl", "📍" }
                    div {
                        p { class: "text-xs text-muted", "Location" }
                        p { class: "text-secondary", "Mexico 🇲🇽" }
                    }
                }
                
                // Availability
                div { class: "flex items-center gap-3",
                    span { class: "text-2xl", "⏰" }
                    div {
                        p { class: "text-xs text-muted", "Availability" }
                        Badge { color: "cyan".to_string(), "Open for Projects" }
                    }
                }
            }
        }
    }
}

/// Social Links (P11-E2)
#[component]
fn SocialLinks() -> Element {
    let socials = vec![
        ("GitHub", "https://github.com/enerbydev", "🐙"),
        ("LinkedIn", "https://linkedin.com/in/enerbydev", "💼"),
        ("Twitter", "https://twitter.com/enerbydev", "🐦"),
    ];
    
    rsx! {
        Card {
            h3 { class: "text-lg font-bold text-white mb-4", "Connect" }
            
            div { class: "flex gap-4",
                for (name, url, emoji) in socials.iter() {
                    a {
                        href: "{url}",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        class: "flex-1 flex flex-col items-center p-4 bg-bg-element rounded-lg hover:bg-primary/10 hover:border-primary/30 border border-transparent transition-all group",
                        span { class: "text-2xl mb-1", "{emoji}" }
                        span { class: "text-xs text-muted group-hover:text-primary", "{name}" }
                    }
                }
            }
        }
    }
}
