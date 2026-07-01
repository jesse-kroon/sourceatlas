use crate::language::parser::LanguageParser;

pub struct FileStats {
    pub(crate) total_lines: usize,
    pub(crate) total_characters: usize,
    pub(crate) total_blank_lines: usize,
    pub(crate) total_non_blank_lines: usize,
    pub(crate) total_functions: usize,
    pub(crate) total_todos: usize,
}

impl FileStats {
    pub fn new(source: &str, parser: &dyn LanguageParser) -> Self {
        Self {
            total_lines: count_file_lines(source),
            total_characters: count_file_non_whitespace_characters(source),
            total_blank_lines: count_file_blank_lines(source),
            total_non_blank_lines: count_file_non_blank_lines(source),
            total_functions: count_file_functions(parser, source),
            total_todos: count_file_todos(parser, source),
        }
    }
}

fn count_file_lines(source: &str) -> usize {
    source.lines().count()
}

fn count_file_non_whitespace_characters(source: &str) -> usize {
    source.chars().filter(|char| !char.is_whitespace()).count()
}

fn count_file_non_blank_lines(source: &str) -> usize {
    source
        .lines()
        .filter(|line| !line.trim().is_empty())
        .count()
}

fn count_file_blank_lines(source: &str) -> usize {
    source.lines().filter(|line| line.trim().is_empty()).count()
}

fn count_file_functions(parser: &dyn LanguageParser, source: &str) -> usize {
    source
        .lines()
        .filter(|line| parser.is_function(line))
        .count()
}

fn count_file_todos(parser: &dyn LanguageParser, source: &str) -> usize {
    source.lines().filter(|line| parser.is_todo(line)).count()
}

#[cfg(test)]
mod tests {
    use crate::language::language::Language;

    use super::*;

    const TEST_SOURCE: &str = r#"fn main() {
        let first_name = "John";
        // TODO: add last name

    }"#;

    #[test]
    fn counts_lines() {
        assert_eq!(5, count_file_lines(TEST_SOURCE))
    }

    #[test]
    fn counts_blank_lines() {
        assert_eq!(1, count_file_blank_lines(TEST_SOURCE))
    }

    #[test]
    fn counts_non_whitespace_characters() {
        assert_eq!(49, count_file_non_whitespace_characters(TEST_SOURCE))
    }

    #[test]
    fn counts_todos() {
        let parser = Language::Rust.parser();

        assert_eq!(1, count_file_todos(parser.as_ref(), TEST_SOURCE));
    }

    #[test]
    fn counts_functions() {
        let parser = Language::Rust.parser();

        assert_eq!(1, count_file_functions(parser.as_ref(), TEST_SOURCE));
    }
}
