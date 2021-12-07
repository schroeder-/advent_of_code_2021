use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn load_input<P>(filename: P) -> io::Result<Vec<i32>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let mut res = Vec::new();
    for line in io::BufReader::new(file).lines() {
        res.extend(line.unwrap().split(',').map(|n| n.parse::<i32>().unwrap()));
    }
    Ok(res)
}

fn fuel_calc<F>(crap_positions: &[i32], func: F) -> (i32, i32) where F: Fn(i32, i32) -> i32{
    let max = crap_positions.iter().max().copied().unwrap();
    let min = crap_positions.iter().min().copied().unwrap();
    let fuels: Vec<_> = (min..=max).map(|f|
        crap_positions.iter()
            .map(|x| {
                func(*x, f)
            }).sum()).collect();
    let min: i32 = fuels.iter().min().copied().unwrap();
    let v = crap_positions[fuels.iter().position(|x| *x == min).unwrap()];
    (min, v)
}


fn diff(x: i32, y: i32) -> i32{
    (x - y).abs()
}

fn diff2(x: i32, y: i32) -> i32{
    let n = (x - y).abs();
    // gaussian sum formula
    (n.pow(2) + n) / 2
}

fn main() {
    println!("Part 1");
    //let pos = vec![16,1,2,0,4,2,7,1,2,14];
    let pos = load_input("input.txt").unwrap();
    let (min, p) = fuel_calc(&pos, diff);
    println!("Fuel needed min {}", min);
    println!("Pos {}", p);
    // Part2
    println!("Part 2");
    let (min, p) = fuel_calc(&pos, diff2);
    println!("Fuel needed min {}", min);
    println!("Pos {}", p);
}
