use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn calc_increased_measure(data: &[i32]) -> u32 {
    data.iter()
        .skip(1)
        .fold((0, data[0]), |(cnt, prev), item| {
            if *item > prev {
                (cnt + 1, *item)
            } else {
                (cnt, *item)
            }
        })
        .0
}

fn calc_increase_measure_sliding(data: &[i32]) -> u32 {
    let sliding: Vec<_> = data.windows(3).map(|a| a.iter().sum()).collect();
    calc_increased_measure(&sliding)
}

fn load_file<P>(filename: P) -> io::Result<Vec<i32>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap().parse::<i32>().unwrap())
        .collect())
}

fn main() {
    match load_file("input.txt") {
        Ok(puzzle) => {
            let result = calc_increased_measure(&puzzle);
            println!("Basic Changes {}", result);
            let result = calc_increase_measure_sliding(&puzzle);
            println!("Sliding Changes {}", result);
        }
        Err(err) => {
            println!("Error reading file {}", err);
        }
    }
}
