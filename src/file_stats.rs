pub struct FileStats {
    pub(crate) total_lines: usize,
    pub(crate) total_characters: usize,
    pub(crate) total_blank_lines: usize,
    pub(crate) total_non_blank_lines: usize,
    pub(crate) total_functions: usize,
    pub(crate) total_todos: usize,
}

impl FileStats {
    pub fn new(source: &str) -> Self {
        Self {
            total_lines: count_file_lines(source),
            total_characters: count_file_characters(source),
            total_blank_lines: count_file_blank_lines(source),
            total_non_blank_lines: count_file_non_blank_lines(source),
            total_functions: count_file_functions(source),
            total_todos: count_file_todos(source),
        }
    }
}

fn count_file_lines(source: &str) -> usize {
    source.lines().count()
}

fn count_file_characters(source: &str) -> usize {
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

fn count_file_functions(source: &str) -> usize {
    source
        .lines()
        .filter(|line| {
            let line = line.trim_start();
            line.starts_with("fn")
        })
        .count()
}

fn count_file_todos(source: &str) -> usize {
    source.to_ascii_lowercase().matches("todo").count()
}
