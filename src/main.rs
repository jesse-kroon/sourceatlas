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
    path::Path,
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
        Command::Scan { directory } => {
            let mut report = Report::new();
            scan_directory(Path::new(&directory), &mut report);
            report.generate();
            report.print();
        }
        Command::Help => println!("You won't get any help here"),
    }
}

fn scan_directory(directory: &Path, report: &mut Report) {
    println!("Scanning {}", directory.display());

    let entries = match fs::read_dir(directory) {
        Ok(entries) => entries,
        Err(err) => {
            eprintln!("Skipping directory {}: {err}", directory.display());
            return;
        }
    };

    for entry in entries {
        let entry = entry.unwrap_or_else(|err| {
            eprintln!("{err}");
            process::exit(1);
        });

        let path = entry.path();
        if path.is_dir() {
            report.add_directory();
            scan_directory(&path, report);
            continue;
        }

        if path.is_file() {
            let source = match fs::read_to_string(&path) {
                Ok(file) => file,
                Err(err) => {
                    eprintln!("Skipping {}: {err}", path.display());
                    continue;
                }
            };
            report.add_file(FileStats::new(&source));
        }

        report.generate();
        report.print();
    }
}
