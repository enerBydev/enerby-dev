use dioxus::prelude::*;
use crate::i18n::Language;
use crate::components::blog::BlogPost;

#[component]
pub fn BlogPostHeader(post: BlogPost, lang: Language) -> Element {
    rsx! {
        header { class: "mb-16 text-center max-w-4xl mx-auto",
            // Tags - Subtle, minimalist
            div { class: "flex flex-wrap justify-center gap-2 mb-6",
                for tag in post.tags.iter() {
                    span { class: "text-xs font-mono uppercase tracking-widest text-primary/80", "#{tag}" }
                }
            }

            // Title - Large, bold, but clean (No Shadows)
            h1 { class: "text-4xl md:text-5xl font-bold text-white mb-6 font-display leading-tight tracking-tight",
                "{post.title(&lang)}"
            }
            
            // Meta Row - Minimalist
            div { class: "flex items-center justify-center gap-4 text-muted font-sans text-sm",
                span { "{post.date}" }
                span { class: "w-1 h-1 rounded-full bg-muted/50" }
                span { "{post.read_time} min read" }
            }
        }
    }
}
