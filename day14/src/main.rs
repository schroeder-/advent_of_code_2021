use itertools::{Itertools, MinMaxResult};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

type PuzzleInput = (String, HashMap<(char, char), char>);

fn load_input<P>(filename: P) -> io::Result<PuzzleInput>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let mut lines = io::BufReader::new(file).lines();
    let polymer = lines.next().unwrap().unwrap();
    let table = lines
        .skip(1)
        .map(|l| {
            let l = l.unwrap();
            let (pair, c) = l.split_once(" -> ").unwrap();
            (
                (pair.chars().next().unwrap(), pair.chars().nth(1).unwrap()),
                c.chars().next().unwrap(),
            )
        })
        .collect();
    Ok((polymer, table))
}

fn calc_polymer_freq(polymer: &str, table: &HashMap<(char, char), char>, steps: usize) -> usize {
    let freq = polymer
        .chars()
        .tuple_windows()
        .map(|(a, b)| (a, b))
        .counts();
    let freq = (0..steps).fold(freq, |freq, _| {
        let mut next_freq = freq.clone();
        for ((c1, c2), f) in freq {
            if let Some(v) = table.get(&(c1, c2)) {
                *next_freq.entry((c1, *v)).or_insert(0) += f;
                *next_freq.entry((*v, c2)).or_insert(0) += f;
                *next_freq.entry((c1, c2)).or_insert(0) -= f;
            }
        }
        next_freq
    });
    let mut chars = HashMap::new();
    for ((c1, c2), f) in freq {
        *chars.entry(c1).or_insert(0) += f;
        *chars.entry(c2).or_insert(0) += f;
    }
    if let MinMaxResult::MinMax(min, max) = chars.into_iter().map(|(_, f)| f).minmax() {
        max / 2 - min / 2 + 1
    } else {
        0
    }
}

fn main() {
    let (polymer, table) = load_input("input.txt").unwrap();
    let start = Instant::now();
    let s1 = calc_polymer_freq(&polymer, &table, 10);
    let s2 = calc_polymer_freq(&polymer, &table, 40);
    let duration = start.elapsed();
    println!("After 10 steps: {}", s1);
    println!("After 40 steps: {}", s2);
    println!("Took {}us", duration.as_micros());
}
