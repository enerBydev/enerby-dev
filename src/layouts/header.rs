//! Header Component - Navbar and Navigation
//! Includes responsive mobile menu and active route styling

use dioxus::prelude::*;
use crate::routes::Route;
use crate::config::{SITE, SOCIAL_LINKS};

#[component]
pub fn Header() -> Element {
    // Mobile menu state
    let mut is_menu_open = use_signal(|| false);
    
    // Get current route for active state (requires implicit router context from RootLayout)
    // In Dioxus 0.6/0.7 use_route might be explicit
    // Since we are inside Router, we can use use_route
    // let route = use_route::<Route>(); 
    // BUT common pattern for "Link" in Dioxus handles styling automatically via "active_class" usually?
    // Dioxus 0.7 Link has `active_class`. Let's use that!
    
    let active_class = "text-primary border-b-2 border-primary";
    let base_class = "nav-link text-secondary hover:text-primary transition-colors text-sm uppercase tracking-wider py-1";

    let toggle_menu = move |_| {
        is_menu_open.set(!is_menu_open());
    };

    // Dynamic classes
    let top_bar_class = if is_menu_open() { "top-2.5 rotate-45" } else { "top-0" };
    let middle_bar_class = if is_menu_open() { "opacity-0" } else { "opacity-100" };
    let bottom_bar_class = if is_menu_open() { "top-2.5 -rotate-45" } else { "top-5" };
    let mobile_menu_class = if is_menu_open() { "max-h-96 opacity-100" } else { "max-h-0 opacity-0" };

    rsx! {
        header { 
            class: "header glass sticky top-0 z-50 transition-all duration-300 backdrop-blur-md bg-opacity-80 bg-bg-primary border-b border-white/5",
            
            nav { class: "navbar container flex justify-between items-center py-4",
                // Logo with Glitch Effect
                Link { 
                    to: Route::HomePage {},
                    class: "logo font-display text-2xl font-bold neon-text relative group",
                    onclick: move |_| is_menu_open.set(false),
                    
                    span { class: "group-hover:animate-glitch", "{SITE.name}" }
                }
                
                // Desktop Navigation
                ul { class: "nav-links hidden md:flex gap-8 items-center",
                    li {
                        Link { 
                            to: Route::HomePage {},
                            class: base_class,
                            active_class: active_class,
                            "Home"
                        }
                    }
                    li {
                        Link { 
                            to: Route::AboutPage {},
                            class: base_class,
                            active_class: active_class,
                            "About"
                        }
                    }
                    li {
                        Link { 
                            to: Route::ProjectsPage {},
                            class: base_class,
                            active_class: active_class,
                            "Projects"
                        }
                    }
                    li {
                        Link { 
                            to: Route::BlogPage {},
                            class: base_class,
                            active_class: active_class,
                            "Blog"
                        }
                    }
                    li {
                        Link { 
                            to: Route::ContactPage {},
                            class: base_class,
                            active_class: active_class,
                            "Contact"
                        }
                    }
                    
                    // Social Icons (Desktop)
                    div { class: "social-links flex gap-4 ml-4 pl-4 border-l border-white/10",
                        for social in SOCIAL_LINKS {
                            a {
                                href: "{social.url}",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                class: "text-muted hover:text-primary transition-colors",
                                title: "{social.name}",
                                span { class: "font-mono font-bold", "{social.name.chars().next().unwrap_or('?')}" } 
                            }
                        }
                    }
                }
                
                // Mobile Menu Button (Hamburger)
                button { 
                    class: "md:hidden text-primary focus:outline-none p-2",
                    onclick: toggle_menu,
                    div { class: "relative w-6 h-5",
                        // Top bar
                        span { 
                            class: "absolute left-0 w-full h-0.5 bg-current transition-all duration-300 {top_bar_class}"
                        }
                        // Middle bar
                        span { 
                            class: "absolute left-0 w-full h-0.5 bg-current transition-all duration-300 top-2.5 {middle_bar_class}"
                        }
                        // Bottom bar
                        span { 
                            class: "absolute left-0 w-full h-0.5 bg-current transition-all duration-300 {bottom_bar_class}"
                        }
                    }
                }
            }
            
            // Mobile Menu Overlay
            div { 
                class: "mobile-menu md:hidden absolute top-full left-0 w-full bg-bg-card/95 backdrop-blur-xl border-b border-primary/20 overflow-hidden transition-all duration-300 {mobile_menu_class}",
                ul { class: "flex flex-col p-6 space-y-4 items-center",
                    li { Link { to: Route::HomePage {}, class: "text-lg", onclick: move |_| is_menu_open.set(false), "Home" } }
                    li { Link { to: Route::AboutPage {}, class: "text-lg", onclick: move |_| is_menu_open.set(false), "About" } }
                    li { Link { to: Route::ProjectsPage {}, class: "text-lg", onclick: move |_| is_menu_open.set(false), "Projects" } }
                    li { Link { to: Route::BlogPage {}, class: "text-lg", onclick: move |_| is_menu_open.set(false), "Blog" } }
                    li { Link { to: Route::ContactPage {}, class: "text-lg", onclick: move |_| is_menu_open.set(false), "Contact" } }
                }
            }
        }
    }
}
