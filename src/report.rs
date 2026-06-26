use crate::file_stats::FileStats;

pub struct Report {
    files: Vec<FileStats>,
    pub total_files: usize,
    pub total_lines: usize,
    pub total_characters: usize,
    pub total_blank_lines: usize,
    pub total_non_blank_lines: usize,
    pub total_functions: usize,
    pub total_todos: usize,
}

impl Report {
    pub fn new() -> Self {
        Self {
            files: Vec::new(),
            total_files: 0,
            total_lines: 0,
            total_characters: 0,
            total_blank_lines: 0,
            total_non_blank_lines: 0,
            total_functions: 0,
            total_todos: 0,
        }
    }

    pub fn add_file(&mut self, file: FileStats) {
        self.files.push(file);
    }

    pub fn generate(&mut self) {
        self.total_files += self.files.iter().count();
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

    pub fn print(self) {
        println!("REPOLENS");
        println!("--------");
        println!("total files: {}", self.total_files);
        println!("total lines: {}", self.total_lines);
        println!("total characters: {}", self.total_characters);
        println!("total non-blank lines: {}", self.total_non_blank_lines);
        println!("total blank lines: {}", self.total_blank_lines);
        println!("total functions: {}", self.total_functions);
        println!("total TODO's: {}", self.total_todos);
    }
}
