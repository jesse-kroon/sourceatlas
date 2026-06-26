use crate::file_stats::FileStats;

#[derive(Default)]
pub struct Report {
    files: Vec<FileStats>,
    total_directories: usize,
    total_files: usize,
    total_lines: usize,
    total_characters: usize,
    total_blank_lines: usize,
    total_non_blank_lines: usize,
    total_functions: usize,
    total_todos: usize,
}

impl Report {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_file(&mut self, file: FileStats) {
        self.files.push(file);
    }

    pub fn add_directory(&mut self) {
        self.total_directories += 1;
    }

    pub fn generate(&mut self) {
        self.total_files = self.files.len();
        self.total_lines = 0;
        self.total_characters = 0;
        self.total_blank_lines = 0;
        self.total_non_blank_lines = 0;
        self.total_functions = 0;
        self.total_todos = 0;

        for file in &self.files {
            self.total_lines += file.total_lines;
            self.total_characters += file.total_characters;
            self.total_blank_lines += file.total_blank_lines;
            self.total_non_blank_lines += file.total_non_blank_lines;
            self.total_functions += file.total_functions;
            self.total_todos += file.total_todos;
        }
    }

    pub fn print(&self) {
        println!("REPOLENS");
        println!("--------");
        println!("total directories: {}", self.total_directories);
        println!("total files: {}", self.total_files);
        println!("total lines: {}", self.total_lines);
        println!("total characters: {}", self.total_characters);
        println!("total non-blank lines: {}", self.total_non_blank_lines);
        println!("total blank lines: {}", self.total_blank_lines);
        println!("total functions: {}", self.total_functions);
        println!("total TODO's: {}", self.total_todos);
    }
}
