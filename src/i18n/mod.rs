//! Internationalization Module (i18n)
//! Handles language detection, context, and hooking.

use dioxus::prelude::*;
use std::fmt::{Display, Formatter, Result as FmtResult};

pub mod dict;
pub mod locales;

use dict::Dictionary;

/// Supported Languages
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Language {
    #[default]
    EN,
    ES,
}

impl Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Language::EN => write!(f, "en"),
            Language::ES => write!(f, "es"),
        }
    }
}

impl Language {
    /// Get the dictionary for this language
    pub fn dict(&self) -> Dictionary {
        match self {
             Language::EN => locales::EN.clone(),
             Language::ES => locales::ES.clone(),
        }
    }
    pub fn from_str(s: &str) -> Option<Self> {
        let s = s.to_lowercase();
        if s.starts_with("es") {
            Some(Language::ES)
        } else if s.starts_with("en") {
            Some(Language::EN)
        } else {
            None
        }
    }
}

/// Global State for I18n
/// Using a GlobalSignal to allow easy access and updates from anywhere.
pub static I18N_CONFIG: GlobalSignal<I18nState> = Signal::global(|| I18nState::new());

#[derive(Clone, Copy)]
pub struct I18nState {
    pub language: Language,
}

impl I18nState {
    pub fn new() -> Self {
        let lang = load_language_preference().unwrap_or_else(detect_browser_language);
        Self { language: lang }
    }

    pub fn dict(&self) -> Dictionary {
        self.language.dict()
    }
}

/// Helper to get the current dictionary in components
/// Usage: `let i18n = use_i18n(); p { "{i18n.hero.greeting}" }`
pub fn use_i18n() -> Dictionary {
    I18N_CONFIG.read().dict()
}

/// Helper to toggle language
pub fn toggle_language() {
    let mut config = I18N_CONFIG.write();
    let new_lang = match config.language {
        Language::EN => Language::ES,
        Language::ES => Language::EN,
    };
    config.language = new_lang;
    save_language_preference(new_lang);
}

const STORAGE_KEY: &str = "enerby_lang_pref";

fn save_language_preference(_lang: Language) {
    #[cfg(target_arch = "wasm32")]
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            let val = match _lang { Language::EN => "en", Language::ES => "es" };
            let _ = storage.set_item(STORAGE_KEY, val);
        }
    }
}

fn load_language_preference() -> Option<Language> {
    #[cfg(target_arch = "wasm32")]
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            // Explicitly annotate type to avoid E0282 and E0277
            let item_result: Result<Option<String>, _> = storage.get_item(STORAGE_KEY);
            if let Ok(Some(val)) = item_result {
               return Language::from_str(&val);
            }
        }
    }
    None
}

/// Detect browser language using web-sys
#[cfg(target_arch = "wasm32")]
fn detect_browser_language() -> Language {
    if let Some(window) = web_sys::window() {
        let navigator = window.navigator(); 
        if let Some(lang) = navigator.language() {
             if let Some(detected) = Language::from_str(&lang) {
                 return detected;
             }
        }
    }
    Language::EN
}

/// Fallback detection for server-side rendering or non-wasm
#[cfg(not(target_arch = "wasm32"))]
fn detect_browser_language() -> Language {
    Language::EN
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_display() {
        assert_eq!(format!("{}", Language::EN), "en");
        assert_eq!(format!("{}", Language::ES), "es");
    }

    #[test]
    fn test_dict_mock_access() {
        let en_dict = Language::EN.dict();
        let es_dict = Language::ES.dict();
        
        assert_eq!(en_dict.nav.home, "Home");
        assert_eq!(es_dict.nav.home, "Inicio");
    }

    #[test]
    fn test_parsing_logic() {
        assert_eq!(Language::from_str("en"), Some(Language::EN));
        assert_eq!(Language::from_str("es"), Some(Language::ES));
        assert_eq!(Language::from_str("es-MX"), Some(Language::ES));
        assert_eq!(Language::from_str("en-US"), Some(Language::EN));
        assert_eq!(Language::from_str("de"), None);
        assert_eq!(Language::from_str("ES"), Some(Language::ES)); // Case insensitive test
    }
}
