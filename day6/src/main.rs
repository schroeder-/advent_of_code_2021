use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn load_input<P>(filename: P) -> io::Result<Vec<u64>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let mut res = Vec::new();
    for line in io::BufReader::new(file).lines() {
        res.extend(line.unwrap().split(',').map(|n| n.parse::<u64>().unwrap()));
    }
    Ok(res)
}

fn calc_fish_count(fishes: &[u64], n: u64) -> usize {
    // group all fishes into days left to birth
    let mut grouped: Vec<_> = (0..=8)
        .map(|x| fishes.iter().filter(|f| x == **f).count())
        .collect();
    for _ in 0..n {
        // for the count of day0 fishes are created (through rotate left)
        let cnt_0days = grouped[0];
        grouped.rotate_left(1);
        // ay 0 fished get set to 6 days left
        grouped[6] += cnt_0days;
    }
    // count all fishes in all day groups
    grouped.iter().sum()
}

fn main() {
    //let mut fishes = load_input("simple_input.txt").unwrap();
    let fishes = load_input("input.txt").unwrap();
    println!("day 18 {}", calc_fish_count(&fishes, 18));
    println!("day 60 {}", calc_fish_count(&fishes, 60));
    println!("day 256 {}", calc_fish_count(&fishes, 256));
}
