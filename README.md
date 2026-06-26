# repolens
A good CLI project for you would be repolens: a source-code repository inspector.

You point it at a project directory, and it reports useful information about the codebase:

$ repolens scan .

Repository: rhabit

Files scanned:       14
Source files:         9
Total lines:          428
Blank lines:          63
TODO comments:        4
Largest source file:  src/main.rs (173 lines)

Languages:
  Rust       361 lines
  TOML        42 lines
  Markdown    25 lines

Later, it could also find TODOs, show large files, generate a directory tree, detect common project files, and output JSON.

This is more interesting than another task manager because it interacts with real directories and source files. It also gives nearly every Rust Book concept a natural use.

What the finished tool could do

Possible commands:

repolens scan .
repolens todos .
repolens largest . --limit 10
repolens tree .
repolens languages .
repolens check .

Example TODO output:

$ repolens todos .

src/main.rs:42
  TODO: handle invalid user input

src/config.rs:18
  FIXME: validate empty command values

2 comments found

Example repository health output:

$ repolens check .

[OK] Cargo.toml found
[OK] README.md found
[OK] .gitignore found
[WARN] No LICENSE file found
[WARN] 3 Rust files contain unwrap()
[INFO] 4 TODO comments found

You should not try to build all of this immediately. Start very small and evolve it alongside the Rust Book.

Chapters 1–3: analyse one string

Do not scan directories yet.

Start with a hardcoded string containing source code:

let source = "
fn main() {
    // TODO: improve this
    println!(\"Hello\");
}
";

Your program should calculate:

total characters;
total lines;
blank lines;
non-blank lines;
number of occurrences of TODO;
number of occurrences of fn.

Example output:

Repository Lens

Characters:      69
Total lines:      6
Blank lines:      1
Non-blank lines:  5
TODO comments:    1
Functions:        1
Concepts used

This applies:String::from(num)

variables;
mutability;
functions;
loops;
conditionals;
string methods;
numeric types;
arrays;
basic command-line arguments later in the phase.
First functions

Build these individually:

fn count_lines(source: &str) -> usize {
    source.lines().count()
}
fn count_blank_lines(source: &str) -> usize {
    let mut count = 0;

    for line in source.lines() {
        if line.trim().is_empty() {
            count += 1;
        }
    }

    count
}
fn count_occurrences(source: &str, search_term: &str) -> usize {
    source.matches(search_term).count()
}
fn print_report(
    character_count: usize,
    line_count: usize,
    blank_line_count: usize,
    todo_count: usize,
) {
    // Print formatted report.
}

Do not worry if methods such as .lines() and .matches() involve iterators you have not studied yet. You can use them as library operations now and understand their iterator behaviour later.

First real milestone: analyse a file

Once the hardcoded source works, accept a filename:

repolens src/main.rs

Initially, you can use:

let args: Vec<String> = std::env::args().collect();

Then read the file:

let contents = std::fs::read_to_string(&args[1])
    .expect("Failed to read file");

Output:

File: src/main.rs

Characters:      3,442
Total lines:      127
Blank lines:      18
TODO comments:    2

At this stage, panicking with expect is acceptable. Chapter 9 will replace it with proper error handling.

Requirements for version 0.1

The program must:

Accept exactly one path.
Show an error when no path is provided.
Read the file.
Count total lines.
Count blank lines.
Count non-blank lines.
Count characters.
Count TODO and FIXME.
Print a formatted report.

Do not scan entire directories yet.

Chapter 4: ownership and borrowing

Repository analysis provides a clear ownership exercise.

Consider:

fn analyse(contents: String) {
    println!("{}", contents.lines().count());
}

Calling this moves the String into the function:

let contents = std::fs::read_to_string(path).unwrap();

analyse(contents);

println!("{contents}"); // Error: contents was moved

Refactor it to borrow:

fn analyse(contents: &str) {
    println!("{}", contents.lines().count());
}

Now the caller retains ownership:

analyse(&contents);
println!("{contents}");
Chapter 4 exercises

Create:

fn first_non_empty_line(source: &str) -> Option<&str>
fn longest_line(source: &str) -> Option<&str>
fn normalise_path(path: &str) -> String
fn add_warning(report: &mut String, warning: &str)

These cover:

&str;
returning slices;
owned String values;
immutable borrowing;
mutable borrowing.
Important ownership decision

The file contents should normally be owned by the function that reads the file:

let contents: String = std::fs::read_to_string(path)?;

Analysis functions should usually borrow it:

fn count_lines(contents: &str) -> usize

That is a realistic ownership model:

reading creates owned data;
analysing temporarily borrows it.
Chapters 5–6: model the report

Replace separate variables with a struct:

#[derive(Debug)]
struct FileReport {
    path: String,
    character_count: usize,
    total_lines: usize,
    blank_lines: usize,
    code_lines: usize,
    todo_count: usize,
    fixme_count: usize,
}

Add methods:

impl FileReport {
    fn analyse(path: String, contents: &str) -> Self {
        // ...
    }

    fn print(&self) {
        // ...
    }

    fn comment_count(&self) -> usize {
        self.todo_count + self.fixme_count
    }
}
Detect file types with an enum
#[derive(Debug, PartialEq)]
enum Language {
    Rust,
    JavaScript,
    TypeScript,
    Python,
    CSharp,
    C,
    Markdown,
    Toml,
    Unknown,
}

Detect the language from the extension:

fn detect_language(path: &str) -> Language {
    if path.ends_with(".rs") {
        Language::Rust
    } else if path.ends_with(".js") {
        Language::JavaScript
    } else if path.ends_with(".ts") {
        Language::TypeScript
    } else if path.ends_with(".py") {
        Language::Python
    } else {
        Language::Unknown
    }
}

Later, use match:

fn comment_prefix(language: &Language) -> Option<&'static str> {
    match language {
        Language::Rust
        | Language::JavaScript
        | Language::TypeScript
        | Language::C
        | Language::CSharp => Some("//"),

        Language::Python => Some("#"),

        Language::Markdown
        | Language::Toml
        | Language::Unknown => None,
    }
}

