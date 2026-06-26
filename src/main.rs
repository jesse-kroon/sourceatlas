mod file_stats;
mod report;

use file_stats::FileStats;
use report::Report;
use std::{
    env,
    fs::{self},
    process,
};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: repolens <command> <directory>");
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

        report.add_file(FileStats::new(&source));
    }

    report.generate();
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
