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

fn parse_line_2(line: &str) -> Vec<(i32, i32)> {
    let do_re = Regex::new(r"do\(\)").unwrap();
    let dont_re = Regex::new(r"don't\(\)").unwrap();

    fn get_positions(re: &Regex, line: &str) -> Vec<usize> {
        re.captures_iter(line)
            .map(|c| c.get(0).unwrap().start())
            .collect()
    }

    fn find_greater(val: usize, vec: &[usize]) -> Option<usize> {
        vec.to_vec()
            .into_iter()
            .filter(|v| *v > val)
            .collect::<Vec<usize>>()
            .get(0)
            .copied()
    }

    let do_pos: Vec<usize> = get_positions(&do_re, line);
    let dont_pos: Vec<usize> = get_positions(&dont_re, line);

    let mut results: Vec<(usize, usize)> = Vec::new();
    let mut last_do: usize = 0;

    loop {
        let last_dont: usize;
        if let Some(new_dont) = find_greater(last_do, &dont_pos) {
            last_dont = new_dont;
            results.push((last_do, last_dont));
        } else {
            results.push((last_do, line.len() - 1));
            break;
        }

        if let Some(new_do) = find_greater(last_dont, &do_pos) {
            last_do = new_do;
        } else {
            break;
        }
    }

    results
        .into_iter()
        .map(|(start, end)| parse_line_1(&line[start..end]))
        .flatten()
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
    let input: Vec<String> = read_lines(&args.path)
        .unwrap()
        .map(|line| line.unwrap())
        .collect();

    let result1: i32 = input
        .clone()
        .into_iter()
        .map(|line| parse_line_1(&line))
        .map(|arr| {
            arr.into_iter()
                .fold(0, |acc, (fac1, fac2)| acc + (fac1 * fac2))
        })
        .sum();

    let result2: i32 = parse_line_2(&input.join(""))
        .into_iter()
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
