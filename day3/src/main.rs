use clap::Parser;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let path = Path::new(filename);
    let file = match File::open(path) {
        Err(why) => panic!("Couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };
    Ok(io::BufReader::new(file).lines())
}

fn parse_line_1(line: &str) -> Vec<(i32, i32)> {
    let re = Regex::new(r"mul\((\d*),(\d*)\)").unwrap();
    re.captures_iter(&line)
        .map(|c| c.extract())
        .map(|(_, [fac1, fac2])| (fac1.parse::<i32>().unwrap(), fac2.parse::<i32>().unwrap()))
        .collect()
}

/// Solve day 3 of Advent of Code 2024
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The input file.
    #[arg(short, long)]
    path: String,
}

fn main() {
    let args = Args::parse();
    let input: Vec<String> = read_lines(&args.path).unwrap().flatten().collect();

    let result1: i32 = input
        .clone()
        .into_iter()
        .flat_map(|line| parse_line_1(&line))
        .fold(0, |acc, (fac1, fac2)| acc + (fac1 * fac2));

    let do_re = Regex::new(r"do\(\)").unwrap();
    let dont_re = Regex::new(r"don't\(\)").unwrap();
    let result2: i32 = do_re
        .split(&input.join(""))
        .flat_map(|s| dont_re.split(s).next())
        .flat_map(|s| parse_line_1(&s))
        .fold(0, |acc, (fac1, fac2)| acc + (fac1 * fac2));

    println!(
        "Result of task 1 is {}, should be 156388521 for my input.txt.",
        result1
    );
    println!(
        "Result of task 2 is {}, should be 75920122 for my input.txt.",
        result2
    );
}
