//! Root Layout
//! Puts it all together: Header + Main + Footer + Global Overlays

use dioxus::prelude::*;
use crate::routes::Route;
use crate::layouts::header::Header;
use crate::layouts::footer::Footer;

#[component]
pub fn RootLayout() -> Element {
    rsx! {
        div { class: "app-wrapper min-h-screen flex flex-col cyber-grid relative overflow-x-hidden",
            
            // Global Overlays (P4-A3, P4-A4)
            div { class: "scanlines fixed inset-0 pointer-events-none z-[900]" }
            div { class: "grain fixed inset-0 pointer-events-none z-[901] opacity-[0.03]" }
            
            // Header
            Header {}
            
            // Main Content
            main { class: "main-content flex-grow relative z-10 animate-fade-in",
                Outlet::<Route> {}
            }
            
            // Footer
            Footer {}
        }
    }
}
