//! 404 Not Found Page

use crate::routes::Route;
use dioxus::prelude::*;

/// 404 Not Found page
#[component]
pub fn NotFoundPage(route: Vec<String>) -> Element {
    let path = route.join("/");

    rsx! {
        section { class: "section",
            div { class: "container",
                div { class: "not-found-content",
                    h1 { class: "neon-text", "404" }
                    h2 { "Page Not Found" }
                    p { class: "text-secondary",
                        "The page \"/{path}\" doesn't exist."
                    }
                    Link { to: Route::HomePage {},
                        class: "btn btn-primary",
                        "← Back to Home"
                    }
                }
            }
        }
    }
}