This gives Option a legitimate use: not every known file type necessarily has a comment syntax you support.

Add issue types
#[derive(Debug)]
enum MarkerType {
    Todo,
    Fixme,
    Hack,
    Note,
}

And:

#[derive(Debug)]
struct Marker {
    marker_type: MarkerType,
    line_number: usize,
    text: String,
}

Example:

src/main.rs:42 [TODO] Handle invalid choices
Version 0.2 requirements

Your tool should now:

detect the file’s language;
return a FileReport;
store found markers in Vec<Marker>;
show line numbers;
use enums instead of arbitrary strings where appropriate;
use methods for report behaviour.
Chapter 7: modules

Split the project:

src/
├── main.rs
├── analysis.rs
├── language.rs
├── marker.rs
└── report.rs

Responsibilities:

main.rs       Reads arguments and controls program flow
analysis.rs   Analyses file content
language.rs   Language detection
marker.rs     TODO/FIXME marker types and parsing
report.rs     FileReport and formatting

An example module API:

pub fn analyse_file(path: &str) -> Result<FileReport, std::io::Error> {
    // ...
}

Keep fields private where possible:

pub struct FileReport {
    path: String,
    language: Language,
    total_lines: usize,
    markers: Vec<Marker>,
}

Expose behaviour through methods:

impl FileReport {
    pub fn total_lines(&self) -> usize {
        self.total_lines
    }

    pub fn markers(&self) -> &[Marker] {
        &self.markers
    }

    pub fn print(&self) {
        // ...
    }
}

The method:

pub fn markers(&self) -> &[Marker]

is especially useful ownership practice. It lets callers inspect markers without giving ownership of the vector away.

Chapter 8: scan whole repositories

This is where repolens becomes a genuine repository tool.

Recursively inspect a directory and collect supported files.

Initially, use the standard library:

std::fs::read_dir(path)

Create:

fn collect_files(path: &Path) -> Result<Vec<PathBuf>, std::io::Error>

You will encounter:

Path;
PathBuf;
directory entries;
vectors;
recursive functions.
Repository report
struct RepositoryReport {
    root: PathBuf,
    files: Vec<FileReport>,
}

Methods:

impl RepositoryReport {
    fn total_files(&self) -> usize {
        self.files.len()
    }

    fn total_lines(&self) -> usize {
        let mut total = 0;

        for report in &self.files {
            total += report.total_lines();
        }

        total
    }

    fn total_markers(&self) -> usize {
        // ...
    }
}
Language statistics with HashMap
use std::collections::HashMap;

let mut line_counts: HashMap<Language, usize> = HashMap::new();

For each file:

let entry = line_counts
    .entry(report.language())
    .or_insert(0);

*entry += report.total_lines();

To use Language as a key, derive:

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Language {
    // ...
}

This provides a concrete reason for Eq and Hash.

Files and directories to ignore

Skip common directories:

.git
target
node_modules
dist
build
.idea
.vscode

Start with an array:

const IGNORED_DIRECTORIES: [&str; 7] = [
    ".git",
    "target",
    "node_modules",
    "dist",
    "build",
    ".idea",
    ".vscode",
];

Add:

fn should_ignore(path: &Path) -> bool
Version 0.3 commands

At this point, support:

repolens scan .
repolens todos .
repolens largest .
repolens languages .

You can initially parse commands manually:

match args[1].as_str() {
    "scan" => {
        // ...
    }
    "todos" => {
        // ...
    }
    "largest" => {
        // ...
    }
    "languages" => {
        // ...
    }
    _ => {
        eprintln!("Unknown command");
    }
}

