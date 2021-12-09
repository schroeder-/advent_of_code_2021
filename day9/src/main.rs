use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn find_low_points(d: &[Vec<u32>]) -> Vec<(usize, usize)> {
    let x_max = d[0].len();
    let y_max = d.len();
    let mut lows = Vec::new();
    for y in 0..y_max {
        for x in 0..x_max {
            let p = d[y][x];
            if (x == 0 || d[y][x - 1] > p)
                && (y == 0 || d[y - 1][x] > p)
                && (x >= x_max - 1 || d[y][x + 1] > p)
                && (y >= y_max - 1 || d[y + 1][x] > p)
            {
                lows.push((x, y));
            }
        }
    }
    lows
}

fn calc_basin(d: &[Vec<u32>], x: usize, y: usize, visited: &mut HashSet<(usize, usize)>) -> usize {
    visited.insert((x, y));
    let mut cnt = 0;
    let p = d[y][x];
    // height 9 doesn't count as basin
    if p != 9 {
        if x != 0 && !visited.contains(&(x - 1, y)) && d[y][x - 1] > p {
            cnt += calc_basin(d, x - 1, y, visited);
        }
        if y != 0 && !visited.contains(&(x, y - 1)) && d[y - 1][x] > p {
            cnt += calc_basin(d, x, y - 1, visited);
        }
        if x + 1 < d[y].len() && !visited.contains(&(x + 1, y)) {
            cnt += calc_basin(d, x + 1, y, visited);
        }
        if y + 1 < d.len() && !visited.contains(&(x, y + 1)) && d[y + 1][x] > p {
            cnt += calc_basin(d, x, y + 1, visited);
        }
        cnt + 1
    } else {
        0
    }
}

fn find_largest_basins(d: &[Vec<u32>], lows: &[(usize, usize)]) -> Vec<usize> {
    let mut basins: Vec<_> = lows
        .iter()
        .map(|(x, y)| calc_basin(d, *x, *y, &mut HashSet::new()))
        .collect();
    basins.sort_unstable();
    basins
}

fn load_input<P>(filename: P) -> io::Result<Vec<Vec<u32>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file)
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect())
}

fn main() {
    let d = load_input("input.txt").unwrap();
    let lows = find_low_points(&d);
    let cnt: u32 = lows.iter().map(|(x, y)| d[*y][*x] + 1).sum();
    println!("Numer of lows {}", cnt);
    let danger_count = find_largest_basins(&d, &lows)
        .iter()
        .rev()
        .take(3)
        .fold(1, |c, i| i * c);
    println!("danger_count = {}", danger_count);
}
