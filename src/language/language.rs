use std::path::Path;

use crate::language::{parser::LanguageParser, rust::RustParser};

pub enum Language {
    Rust,
}

impl Language {
    pub fn from_path(path: &Path) -> Option<Self> {
        match path.extension().and_then(|e| e.to_str()) {
            Some("rs") => Some(Language::Rust),
            _ => None,
        }
    }

    pub fn parser(&self) -> &dyn LanguageParser {
        match self {
            Language::Rust => &RustParser,
        }
    }
}
