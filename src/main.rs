use clap::Parser;
use core::fmt;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader, IsTerminal, Read};
use std::path::Path;

/// Redact the sensitive contents of a file / stdin
#[derive(Parser, Debug)]
struct Args {
    /// The text to appear in the output in place of redacted values
    #[arg(short, long, default_value = "<REDACTED>")]
    mask: String,

    /// The input file containing patterns to redact
    #[arg(short, long, default_value = ".redact")]
    rules: String,

    /// The file to process
    input: Option<String>,
}

fn load_patterns<P>(filename: P) -> Result<Vec<Regex>, std::io::Error>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let results = BufReader::new(file)
        .lines()
        .map(|line| Regex::new(line.unwrap().as_str()).unwrap())
        .collect();
    Ok(results)
}

fn redact(line: &str, patterns: &Vec<Regex>, mask: &String) -> String {
    for pattern in patterns {
        if let Some(caps) = pattern.captures(line) {
            let mut result = String::new();
            let mut index = 0;
            for i in 1..caps.len() {
                let cap = caps.get(i).unwrap();
                result += &line[index..cap.start()];
                result += mask;
                index = cap.end();
            }
            return result;
        }
    }
    line.to_string()
}

fn process<S>(source: S, patterns: &Vec<Regex>, mask: &String) -> std::io::Result<()>
where
    S: Read,
{
    let reader = BufReader::new(source);
    for line in reader.lines() {
        println!("{}", redact(&line.unwrap(), &patterns, mask));
    }
    Ok(())
}

enum AppError {
    NoInput,
    NoRules,
    Unprocessable,
}

impl std::fmt::Debug for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoInput => write!(f, "No input found"),
            Self::NoRules => write!(f, "No rules defined"),
            Self::Unprocessable => write!(f, "Input is unprocessable"),
        }
    }
}

fn main() -> Result<(), AppError> {
    let args = Args::parse();

    let patterns = load_patterns(Path::new(args.rules.as_str()))
        .or(load_patterns(
            home::home_dir().unwrap().join(args.rules.as_str()),
        ))
        .map_err(|_| AppError::NoRules)?;

    if let Some(input) = args.input {
        let file = File::open(input).map_err(|_| AppError::NoInput)?;
        process(file, &patterns, &args.mask).map_err(|_| AppError::Unprocessable)
    } else {
        let input = std::io::stdin();
        if input.is_terminal() {
            Err(AppError::NoInput)
        } else {
            process(input, &patterns, &args.mask).map_err(|_| AppError::Unprocessable)
        }
    }
}
