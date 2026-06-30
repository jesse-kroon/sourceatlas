mod command;
mod config;
mod file_stats;
mod report;
mod scanner;

use config::Config;
use report::Report;
use std::{env, path::Path, process};

use crate::{command::Command, scanner::Scanner};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::parse(&args).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1)
    });

    match config.command {
        Command::Scan { directory } => {
            let mut report = Report::new();
            Scanner::scan(Path::new(&directory), &mut report);
            report.generate();
            report.print();
        }
        Command::Help => println!("You won't get any help here"),
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs::{self},
        path::Path,
    };

    use tempfile::tempdir;

    use crate::{report::Report, scanner::Scanner};

    #[test]
    fn binary_files_are_skipped_but_counted() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path();
        fs::write(dir_path.join("image1.png"), [0xFF, 0xD7, 0xFF]).unwrap();
        fs::write(dir_path.join("image2.png"), [0xFF, 0xD7, 0xFF]).unwrap();
        fs::write(dir_path.join("image3.png"), [0xFF, 0xD7, 0xFF]).unwrap();

        let mut report = Report::new();
        Scanner::scan(Path::new(dir_path), &mut report);
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
        Scanner::scan(Path::new(dir_path), &mut report);
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
        Scanner::scan(Path::new(dir_path), &mut report);
        // report.generate();

        assert_eq!(report.total_directories_found(), 1);
    }
}
