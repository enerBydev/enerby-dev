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
                    
                    // Social Icons (Desktop) - Material Design Icons
                    div { class: "social-links flex gap-4 ml-4 pl-4 border-l border-white/10",
                        // GitHub
                        a {
                            href: "https://github.com/enerBydev",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "text-muted hover:text-primary transition-colors",
                            title: "GitHub",
                            svg {
                                class: "w-5 h-5",
                                fill: "currentColor",
                                view_box: "0 0 24 24",
                                path { d: "M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0024 12c0-6.63-5.37-12-12-12z" }
                            }
                        }
                        // LinkedIn
                        a {
                            href: "https://linkedin.com/in/enerbydev",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "text-muted hover:text-primary transition-colors",
                            title: "LinkedIn",
                            svg {
                                class: "w-5 h-5",
                                fill: "currentColor",
                                view_box: "0 0 24 24",
                                path { d: "M20.447 20.452h-3.554v-5.569c0-1.328-.027-3.037-1.852-3.037-1.853 0-2.136 1.445-2.136 2.939v5.667H9.351V9h3.414v1.561h.046c.477-.9 1.637-1.85 3.37-1.85 3.601 0 4.267 2.37 4.267 5.455v6.286zM5.337 7.433c-1.144 0-2.063-.926-2.063-2.065 0-1.138.92-2.063 2.063-2.063 1.14 0 2.064.925 2.064 2.063 0 1.139-.925 2.065-2.064 2.065zm1.782 13.019H3.555V9h3.564v11.452zM22.225 0H1.771C.792 0 0 .774 0 1.729v20.542C0 23.227.792 24 1.771 24h20.451C23.2 24 24 23.227 24 22.271V1.729C24 .774 23.2 0 22.222 0h.003z" }
                            }
                        }
                        // Twitter/X
                        a {
                            href: "https://twitter.com/enerbydev",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "text-muted hover:text-primary transition-colors",
                            title: "Twitter",
                            svg {
                                class: "w-5 h-5",
                                fill: "currentColor",
                                view_box: "0 0 24 24",
                                path { d: "M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.084 4.126H5.117z" }
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
