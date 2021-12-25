use std::{
    fmt::{Display, Write},
    fs::File,
    io::{self, BufRead},
    path::Path,
};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Left,
    Down,
    None,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Direction::Left => '>',
            Direction::Down => 'v',
            Direction::None => '.',
        };
        f.write_char(c)
    }
}

struct SeaFloor {
    floor: Vec<Vec<Direction>>,
}

impl SeaFloor {
    fn new(floor: Vec<Vec<Direction>>) -> Self {
        Self { floor }
    }
    fn print(f: &[Vec<Direction>]) {
        for r in f {
            for v in r {
                print!("{}", v);
            }
            println!();
        }
    }
    fn move_till_stop(&self) -> usize {
        let mut floor = self.floor.clone();
        let mut cnt = 0;
        let stops = [0, 1, 2, 3, 4, 5, 10, 20, 30, 40, 50, 55];
        loop {
            let mut moves = 0;
            if stops.contains(&cnt) {
                println!("After: {}", cnt);
                Self::print(&floor);
            }
            let mv: Vec<_> = floor
                .iter()
                .enumerate()
                .map(|(y, row)| {
                    row.iter()
                        .enumerate()
                        .filter_map(|(x, v)| {
                            if *v == Direction::Left && row[(x + 1) % row.len()] == Direction::None
                            {
                                Some((y, x))
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .flatten()
                .collect();
            moves += mv.len();
            for (y, x) in mv {
                let nx = (x + 1) % floor[y].len();
                floor[y][nx] = floor[y][x];
                floor[y][x] = Direction::None
            }
            let mv: Vec<_> = floor
                .iter()
                .enumerate()
                .map(|(y, row)| {
                    row.iter()
                        .enumerate()
                        .filter_map(|(x, v)| {
                            if *v == Direction::Down
                                && floor[(y + 1) % floor.len()][x] == Direction::None
                            {
                                Some((y, x))
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .flatten()
                .collect();
            moves += mv.len();
            for (y, x) in mv {
                let ny = (y + 1) % floor.len();
                floor[ny][x] = floor[y][x];
                floor[y][x] = Direction::None
            }
            cnt += 1;
            if moves == 0 {
                break cnt;
            }
        }
    }
}

fn load_input<P>(filename: P) -> io::Result<Vec<Vec<Direction>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file)
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| match c {
                    '>' => Direction::Left,
                    'v' => Direction::Down,
                    _ => Direction::None,
                })
                .collect()
        })
        .collect())
}

fn main() {
    let inp = load_input("input.txt").unwrap();
    let floor = SeaFloor::new(inp);

    println!("Rounds till stop {}", floor.move_till_stop());
}
