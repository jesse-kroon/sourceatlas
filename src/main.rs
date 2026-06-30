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
        let entry = match entry {
            Ok(entry) => entry,
            Err(err) => {
                eprintln!("Skipping entry in {}: {err}", directory.display());
                continue;
            }
        };

        let path = entry.path();
        if path.is_dir() {
            if should_ignore_directory(&path) {
                continue;
            }

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

fn should_ignore_directory(path: &Path) -> bool {
    let skippable_directories = [".idea", ".vscode", "build", "target", "node_modules"];
    let Some(directory_name) = path.file_name().and_then(|name| name.to_str()) else {
        return false;
    };

    skippable_directories.contains(&directory_name)
}

#[cfg(test)]
mod tests {
    use std::{
        fs::{self},
        path::Path,
    };

    use tempfile::tempdir;

    use crate::{report::Report, scan_directory};

    #[test]
    fn binary_files_are_skipped_but_counted() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path();
        fs::write(dir_path.join("image1.png"), [0xFF, 0xD7, 0xFF]).unwrap();
        fs::write(dir_path.join("image2.png"), [0xFF, 0xD7, 0xFF]).unwrap();
        fs::write(dir_path.join("image3.png"), [0xFF, 0xD7, 0xFF]).unwrap();

        let mut report = Report::new();
        scan_directory(Path::new(dir_path), &mut report);
        report.generate();

        assert_eq!(report.total_files_found(), 3);
        assert_eq!(report.total_files_skipped(), 3);
        assert_eq!(report.total_files_analyzed(), 0);
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

        assert_eq!(report.total_files_found(), 3);
        assert_eq!(report.total_files_analyzed(), 2);
        assert_eq!(report.total_files_skipped(), 1);

        fs::remove_dir_all(dir_path).unwrap();
    }

    #[test]
    fn ignores_skippable_directories() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path();

        let test_directories = vec![".idea", ".vscode", "target", "build", "node_modules"];
        for directory in test_directories {
            let fake_ide_dir = dir_path.join(directory);
            fs::create_dir(&fake_ide_dir).unwrap();
        }

        let lib_dir = dir_path.join("lib");
        fs::create_dir(&lib_dir).unwrap();

        let mut report = Report::new();
        scan_directory(Path::new(dir_path), &mut report);
        report.generate();

        assert_eq!(report.total_directories_found(), 1);
    }
}
