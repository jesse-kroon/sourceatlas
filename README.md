# SourceAtlas

SourceAtlas is a command-line tool written in Rust that scans a source code repository and generates useful statistics.

I started this project as a way to learn Rust by building something practical instead of only completing exercises. The goal isn't to compete with existing tools, but to build a clean, well-structured application while learning more about idiomatic Rust and software engineering.

## Features

Current features include:

- Recursive directory scanning
- Ignores common build and IDE directories:
  - `target`
  - `build`
  - `node_modules`
  - `.idea`
  - `.vscode`
- Reports:
  - Directories scanned
  - Files scanned
  - Files analyzed
  - Files skipped
  - Total lines
  - Blank lines
  - Non-blank lines
  - Characters
  - Functions (currently Rust only)
  - TODO comments
- Gracefully skips unreadable or binary files
- Unit tested

## Installation

Install from crates.io:

```bash
cargo install sourceatlas
```

Or build from source:

```bash
git clone https://github.com/jesse-kroon/sourceatlas.git
cd sourceatlas
cargo build --release
```

## Usage

Scan the current directory:

```bash
sourceatlas scan .
```

Scan another directory:

```bash
sourceatlas scan /path/to/project
```

## Example Output

```text
----------
|SourceAtlas|
----------

Languages used:
Rust: 10 files

Language stats
Rust:
 - Files: 10
 - Lines: 562
 - Blank lines: 90
 - Non-blank lines: 472
 - Characters: 11153
 - Functions: 24
 - TODOs: 2

DIRECTORIES
--------
total directories: 105

FILES
--------
total files found: 160
total files analyzed: 10
total files skipped: 150
total lines: 562
total characters: 11153
total non-blank lines: 472
total blank lines: 90
total functions: 24
total TODO's: 2
```

*The above output is generated when running sourceatlas on itself*

## Roadmap

Some features I'd like to add:

- More visually appealing report generation
- Support for more languages
- Custom ignore files
- JSON output
- Markdown reports
- Better language detection
- Parallel directory scanning
- Additional repository metrics

## License

This project is licensed under the MIT License.
