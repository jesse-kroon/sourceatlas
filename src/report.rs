use crate::file_stats::FileStats;

#[derive(Default)]
pub struct Report {
    files: Vec<FileStats>,
    total_directories: usize,
    total_files_found: usize,
    total_files_analyzed: usize,
    total_files_skipped: usize,
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

    pub fn record_directory_found(&mut self) {
        self.total_directories += 1;
    }

    pub fn record_file_found(&mut self) {
        self.total_files_found += 1;
    }

    pub fn record_skipped_file(&mut self) {
        self.total_files_skipped += 1
    }

    pub fn total_files(&self) -> usize {
        self.total_files_found
    }

    pub fn total_files_skipped(&self) -> usize {
        self.total_files_skipped
    }

    pub fn total_files_analyzed(&self) -> usize {
        self.total_files_analyzed
    }

    pub fn generate(&mut self) {
        Self::default();
        self.total_files_analyzed = self.files.len();

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
        println!("----------");
        println!("|REPOLENS|");
        println!("----------");
        println!("");

        println!("DIRECTORIES");
        println!("--------");
        println!("total directories: {}", self.total_directories);
        println!("");

        println!("FILES");
        println!("--------");
        println!("total files found: {}", self.total_files_found);
        println!("total files analyzed: {}", self.total_files_analyzed);
        println!("total files skipped: {}", self.total_files_skipped);
        println!("total lines: {}", self.total_lines);
        println!("total characters: {}", self.total_characters);
        println!("total non-blank lines: {}", self.total_non_blank_lines);
        println!("total blank lines: {}", self.total_blank_lines);
        println!("total functions: {}", self.total_functions);
        println!("total TODO's: {}", self.total_todos);
    }
}
