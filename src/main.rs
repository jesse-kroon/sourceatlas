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
            report.record_directory_found();
            scan_directory(&path, report);
            continue;
        }

        if path.is_file() {
            report.record_file_found();
            match fs::read_to_string(&path) {
                Ok(source) => report.add_file(FileStats::new(&source)),
                Err(_) => {
                    report.record_skipped_file();
                    continue;
                }
            };
        }
    }
}
#[cfg(test)]
mod tests {
    use std::{
        fs::{self, DirBuilder},
        path::Path,
    };

    use tempfile::tempdir;

    use crate::{report::Report, scan_directory};

    #[test]
    fn binary_file_is_skipped_but_is_counted() {
        let dir_path = "./tmp/";
        let mut builder = DirBuilder::new();
        builder.recursive(true).create(dir_path).unwrap();
        fs::write(dir_path.to_owned() + "/image1.png", [0xFF, 0xD7, 0xFF]).unwrap();
        fs::write(dir_path.to_owned() + "/image2.png", [0xFF, 0xD7, 0xFF]).unwrap();
        fs::write(dir_path.to_owned() + "/image3.png", [0xFF, 0xD7, 0xFF]).unwrap();

        let mut report = Report::new();
        scan_directory(Path::new(dir_path), &mut report);
        report.generate();

        assert_eq!(report.total_files(), 3);
        assert_eq!(report.total_files_skipped(), 3);
        assert_eq!(report.total_files_analyzed(), 0);

        fs::remove_dir_all(dir_path).unwrap();
    }

    #[test]
    fn recursively_scans_directories() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path();

        let lib_dir = dir_path.join("lib");
        fs::create_dir(&lib_dir).unwrap();

        fs::write(dir_path.join("main.rs"), "fn main() {}").unwrap();
        fs::write(dir_path.join("image1.png"), [0xFF, 0xDD, 0xFF]).unwrap();
        fs::write(lib_dir.join("main.rs"), "fn main() {}").unwrap();

        let mut report = Report::new();
        scan_directory(Path::new(dir_path), &mut report);
        report.generate();

        assert_eq!(report.total_files(), 3);
        assert_eq!(report.total_files_analyzed(), 2);
        assert_eq!(report.total_files_skipped(), 1);

        fs::remove_dir_all(dir_path).unwrap();
    }
}
