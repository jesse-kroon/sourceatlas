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
|sourceatlas|
----------

DIRECTORIES
--------
total directories: 55

FILES
--------
total files found: 86
total files analyzed: 43
total files skipped: 43
total lines: 1607
total characters: 40438
total non-blank lines: 1372
total blank lines: 235
total functions: 16
total TODO's: 26
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

## Why SourceAtlas?

The idea behind the name is simple: rather than just counting lines of code, SourceAtlas aims to provide an overview of a source tree—almost like a map of a repository.

## License

This project is licensed under the MIT License.