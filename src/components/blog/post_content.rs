use dioxus::prelude::*;
use crate::i18n::Language;
use crate::components::blog::BlogPost;

#[component]
pub fn BlogPostContent(post: BlogPost, lang: Language) -> Element {
    rsx! {
        // Enforce Serenity Typography directly in component to bypass CSS pipeline issues
        style {
            "
            .prose h1, .prose h2, .prose h3, .prose h4, .prose h5, .prose h6 {{
                font-family: 'Inter', system-ui, -apple-system, sans-serif !important;
                color: #FFFFFF !important;
                text-shadow: none !important;
            }}
            "
        }
        
        // Prose Container - Transparent, no borders, pure content.
        article { class: "prose prose-invert max-w-none",
            div { dangerous_inner_html: "{post.content(&lang)}" }
        }
    }
}
