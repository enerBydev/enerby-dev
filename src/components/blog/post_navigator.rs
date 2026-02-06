use dioxus::prelude::*;
use crate::components::atoms::{Button, ButtonVariant};
use crate::routes::Route;

#[component]
pub fn BlogPostNavigator() -> Element {
    rsx! {
        nav { class: "mb-12 flex justify-start",
            Button {
                variant: ButtonVariant::Ghost,
                to: Route::BlogPage {},
                "← Back to Blog"
            }
        }
    }
}
