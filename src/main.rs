use std::{
    env,
    error::Error,
    fs::{self, ReadDir},
    ops::Add,
    process,
};

struct File {
    total_lines: usize,
    total_characters: usize,
    total_blank_lines: usize,
    total_non_blank_lines: usize,
    total_functions: usize,
    total_todos: usize,
}

impl File {
    fn new(source: &str) -> Self {
        File {
            total_lines: count_file_lines(source),
            total_characters: count_file_characters(source),
            total_blank_lines: count_file_blank_lines(source),
            total_non_blank_lines: count_file_non_blank_lines(source),
            total_functions: count_file_functions(source),
            total_todos: count_file_todos(source),
        }
    }
}

struct Report {
    files: Vec<File>,
    total_files: usize,
    total_lines: usize,
    total_characters: usize,
    total_blank_lines: usize,
    total_non_blank_lines: usize,
    total_functions: usize,
    total_todos: usize,
}

impl Report {
    fn new() -> Self {
        Report {
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

    fn generate_report(mut self) -> Self {
        self.total_files += self.files.iter().count();

        for file in &self.files {
            self.total_lines += file.total_lines;
            self.total_characters += file.total_characters;
            self.total_blank_lines += file.total_blank_lines;
            self.total_non_blank_lines += file.total_non_blank_lines;
            self.total_functions += file.total_functions;
            self.total_todos += file.total_todos;
        }

        self
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("too few arguments");
        process::exit(1)
    }

    let _command = &args[1];
    let directory = &args[2];

    let mut report = Report::new();

    let entries = fs::read_dir(directory).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1)
    });

    for entry in entries {
        let entry = entry.unwrap_or_else(|err| {
            eprintln!("{err}");
            process::exit(1);
        });
        let file_path = entry.path();
        let source = fs::read_to_string(file_path).unwrap_or_else(|err| {
            eprintln!("{err}");
            process::exit(1)
        });

        report.files.push(File::new(&source));
    }

    let report = report.generate_report();
    print_report(&report);
}

fn print_report(report: &Report) {
    println!("REPOLENS");
    println!("--------");
    println!("total files: {}", report.total_files);
    println!("total lines: {}", report.total_lines);
    println!("total characters: {}", report.total_characters);
    println!("total non-blank lines: {}", report.total_non_blank_lines);
    println!("total blank lines: {}", report.total_blank_lines);
    println!("total functions: {}", report.total_functions);
    println!("total TODO's: {}", report.total_todos);
}

fn count_file_lines(source: &str) -> usize {
    source.lines().count()
}

fn count_file_characters(source: &str) -> usize {
    source.chars().filter(|char| !char.is_whitespace()).count()
}

fn count_file_non_blank_lines(source: &str) -> usize {
    source.lines().filter(|line| !line.is_empty()).count()
}

fn count_file_blank_lines(source: &str) -> usize {
    source.lines().filter(|line| line.is_empty()).count()
}

fn count_file_functions(source: &str) -> usize {
    source.lines().filter(|line| line.contains("fn")).count()
}

fn count_file_todos(source: &str) -> usize {
    source
        .lines()
        .filter(|line| line.to_lowercase().contains("todo"))
        .count()
}
