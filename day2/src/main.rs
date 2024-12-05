use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn is_safe(vec: &[u32]) -> bool {
    (vec.is_sorted_by(|a, b| a < b) || vec.is_sorted_by(|a, b| a > b))
        && vec
            .windows(2)
            .all(|w| (1..4).contains(&w[0].abs_diff(w[1])))
}

fn is_safe_when_damped(vec: &[u32]) -> bool {
    let errors: Vec<bool> = vec.windows(3).map(|w| is_safe(&w)).collect();

    if errors.clone().into_iter().filter(|b| !*b).count() > 3 {
        return false;
    }

    if (!errors[0] && is_safe(&vec[1..]))
        || (!errors[errors.len() - 1] && is_safe(&vec[..(vec.len() - 1)]))
    {
        return true;
    }

    let error_positions = errors
        .iter()
        .enumerate()
        .filter(|(_, t)| !**t)
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();

    for pos in error_positions {
        if is_safe(&[&vec[..pos + 1], &vec[pos + 2..]].concat()) {
            return true;
        }
    }

    false
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let path = Path::new(filename);
    let file = match File::open(path) {
        Err(why) => panic!("Couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };
    Ok(io::BufReader::new(file).lines())
}

fn split_line(line: String) -> Vec<u32> {
    line.split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
}

/// Solve day 2 of Advent of Code 2024
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The input file.
    #[arg(short, long)]
    path: String,
}

fn main() {
    let args = Args::parse();
    let input: Vec<Vec<u32>> = read_lines(&args.path)
        .unwrap()
        .map(|line| split_line(line.unwrap()))
        .collect();

    let result1: u32 = input
        .clone()
        .into_iter()
        .map(|v| match is_safe(&v) {
            true => 1,
            false => 0,
        })
        .sum();

    let result2: u32 = input
        .clone()
        .into_iter()
        .map(|v| match is_safe(&v) || is_safe_when_damped(&v) {
            true => 1,
            false => 0,
        })
        .sum();

    println!(
        "Result of task 1 is {}, should be 379 for my input.txt.",
        result1
    );
    println!(
        "Result of task 2 is {}, should be 430 for my input.txt.",
        result2
    );
}
