use std::{fs, path::Path};

use crate::{
    file_stats::FileStats,
    language::Language,
    report::{FileReport, Report},
};

pub struct Scanner {}

impl Scanner {
    pub fn scan(directory: &Path, report: &mut Report) {
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
                Self::scan(&path, report);
                continue;
            }

            if path.is_file() {
                report.record_file_found();
                match fs::read_to_string(&path) {
                    Ok(source) => {
                        let Some(language) = Language::from_path(&path) else {
                            report.record_skipped_file();
                            continue;
                        };

                        let parser = language.parser();
                        let stats = FileStats::new(&source, parser);
                        report.add_file(FileReport::new(language, stats));
                    }
                    Err(_) => {
                        report.record_skipped_file();
                        continue;
                    }
                };
            }
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
