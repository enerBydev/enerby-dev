//! Footer Component
//! Contains copyright, social links, and proper attribution

use crate::config::{SITE, SOCIAL_LINKS};
use crate::routes::Route;
use chrono::{Datelike, Utc};
use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    let current_year = Utc::now().year();
    let i18n = crate::i18n::use_i18n();

    rsx! {
        footer { class: "footer py-12 mt-auto border-t border-white/5",
            div { class: "container flex flex-col md:flex-row justify-between items-center gap-6",

                // Left: Copyright & Tech Stack
                div { class: "text-center md:text-left",
                    p { class: "text-muted text-sm",
                        "© {current_year} {SITE.name}."
                    }
                    p { class: "text-xs text-muted mt-1",
                        "{i18n.footer.built_with}"
                        span { class: "text-primary font-semibold", "Rust" }
                        "{i18n.footer.and}"
                        span { class: "text-primary font-semibold", "Dioxus" }
                    }
                }

                // Center: Quick Links
                div { class: "hidden md:flex gap-6",
                    Link { to: Route::HomePage {}, class: "text-muted hover:text-primary text-sm transition-colors", "{i18n.nav.home}" }
                    Link { to: Route::AboutPage {}, class: "text-muted hover:text-primary text-sm transition-colors", "{i18n.nav.about}" }
                    Link { to: Route::ProjectsPage {}, class: "text-muted hover:text-primary text-sm transition-colors", "{i18n.nav.projects}" }
                    Link { to: Route::BlogPage {}, class: "text-muted hover:text-primary text-sm transition-colors", "{i18n.nav.blog}" }
                }

                // Right: Social Links
                div { class: "flex gap-6",
                    for social in SOCIAL_LINKS {
                        a {
                            href: "{social.url}",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "text-secondary hover:text-primary transition-all hover:scale-110",
                            "{social.name}"
                        }
                    }
                }
            }
        }
    }
}
