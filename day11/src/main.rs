use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn do_round(inp: &[Vec<u32>]) -> (Vec<Vec<u32>>, usize) {
    let x_m = inp[0].len() as i32;
    let y_m = inp.len() as i32;
    // Update all values by 1
    let mut grid: Vec<Vec<u32>> = inp
        .iter()
        .map(|y| y.iter().map(|x| *x + 1).collect())
        .collect();

    let mut count = 0;
    loop {
        let last_count = count;
        for x in 0..x_m {
            for y in 0..y_m {
                let p = &mut grid[y as usize][x as usize];
                if *p == 10 {
                    *p = 0;
                    count += 1;
                    for (nx, ny) in [
                        (x, y + 1),
                        (x + 1, y),
                        (x + 1, y + 1),
                        (x, y - 1),
                        (x - 1, y),
                        (x - 1, y - 1),
                        (x + 1, y - 1),
                        (x - 1, y + 1),
                    ] {
                        if nx >= 0 && nx < x_m && ny >= 0 && ny < y_m {
                            let np = &mut grid[ny as usize][nx as usize];
                            if *np != 0 && *np != 10 {
                                *np += 1;
                            }
                        }
                    }
                }
            }
        }
        if last_count == count {
            break;
        }
    }
    (grid, count)
}

fn simulate(inp: &[Vec<u32>], rounds: usize) -> usize {
    (0..rounds)
        .fold((inp.to_vec(), 0), |(last, flashes), _| {
            let mut res = do_round(&last);
            res.1 += flashes;
            res
        })
        .1
}

fn check_for_flash(inp: &[Vec<u32>]) -> usize {
    let mut grid = (*inp).to_vec();
    let mut round = 0;
    loop {
        grid = do_round(&grid).0;
        round += 1;
        if grid.iter().all(|y| y.iter().all(|x| *x == 0)) {
            break round;
        }
    }
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
    let inp = load_input("input.txt").unwrap();
    let flashes = simulate(&inp, 10);
    println!("After 10 round {}", flashes);
    let flashes = simulate(&inp, 100);
    println!("After 100 round {}", flashes);
    let all_flash = check_for_flash(&inp);
    println!("All flash at round {}", all_flash);
}
