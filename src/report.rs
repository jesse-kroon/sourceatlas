use std::collections::HashMap;

use crate::{file_stats::FileStats, language::Language};

#[derive(Default)]
pub struct Report {
    files: Vec<FileReport>,
    total_directories_scanned: usize,
    total_files_scanned: usize,
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

    pub fn add_file(&mut self, file: FileReport) {
        self.files.push(file);
        self.total_files_analyzed += 1;
    }

    pub fn record_directory_found(&mut self) {
        self.total_directories_scanned += 1;
    }

    pub fn record_file_found(&mut self) {
        self.total_files_scanned += 1;
    }

    pub fn record_skipped_file(&mut self) {
        self.total_files_skipped += 1
    }

    pub fn language_totals(&self) -> HashMap<Language, LanguageTotals> {
        let mut totals = HashMap::new();
        for file_report in &self.files {
            let entry = totals
                .entry(file_report.language)
                .or_insert(LanguageTotals::default());
            entry.files += 1;
            entry.lines += file_report.stats.total_lines;
            entry.blank_lines += file_report.stats.total_blank_lines;
            entry.non_blank_lines += file_report.stats.total_non_blank_lines;
            entry.characters += file_report.stats.total_characters;
            entry.functions += file_report.stats.total_functions;
            entry.todos += file_report.stats.total_todos;
        }
        totals
    }

    #[cfg(test)]
    pub fn total_files_found(&self) -> usize {
        self.total_files_scanned
    }

    #[cfg(test)]
    pub fn total_files_skipped(&self) -> usize {
        self.total_files_skipped
    }

    #[cfg(test)]
    pub fn total_files_analyzed(&self) -> usize {
        self.total_files_analyzed
    }

    #[cfg(test)]
    pub fn total_directories_found(&self) -> usize {
        self.total_directories_scanned
    }

    pub fn generate(&mut self) {
        self.total_files_analyzed = self.files.len();
        self.total_lines = 0;
        self.total_characters = 0;
        self.total_blank_lines = 0;
        self.total_non_blank_lines = 0;
        self.total_functions = 0;
        self.total_todos = 0;

        for file in &self.files {
            self.total_lines += file.stats.total_lines;
            self.total_characters += file.stats.total_characters;
            self.total_blank_lines += file.stats.total_blank_lines;
            self.total_non_blank_lines += file.stats.total_non_blank_lines;
            self.total_functions += file.stats.total_functions;
            self.total_todos += file.stats.total_todos;
        }
    }

    pub fn print(&self) {
        let mut languages = HashMap::new();
        for file in &self.files {
            *languages.entry(file.language).or_insert(0) += 1;
        }

        println!("----------");
        println!("|SourceAtlas|");
        println!("----------");
        println!("");
        println!("Languages used:");
        for (language, count) in languages {
            println!("{language}: {count} files");
        }
        println!("");

        println!("Language stats");
        let language_totals = self.language_totals();
        for (language, totals) in language_totals {
            println!("{}:", language);
            println!(" - Files: {}", totals.files);
            println!(" - Lines: {}", totals.lines);
            println!(" - Blank lines: {}", totals.blank_lines);
            println!(" - Non-blank lines: {}", totals.non_blank_lines);
            println!(" - Characters: {}", totals.characters);
            println!(" - Functions: {}", totals.functions);
            println!(" - TODOs: {}\n", totals.todos);
        }

        println!("DIRECTORIES");
        println!("--------");
        println!("total directories: {}", self.total_directories_scanned);
        println!("");

        // TODO: Decided if I can remove these in the future
        println!("FILES");
        println!("--------");
        println!("total files found: {}", self.total_files_scanned);
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

pub struct FileReport {
    language: Language,
    stats: FileStats,
}

impl FileReport {
    pub fn new(language: Language, stats: FileStats) -> Self {
        Self { language, stats }
    }
}

pub struct LanguageTotals {
    pub files: usize,
    pub lines: usize,
    pub blank_lines: usize,
    pub non_blank_lines: usize,
    pub characters: usize,
    pub functions: usize,
    pub todos: usize,
}

impl Default for LanguageTotals {
    fn default() -> Self {
        Self {
            files: 0,
            lines: 0,
            blank_lines: 0,
            non_blank_lines: 0,
            characters: 0,
            functions: 0,
            todos: 0,
        }
    }
}
