use std::{fmt, path::Path};

use crate::language::{parser::LanguageParser, rust::RustParser};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Language::Rust => write!(f, "Rust"),
        }
    }
}