Do not introduce Clap yet unless you specifically want to study an external CLI library. Manual parsing teaches the underlying design first.

Chapter 9: proper errors

Directory tools can fail in several ways:

path does not exist;
path is not readable;
file is not valid UTF-8;
command is missing;
command is unknown;
option value is invalid.

Create custom errors:

#[derive(Debug)]
enum CliError {
    MissingCommand,
    UnknownCommand(String),
    MissingPath,
    InvalidLimit(String),
}
#[derive(Debug)]
enum ScanError {
    PathNotFound(PathBuf),
    CannotReadDirectory {
        path: PathBuf,
        source: std::io::Error,
    },
    CannotReadFile {
        path: PathBuf,
        source: std::io::Error,
    },
}

Your main function can return:

fn main() -> Result<(), Box<dyn std::error::Error>> {
    run()
}

Or:

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {error}");
        std::process::exit(1);
    }
}

Then:

fn run() -> Result<(), Box<dyn std::error::Error>> {
    // ...
    Ok(())
}
Do not allow one unreadable file to destroy the entire scan

This is an important design decision.

You can distinguish between fatal and non-fatal failures:

root directory cannot be opened: fatal;
one nested file cannot be read: warning;
unsupported binary file: skip;
malformed option: fatal.

Store warnings:

struct ScanWarning {
    path: PathBuf,
    message: String,
}

Then show:

Scan completed with 2 warnings.

WARN: could not read private/secrets.txt
WARN: skipped binary file assets/logo.png
Chapters 10–11: traits and testing
Output format trait

Create:

trait ReportFormatter {
    fn format(&self, report: &RepositoryReport) -> String;
}

Implement:

struct TextFormatter;
struct CompactFormatter;
struct JsonFormatter;

For now, JSON can be handcrafted as an exercise. Later, use serde_json.

Example:

impl ReportFormatter for TextFormatter {
    fn format(&self, report: &RepositoryReport) -> String {
        // ...
    }
}

This creates a real reason for traits: multiple output strategies share one interface.

Tests

Test isolated behaviour.

Language detection
#[test]
fn detects_rust_files() {
    assert_eq!(
        detect_language(Path::new("src/main.rs")),
        Language::Rust
    );
}
Marker detection
#[test]
fn finds_todo_with_line_number() {
    let source = "fn main() {\n    // TODO: improve this\n}\n";

    let markers = find_markers(source);

    assert_eq!(markers.len(), 1);
    assert_eq!(markers[0].line_number(), 2);
}
Blank lines
#[test]
fn counts_blank_lines() {
    let source = "one\n\nthree\n";

    assert_eq!(count_blank_lines(source), 1);
}
Ignore logic
#[test]
fn ignores_target_directory() {
    let path = Path::new("./target/debug/app");

    assert!(should_ignore(path));
}
Repository integration test

Create a temporary test directory:

test-project/
├── src/
│   └── main.rs
└── README.md

Scan it and verify:

two files are found;
Rust is detected;
Markdown is detected;
expected line totals are produced.
Chapter 12: Minigrep integration

The Minigrep chapter fits naturally into this tool.

Add:

repolens search . unwrap

Output:

src/main.rs:14
    let config = Config::parse(&args).unwrap();

src/storage.rs:31
    let json = serde_json::to_string(tasks).unwrap();

2 matches

This uses the same search concepts as Minigrep but applies them across a repository.

Useful flags later:

repolens search . unwrap --ignore-case
repolens search . "Box<dyn Error>" --extension rs
repolens search . TODO --count
Chapter 13: iterators

Your first implementation will likely contain manual accumulation:

let mut total = 0;

for report in &repository.files {
    total += report.total_lines();
}

Refactor:

let total: usize = repository
    .files
    .iter()
    .map(|report| report.total_lines())
    .sum();

Find Rust files:

let rust_files: Vec<&FileReport> = repository
    .files
    .iter()
    .filter(|report| report.language() == Language::Rust)
    .collect();

Find the largest file:

let largest = repository
    .files
    .iter()
    .max_by_key(|report| report.total_lines());

Sort files:

let mut files: Vec<&FileReport> = repository.files.iter().collect();

files.sort_by_key(|report| std::cmp::Reverse(report.total_lines()));

Find all TODO markers:

let todos: Vec<&Marker> = repository
    .files
    .iter()
    .flat_map(|report| report.markers())
    .filter(|marker| marker.marker_type() == MarkerType::Todo)
    .collect();

This project gives iterators a clear purpose.

Later improvements with crates

After you understand the standard-library implementation, introduce ecosystem crates.

Clap

Use Clap for proper command parsing:

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}
#[derive(Subcommand)]
enum Command {
    Scan {
        path: PathBuf,
    },
    Todos {
        path: PathBuf,
    },
    Largest {
        path: PathBuf,

        #[arg(short, long, default_value_t = 10)]
        limit: usize,
    },
}

