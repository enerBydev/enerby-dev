//! Contact Page - Contact form

use dioxus::prelude::*;

/// Contact page component
#[component]
pub fn ContactPage() -> Element {
    rsx! {
        section { class: "section",
            div { class: "container container-narrow",
                h1 { class: "section-title", "Contact" }

                div { class: "card",
                    form { class: "contact-form",
                        div { class: "form-group",
                            label { "Name" }
                            input {
                                class: "input",
                                r#type: "text",
                                placeholder: "Your name",
                                name: "name"
                            }
                        }

                        div { class: "form-group",
                            label { "Email" }
                            input {
                                class: "input",
                                r#type: "email",
                                placeholder: "your@email.com",
                                name: "email"
                            }
                        }

                        div { class: "form-group",
                            label { "Message" }
                            textarea {
                                class: "input",
                                placeholder: "Your message...",
                                name: "message",
                                rows: "5"
                            }
                        }

                        button {
                            class: "btn btn-primary",
                            r#type: "submit",
                            "Send Message"
                        }
                    }
                }
            }
        }
    }
}
