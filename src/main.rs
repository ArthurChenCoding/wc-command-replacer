use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];

    match analyze_file(file_path) {
        Ok((lines, words, characters)) => {
            println!("{} {} {} {}", lines, words, characters, file_path);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn analyze_file(file_path: &str) -> io::Result<(usize, usize, usize)> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut lines = 0;
    let mut words = 0;
    let mut characters = 0;

    for line_result in reader.lines() {
        let line = line_result?;
        lines += 1;
        characters += line.chars().count();
        words += line.split_whitespace().count();
    }

    Ok((lines, words, characters))
}
