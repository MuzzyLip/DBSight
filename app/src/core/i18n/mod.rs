use gpui::Global;
use locale_config::Locale;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    En,
    ZhCN,
}

impl Language {
    pub fn code(&self) -> &'static str {
        match self {
            Language::En => "en",
            Language::ZhCN => "zh-cn",
        }
    }

    #[allow(clippy::if_same_then_else)]
    pub fn from(locale: Locale) -> Self {
        let lang_code = locale.to_string().to_lowercase();
        if lang_code.starts_with("zh-cn") {
            Language::ZhCN
        } else if lang_code.starts_with("en") {
            Language::En
        } else {
            Language::En
        }
    }
}

#[derive(Debug, Clone)]
pub struct I18n {
    lang: Language,
    dict: HashMap<String, String>,
}

impl I18n {
    pub fn new() -> Self {
        let locale = Locale::user_default();
        let lang = Language::from(locale);
        Self::with_lang(lang)
    }

    pub fn with_lang(lang: Language) -> Self {
        let dict = Self::load_language(lang);
        Self { lang, dict }
    }

    pub fn lang(&self) -> Language {
        self.lang
    }

    pub fn set_lang(&mut self, lang: Language) {
        self.lang = lang;
        self.dict = Self::load_language(lang);
    }

    pub fn t(&self, key: &str) -> String {
        match self.dict.get(key) {
            Some(value) => value.clone(),
            None => {
                #[cfg(debug_assertions)]
                {
                    eprintln!("[I18n] Key not found: {}", key);
                }
                String::new()
            }
        }
    }

    pub fn t_with(&self, key: &str, params: &[(&str, &str)]) -> String {
        let mut text = self.t(key);
        if text.is_empty() {
            return text;
        }
        for (name, value) in params {
            let placeholder = format!("{{{{{}}}}}", name);
            text = text.replace(&placeholder, value);
        }

        text
    }

    fn load_language(lang: Language) -> HashMap<String, String> {
        let content = match lang {
            Language::En => include_str!("en.toml"),
            Language::ZhCN => include_str!("zh-cn.toml"),
        };

        toml::from_str(content).expect("Invalid TOML format")
    }
}

impl Default for I18n {
    fn default() -> Self {
        Self::new()
    }
}

impl Global for I18n {}
