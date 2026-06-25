fn main() {
    let source = "
        fn main() {
            // TODO: improve this.
            println!(\"Hello\");
        }
    ";

    println!("Repo Lens report");
    println!("----------------");
    println!("total lines: {}", count_file_lines(source));
    println!("total characters: {}", count_file_characters(source));
    println!(
        "total non-blank lines: {}",
        count_file_non_blank_lines(source)
    );
    println!("total blank lines: {}", count_file_blank_lines(source));
    println!("total functions: {}", count_file_functions(source));
    println!("total TODO's: {}", count_file_todos(source));
}

fn count_file_lines(source: &str) -> usize {
    source.lines().count()
}

fn count_file_characters(source: &str) -> usize {
    source.chars().filter(|char| char != &' ').count()
}

fn count_file_blank_lines(source: &str) -> usize {
    source.lines().filter(|line| line.trim().len() == 0).count()
}

fn count_file_non_blank_lines(source: &str) -> usize {
    count_file_lines(source) - count_file_blank_lines(source)
}

fn count_file_functions(source: &str) -> usize {
    source.lines().filter(|line| line.contains("fn")).count()
}

fn count_file_todos(source: &str) -> usize {
    source
        .lines()
        .filter(|line| line.to_lowercase().contains("todo"))
        .count()
}
