use clap::Parser;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn result1(vec1: &[u32], vec2: &[u32]) -> u32 {
    let mut vec1 = vec1.to_owned();
    let mut vec2 = vec2.to_owned();

    vec1.sort();
    vec2.sort();

    vec1.into_iter()
        .zip(vec2)
        .map(|tup| tup.0.abs_diff(tup.1))
        .sum()
}

fn frequency_of_elements(vec: &[u32]) -> HashMap<u32, usize> {
    vec.into_iter()
        .fold(HashMap::<u32, usize>::new(), |mut m, val| {
            *m.entry(*val).or_default() += 1;
            m
        })
}

fn result2(vec1: &[u32], vec2: &[u32]) -> u32 {
    let set1 = frequency_of_elements(vec1);
    let set2 = frequency_of_elements(vec2);

    set1.into_iter()
        .map(|(elem, amount)| elem * (amount as u32) * (*set2.get(&elem).unwrap_or(&0) as u32))
        .sum()
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let path = Path::new(filename);
    let file = match File::open(path) {
        Err(why) => panic!("Couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };
    Ok(io::BufReader::new(file).lines())
}

fn split_line(line: String) -> (u32, u32) {
    let mut parts = line.split_whitespace();
    (
        parts.next().unwrap().to_string().parse::<u32>().unwrap(),
        parts.next().unwrap().to_string().parse::<u32>().unwrap(),
    )
}

/// Solve day 1 of Advent of Code 2024
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The input file.
    #[arg(short, long)]
    path: String,
}

fn main() {
    let args = Args::parse();

    let (vec1, vec2): (Vec<u32>, Vec<u32>) = read_lines(&args.path)
        .unwrap()
        .map(|line| split_line(line.unwrap()))
        .collect::<Vec<(u32, u32)>>()
        .into_iter()
        .unzip();

    println!(
        "Result of task 1 is {}, should be 2375403 for my input.txt.",
        result1(&vec1, &vec2)
    );
    println!(
        "Result of task 2 is {}, should be 23082277 for my input.txt.",
        result2(&vec1, &vec2)
    );
}
