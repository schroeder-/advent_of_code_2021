use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn load_input<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect())
}

fn check_line(l: &str) -> usize {
    let mut stack = Vec::new();
    let illegal = l
        .chars()
        .find(|c| {
            match c {
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                '<' => stack.push('>'),
                _ => {
                    if Some(*c) != stack.pop() {
                        return true;
                    }
                }
            }
            false
        });
    if let Some(c) = illegal {
        match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => unreachable!(),
        }
    } else {
        0
    }
}

fn get_completion_score(c: &char) -> usize {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => unreachable!(),
    }
}

fn calc_completion(l: &str) -> Option<usize> {
    let mut stack = Vec::new();
    for c in l.chars() {
        match c {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            '<' => stack.push('>'),
            _ => {
                if Some(c) != stack.pop() {
                    return None;
                }
            }
        }
    }
    Some(
        stack
            .iter()
            .rev()
            .map(get_completion_score)
            .fold(0, |c, i| c * 5 + i),
    )
}

fn main() {
    let inp = load_input("input.txt").unwrap();
    let line_check: Vec<_> = inp.iter().map(|f| check_line(f)).collect();
    let score: usize = line_check.iter().sum();
    println!("error score {}", score);
    let mut scores: Vec<_> = inp.iter().filter_map(|f| calc_completion(f)).collect();
    scores.sort_unstable();
    let median_score = scores[scores.len() / 2];
    println!("completion score {}", median_score);
}
