use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let _command = &args[1];
    let directory = &args[2];

    for file in fs::read_dir(directory).unwrap() {
        let file = file.unwrap();
        let file_path = file.path();

        let source = fs::read_to_string(file_path).expect("cannot read file");

        print_report(source.as_str());
    }
}

fn print_report(source: &str) {
    println!("REPOLENS");
    println!("--------");
    println!("total lines: {}", count_file_lines(source));
    println!("total characters: {}", count_file_characters(source));
    println!(
        "total non-blank lines: {}",
        count_file_non_blank_lines(source)
    );
    println!("total blank lines: {}", count_file_blank_lines(source));
    println!("total functions: {}", count_file_functions(source));
    println!("total TODO's: {}", count_file_todos(source));
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