Do not begin with Clap. First build one or two commands manually so you understand what the library is replacing.

Walkdir

Replace hand-written recursive traversal with the walkdir crate.

That gives you an opportunity to compare:

your recursive standard-library implementation;
a mature abstraction;
handling errors through iterator items.
Ignore

The ignore crate can respect:

.gitignore
.ignore
.git/info/exclude

This would make repolens behave more like a professional source-code tool.

Serde

Add JSON output:

repolens scan . --format json

Example:

{
  "files": 14,
  "lines": 428,
  "languages": {
    "Rust": 361,
    "TOML": 42,
    "Markdown": 25
  }
}
Advanced milestones
Directory tree
$ repolens tree .

rhabit/
├── Cargo.toml
├── README.md
└── src/
    ├── command.rs
    ├── config.rs
    └── main.rs

You can represent it recursively:

enum TreeNode {
    File {
        name: String,
    },
    Directory {
        name: String,
        children: Vec<TreeNode>,
    },
}

That connects naturally to the smart-pointer and recursive-type chapters.

Repository rules

Allow a configuration file:

[scan]
ignore = ["vendor", "generated"]
extensions = ["rs", "toml", "md"]

[rules]
maximum_file_lines = 300
forbid_unwrap = true
require_readme = true
require_license = false

Then:

$ repolens check .

PASS  README.md exists
WARN  LICENSE file is missing
FAIL  src/main.rs has 412 lines; maximum is 300
FAIL  3 occurrences of unwrap() found

Repository health: 65/100

This could eventually be used in CI:

repolens check . --fail-on-warning

The process exit code could communicate success or failure:

std::process::exit(1);

That makes the project more than a learning exercise. It becomes an actual development tool.

Parallel scanning

When you reach concurrency, analyse multiple files in parallel.

Possible structure:

main thread traverses files;
worker threads analyse file contents;
workers send FileReport values through a channel;
main thread collects results.

This is a legitimate concurrency use case because repositories can contain many independent files.

Recommended implementation order

Build it as releases rather than trying to match every single chapter precisely.

0.1.0 — single-file analyser
repolens src/main.rs

Features:

line count;
blank line count;
character count;
TODO/FIXME count;
language detection.

Relevant chapters: 1–6.

0.2.0 — repository scanner
repolens scan .

Features:

recursive directory traversal;
ignored directories;
Vec<FileReport>;
language totals;
largest file.

Relevant chapters: 7–9.

0.3.0 — commands and tests
repolens todos .
repolens languages .
repolens largest . --limit 5

Features:

command enum;
custom errors;
unit tests;
integration tests;
library and binary separation.

Relevant chapters: 10–12.

0.4.0 — polished CLI

Features:

Clap;
JSON output;
.gitignore support;
configuration file;
repository health checks;
meaningful exit codes.

Relevant chapters: 13–14 and ecosystem practice.

1.0.0 — usable developer tool

Features:

stable commands;
clear documentation;
installation with cargo install;
Linux, macOS, and Windows support;
parallel scanning;
reliable test suite.
Your immediate assignment

Start only with this:

repolens <file>

It must print:

File report
-----------

Path:            src/main.rs
Language:        Rust
Characters:      3421
Lines:           127
Blank lines:     18
Non-blank lines: 109
TODO markers:    2
FIXME markers:   1

Use this initial structure:

use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: repolens <file>");
        return;
    }

    let path = &args[1];

    let contents = fs::read_to_string(path)
        .expect("Failed to read file");

    let total_lines = count_lines(&contents);
    let blank_lines = count_blank_lines(&contents);
    let todo_count = count_occurrences(&contents, "TODO");
    let fixme_count = count_occurrences(&contents, "FIXME");

    println!("File report");
    println!("-----------");
    println!();
    println!("Path:            {path}");
    println!("Characters:      {}", contents.chars().count());
    println!("Lines:           {total_lines}");
    println!("Blank lines:     {blank_lines}");
    println!("Non-blank lines: {}", total_lines - blank_lines);
    println!("TODO markers:    {todo_count}");
    println!("FIXME markers:   {fixme_count}");
}

fn count_lines(source: &str) -> usize {
    source.lines().count()
}

fn count_blank_lines(source: &str) -> usize {
    let mut count = 0;

    for line in source.lines() {
        if line.trim().is_empty() {
            count += 1;
        }
    }

    count
}

fn count_occurrences(source: &str, search_term: &str) -> usize {
    source.matches(search_term).count()
}

Do not copy the entire final architecture upfront. Build this first, then improve it when the current Rust Book chapter gives you a reason to improve it. RepoLens is broad enough to follow you through most of the book without turning into an artificial collection of unrelated features.