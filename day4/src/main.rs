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

fn rotate90(input: &Vec<String>) -> Vec<String> {
    input
        .clone()
        .into_iter()
        .map(|s: String| s.chars().enumerate().collect::<Vec<(usize, char)>>())
        .flatten()
        .rev()
        .fold(
            std::iter::repeat_n(vec![], input.len()).collect(),
            |mut vecs: Vec<Vec<char>>, (i, v)| {
                vecs[i].push(v);
                vecs
            },
        )
        .into_iter()
        .map(|arr| String::from_iter(arr))
        .collect::<Vec<String>>()
}

fn rotate45(input: &Vec<String>) -> Vec<String> {
    input
        .clone()
        .into_iter()
        .map(|s: String| s.chars().enumerate().collect::<Vec<(usize, char)>>())
        .enumerate()
        .flat_map(|(oi, arr)| {
            arr.into_iter()
                .map(|(ii, v)| (ii + oi, v))
                .collect::<Vec<(usize, char)>>()
        })
        .rev()
        .fold(
            std::iter::repeat_n(vec![], input.len() * 2 - 1).collect(),
            |mut vecs: Vec<Vec<char>>, (i, v): (usize, char)| {
                vecs[i].push(v);
                vecs
            },
        )
        .into_iter()
        .map(|arr| String::from_iter(arr))
        .collect::<Vec<String>>()
}

fn result1(mut input: Vec<String>) -> usize {
    let xmas_re = Regex::new(r"XMAS").unwrap();
    let mut result: usize = 0;

    for _ in 0..4 {
        input = rotate90(&input);
        result += input
            .clone()
            .into_iter()
            .fold(0, |acc, s| acc + xmas_re.find_iter(&s).count());
        result += rotate45(&input)
            .into_iter()
            .fold(0, |acc, s| acc + xmas_re.find_iter(&s).count());
    }

    result
}

fn check_xmas(mut input: Vec<String>) -> bool {
    let mas_re = Regex::new(r"MAS").unwrap();
    let mut result: usize = 0;

    for _ in 0..4 {
        input = rotate90(&input);
        result += rotate45(&input)
            .into_iter()
            .fold(0, |acc, s| acc + mas_re.find_iter(&s).count());
    }

    result == 2
}

fn result2(input: &Vec<String>) -> usize {
    input
        .clone()
        .into_iter()
        .map(|s: String| s.chars().collect::<Vec<char>>())
        .map(|c: Vec<char>| {
            c.windows(3)
                .map(|w| String::from_iter(vec![w[0].clone(), w[1].clone(), w[2].clone()]))
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>()
        .windows(3)
        .map(|w| {
            vec![
                w[0].clone()
                    .into_iter()
                    .enumerate()
                    .collect::<Vec<(usize, String)>>(),
                w[1].clone()
                    .into_iter()
                    .enumerate()
                    .collect::<Vec<(usize, String)>>(),
                w[2].clone()
                    .into_iter()
                    .enumerate()
                    .collect::<Vec<(usize, String)>>(),
            ]
        })
        .map(|v| v.into_iter().flatten().collect::<Vec<(usize, String)>>())
        .map(|v: Vec<(usize, String)>| {
            v.into_iter().fold(
                std::iter::repeat_n(vec![], input[0].chars().count() - 2).collect(),
                |mut vecs: Vec<Vec<String>>, (i, s): (usize, String)| {
                    vecs[i].push(s.clone());
                    vecs
                },
            )
        })
        .flatten()
        .collect::<Vec<Vec<String>>>()
        .into_iter()
        .map(|v| check_xmas(v.clone()))
        .filter(|b| *b)
        .count()
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

    assert_eq!(input.len(), input[0].len());

    println!(
        "Result of task 1 is {}, should be 2549 for my input.txt.",
        result1(input.clone())
    );

    println!(
        "Result of task 2 is {}, should be 2003 for my input.txt.",
        result2(&input)
    );
}
