pub trait LanguageParser {
    fn is_todo(&self, line: &str) -> bool;
    fn is_comment(&self, line: &str) -> bool;
    fn is_function(&self, line: &str) -> bool;
}
