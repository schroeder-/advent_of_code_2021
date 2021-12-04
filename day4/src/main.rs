use std::fmt::Debug;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn load_file<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .collect())
}

#[derive(Debug)]
struct Board {
    arr: [[i32; 5]; 5],
    no: usize,
}

impl Board {
    fn from_vec(no: usize, numbers: &[Vec<i32>]) -> Self {
        let mut b = Self {
            arr: [[0; 5]; 5],
            no,
        };
        for (x, nos) in numbers.iter().enumerate() {
            for (y, n) in nos.iter().enumerate() {
                b.arr[x][y] = *n;
            }
        }
        b
    }

    fn calc_score(&self, number: &[i32]) -> i32 {
        let sum: i32 = self
            .arr
            .iter()
            .map(|line| line.iter().filter(|x| !number.contains(x)).sum::<i32>())
            .sum();
        sum * number.last().unwrap()
    }

    fn bingo(&self, numbers: &[i32]) -> bool {
        for x in 0..5 {
            let matches = numbers.iter().filter(|n| self.arr[x].contains(n)).count();
            if matches == 5 {
                return true;
            }
            let y: Vec<_> = self.arr.iter().map(|arr| arr[x]).collect();
            let matches = numbers.iter().filter(|n| y.contains(n)).count();
            if matches == 5 {
                return true;
            }
        }
        false
    }
}

fn first_bingo(inp: &[i32], boards: &[Board]) -> Option<(usize, usize)> {
    for x in 5..inp.len() {
        for b in boards.iter() {
            if b.bingo(&inp[0..x]) {
                return Some((x, b.no));
            }
        }
    }
    None
}

fn last_bingo(inp: &[i32], boards: &[Board], offset: usize) -> Option<(usize, usize)> {
    let mut bb: Vec<_> = boards.iter().collect();
    for x in offset..inp.len() {
        let (filtered, tmp): (_, Vec<_>) = bb.into_iter().partition(|b| b.bingo(&inp[0..x]));
        if tmp.is_empty() {
            return Some((x, filtered[0].no));
        }
        bb = tmp
    }
    None
}

fn load_boards(inp: &[String]) -> Vec<Board> {
    inp.chunks(6)
        .enumerate()
        .map(|(n, lines)| {
            let x: Vec<Vec<i32>> = lines
                .iter()
                .skip(1)
                .map(|s| s.split(' ').filter_map(|l| l.parse::<i32>().ok()).collect())
                .collect();
            Board::from_vec(n, &x)
        })
        .collect()
}

fn main() {
    let input = load_file("input.txt").unwrap();
    let inp: Vec<i32> = input
        .first()
        .unwrap()
        .split(',')
        .filter_map(|s| s.parse().ok())
        .collect();
    let boards: Vec<Board> = load_boards(&input[1..]);
    let winner = first_bingo(&inp, &boards);
    if let Some((rounds, winner)) = winner {
        println!("Winner is: {}", winner);
        let score = boards[winner].calc_score(&inp[..rounds]);
        println!("Score is {}", score);
        let (last_round, last) = last_bingo(&inp, &boards, rounds).unwrap();
        println!("Last Winer {}", last);
        let score = boards[last].calc_score(&inp[..last_round]);
        println!("Last Score {}", score);
    } else {
        println!("No Winner found!")
    }
}
