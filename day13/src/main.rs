use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
enum FoldInstruction {
    X(usize),
    Y(usize),
}

struct Origami {
    transparents: Vec<(usize, usize)>,
    instructions: Vec<FoldInstruction>,
}

fn load_input<P>(filename: P) -> io::Result<Origami>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let mut strs: Vec<_> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect();
    let split = strs.iter().position(|l| l.is_empty()).unwrap();
    let instructions: Vec<_> = strs
        .drain(split + 1..)
        .map(|s| {
            let sp = s.find('=').unwrap();
            let (txt, p) = s.split_at(sp);
            let no = p[1..].parse().unwrap();
            match txt {
                "fold along x" => FoldInstruction::X(no),
                "fold along y" => FoldInstruction::Y(no),
                _ => unreachable!(),
            }
        })
        .collect();
    // Pop empty line
    strs.pop();
    let transparents = strs
        .iter()
        .map(|s| s.split_at(s.find(',').unwrap()))
        .map(|(x, y)| (x.parse().unwrap(), y[1..].parse().unwrap()))
        .collect();
    Ok(Origami {
        transparents,
        instructions,
    })
}

impl Origami {
    fn fold(&self, folds: Option<usize>) -> Vec<Vec<u8>> {
        let x_max = self.transparents.iter().map(|(x, _)| x).max().unwrap() + 1;
        let y_max = self.transparents.iter().map(|(_, y)| y).max().unwrap() + 1;
        let mut map = vec![vec![0; x_max]; y_max];
        for (x, y) in &self.transparents {
            map[*y][*x] = 1;
        }
        let folds = folds.unwrap_or(self.instructions.len());
        for inst in &self.instructions[..folds] {
            map = match inst {
                FoldInstruction::Y(y) => {
                    let mut new: Vec<Vec<_>> = map.drain(..=y).collect();
                    new.pop();
                    let len = map.len().min(new.len());
                    for yy in 0..len {
                        for (x, s) in map[yy].iter().enumerate() {
                            new[y - 1 - yy][x] |= s;
                        }
                    }
                    new
                }
                FoldInstruction::X(x) => map
                    .into_iter()
                    .map(|mut row| {
                        let mut new: Vec<_> = row.drain(..=x).collect();
                        new.pop();
                        let len = row.len().min(new.len());
                        for (xx, s) in row.iter().take(len).enumerate() {
                            new[x - 1 - xx] |= s;
                        }
                        new
                    })
                    .collect(),
            };
        }
        map
    }
}

fn main() {
    let origami = load_input("input.txt").unwrap();
    let step1 = origami.fold(Some(1));
    let dots: usize = step1
        .iter()
        .map(|r| r.iter().filter(|v| **v != 0).count())
        .sum();
    println!("Dots after 1 step: {}", dots);
    let all_steps = origami.fold(None);
    println!("Output:");
    for row in all_steps {
        for r in row {
            print!("{}", if r > 0 { '#' } else { '.' });
        }
        println!();
    }
}
