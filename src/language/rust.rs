use crate::language::parser::LanguageParser;

pub struct RustParser;

impl LanguageParser for RustParser {
    fn is_comment(&self, line: &str) -> bool {
        line.trim().starts_with("//")
    }

    fn is_todo(&self, line: &str) -> bool {
        self.is_comment(line) && line.trim().to_lowercase().contains("todo")
    }

    fn is_function(&self, line: &str) -> bool {
        line.trim().starts_with("fn")
    }
}
