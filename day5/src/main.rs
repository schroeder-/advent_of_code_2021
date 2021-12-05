use regex::Regex;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn load_input<P>(filename: P) -> io::Result<Vec<Line>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let re = Regex::new(r"^(.*),(.*) -> (.*),(.*)$").unwrap();
    let buf = io::BufReader::new(file);
    Ok(buf
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| {
            re.captures(&line).map(|cap|
                Line {
                    x1: cap[1].parse().unwrap(),
                    y1: cap[2].parse().unwrap(),
                    x2: cap[3].parse().unwrap(),
                    y2: cap[4].parse().unwrap(),
                })
        })
        .collect())
}

struct Line {
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
}

impl Line {
    fn start_x(&self) -> u32 {
        std::cmp::min(self.x1, self.x2)
    }
    fn start_y(&self) -> u32 {
        std::cmp::min(self.y1, self.y2)
    }

    fn end_x(&self) -> u32 {
        std::cmp::max(self.x1, self.x2)
    }

    fn end_y(&self) -> u32 {
        std::cmp::max(self.y1, self.y2)
    }

    fn horizontal(&self) -> bool {
        self.x1 == self.x2
    }

    fn vertical(&self) -> bool {
        self.y1 == self.y2
    }

    fn diag_direction(&self) -> (i32, i32) {
        if self.x1 > self.x2 {
            if self.y1 > self.y2 {
                (-1, -1)
            } else {
                (-1, 1)
            }
        } else if self.y1 > self.y2 {
            (1, -1)
        } else {
            (1, 1)
        }

    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{} -> {},{}", self.x1, self.y1, self.x2, self.y2)
    }
}

#[derive(PartialEq)]
enum HydroOperationMode {
    None,
    Diag,
}

struct HydroVenture {
    map: Vec<Vec<u32>>,
}

impl HydroVenture {
    fn new() -> Self {
        Self { map: Vec::new() }
    }

    fn find_max(lines: &[Line]) -> (usize, usize) {
        let x = lines.iter().map(Line::end_x).max().unwrap();
        let y = lines.iter().map(Line::end_y).max().unwrap();
        (x as usize, y as usize)
    }

    fn mark_vent(&mut self, line: &Line, mode: &HydroOperationMode) {
        // Only horizontal or vertical line are possible
        if line.horizontal() {
            for y in line.start_y()..=line.end_y() {
                self.map[y as usize][line.x1 as usize] += 1;
            }
        } else if line.vertical() {
            for x in line.start_x()..=line.end_x() {
                self.map[line.y1 as usize][x as usize] += 1;
            }
        } else if *mode == HydroOperationMode::Diag {
            let (dx, dy) = line.diag_direction();
            let rx: Vec<_> = if dx == 1 {
                (line.x1..=line.x2).rev().collect()
            } else {
                (line.x2..=line.x1).into_iter().collect()
            };
            let ry: Vec<_> = if dy == 1 {
                (line.y1..=line.y2).rev().collect()
            } else {
                (line.y2..=line.y1).into_iter().collect()
            };
            for (x, y) in rx.into_iter().zip(ry.into_iter()) {
                self.map[y as usize][x as usize] += 1;
            }
        }
    }

    fn mark_vents(self, lines: &[Line], mode: HydroOperationMode) -> Self {
        let (max_x, max_y) = Self::find_max(lines);
        let map = vec![vec![0; max_x + 1]; max_y + 1];
        let mut s = Self { map };
        for l in lines {
            s.mark_vent(l, &mode);
        }
        s
    }

    fn calc_danger_level(&self) -> usize {
        self.map
            .iter()
            .map(|r| r.iter().filter(|n| **n > 1).count())
            .sum()
    }
}

impl Default for HydroVenture {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for HydroVenture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.map.iter() {
            for col in row {
                if *col == 0 {
                    write!(f, ".")?;
                } else {
                    write!(f, "{}", col)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    let inp = load_input("input.txt").unwrap();

    let hydro = HydroVenture::new();
    let hydro = hydro.mark_vents(&inp, HydroOperationMode::None);
    println!("Part1: Danger level: {}", hydro.calc_danger_level());

    let hydro = HydroVenture::new();
    let hydro = hydro.mark_vents(&inp, HydroOperationMode::Diag);
    println!("Part2: Danger level: {}", hydro.calc_danger_level());
}
