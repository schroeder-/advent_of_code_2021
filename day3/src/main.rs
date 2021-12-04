use std::convert::TryInto;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn calc_bitcount(lines: &[String]) -> Vec<i32> {
    let line_sz = lines[0].len();
    let mut res = vec![0; line_sz];
    for line in lines {
        for (x, c) in line.chars().enumerate() {
            res[x] += if c == '1' { 1 } else { 0 };
        }
    }
    res
}

fn calc_consumption(lines: &[String], bitcount: &[i32]) -> i32 {
    let msb = (lines.len() / 2).try_into().unwrap();
    let mut gamma = 0;
    let mut epsilion = 0;
    for (i, v) in bitcount.iter().rev().enumerate() {
        if v > &msb {
            gamma += 1 << i;
        } else {
            epsilion += 1 << i;
        }
    }
    gamma * epsilion
}

fn get_life_value(lines: &[String], line_sz: usize, x: usize, neg: bool) -> i32 {
    let (c_p, c_n) = if neg { ('0', '1') } else { ('1', '0') };
    if x > line_sz {
        0
    } else {
        let s: usize = lines
            .iter()
            .map(|f| {
                if f.chars().nth(x).unwrap() == '1' {
                    1
                } else {
                    0
                }
            })
            .sum();
        let c = if s >= ((lines.len() + 1) / 2) {
            c_p
        } else {
            c_n
        };
        let res: Vec<_> = lines
            .iter()
            .filter(|l| l.chars().nth(x).unwrap() == c)
            .map(String::clone)
            .collect();
        if res.len() == 1 {
            i32::from_str_radix(&res[0], 2).unwrap()
        } else {
            get_life_value(&res, line_sz, x + 1, neg)
        }
    }
}

fn calc_life_support(lines: &[String]) -> i32 {
    let oxygen = get_life_value(lines, lines.len(), 0, true);
    let c02 = get_life_value(lines, lines.len(), 0, false);
    println!("{}, {}", oxygen, c02);
    c02 * oxygen
}

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

fn main() {
    let input = load_file("input.txt").unwrap();
    let bitcount = calc_bitcount(&input);
    println!("Power consumption {}", calc_consumption(&input, &bitcount));
    println!("oxygen  rating {}", calc_life_support(&input));
}
