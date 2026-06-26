mod command;
mod config;
mod file_stats;
mod report;

use config::Config;
use file_stats::FileStats;
use report::Report;
use std::{
    env,
    fs::{self},
    process,
};

use crate::command::Command;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::parse(&args).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1)
    });

    match config.command {
        Command::Scan => scan_directory(&config.file_path),
    }
}

fn scan_directory(directory: &str) {
    println!("Scanning {}", directory);
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
        let file = fs::read_to_string(file_path).unwrap_or_else(|err| {
            eprintln!("{err}");
            process::exit(1)
        });

        report.add_file(FileStats::new(&file));
    }

    report.generate();
    report.print();
}
