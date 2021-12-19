#[macro_use]
extern crate lazy_static;
use std::collections::HashSet;
use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Position {
    x: isize,
    y: isize,
    z: isize,
}

impl Position {
    fn rotate(&self, rot: u8) -> Self {
        match rot {
    0  => Position { x:  self.x, y:  self.y, z:  self.z},
    1  => Position { x:  self.x, y:  self.z, z: -self.y},
    2  => Position { x:  self.x, y: -self.y, z: -self.z},
    3  => Position { x:  self.x, y: -self.z, z:  self.y},
    4  => Position { x:  self.y, y:  self.x, z: -self.z},
    5  => Position { x:  self.y, y:  self.z, z:  self.x},
    6  => Position { x:  self.y, y: -self.x, z:  self.z},
    7  => Position { x:  self.y, y: -self.z, z: -self.x},
    8  => Position { x:  self.z, y:  self.x, z:  self.y},
    9  => Position { x:  self.z, y:  self.y, z: -self.x},
    10 => Position { x:  self.z, y: -self.x, z: -self.y},
    11 => Position { x:  self.z, y: -self.y, z:  self.x},
    12 => Position { x: -self.x, y:  self.y, z: -self.z},
    13 => Position { x: -self.x, y:  self.z, z:  self.y},
    14 => Position { x: -self.x, y: -self.y, z:  self.z},
    15 => Position { x: -self.x, y: -self.z, z: -self.y},
    16 => Position { x: -self.y, y:  self.x, z:  self.z},
    17 => Position { x: -self.y, y:  self.z, z: -self.x},
    18 => Position { x: -self.y, y: -self.x, z: -self.z},
    19 => Position { x: -self.y, y: -self.z, z:  self.x},
    20 => Position { x: -self.z, y:  self.x, z: -self.y},
    21 => Position { x: -self.z, y:  self.y, z:  self.x},
    22 => Position { x: -self.z, y: -self.x, z:  self.y},
    23 => Position { x: -self.z, y: -self.y, z: -self.x},
            _ => unreachable!(),
        }
    }

    fn sub_ref(&self, rhs: &Self) -> Self {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }

    fn add_ref(&self, rhs: &Self) -> Self {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }

    fn manhatten_distance(&self, other: &Self) -> isize{
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}


#[derive(Debug, Hash, Clone)]
struct ScannerMap {
    beacons: Vec<Position>
}


impl ScannerMap {
    fn from_strings(strs: &[String]) -> Option<Self> {
        lazy_static! {
            static ref RE: regex::Regex =
                regex::Regex::new(r"\-\-\- scanner \d* \-\-\-$").unwrap();
        }
        let firstline = strs.first().unwrap();
        if RE.is_match(firstline) {
            let beacons: Vec<_> = strs
                .iter()
                .skip(1)
                .map(|s| {
                    let mut nos = s.split(',').map(|f| f.parse().unwrap());
                    Position {
                        x: nos.next().unwrap(),
                        y: nos.next().unwrap(),
                        z: nos.next().unwrap(),
                    }
                })
                .collect();
            Some(Self { beacons })
        } else {
            None
        }
    }

    fn rotate(&self, rot: u8) -> Self {
        let beacons = self.beacons.iter().map(|b| b.rotate(rot)).collect();
        Self {
            beacons,
        }
    }
}

fn find_beacons(all_scans: &mut HashSet<Position>, scan: &ScannerMap) -> Option<Position> {
    for rot in 0u8..24u8 {
        let rotated_scan = scan.rotate(rot);
        let dists = all_scans
            .iter()
            .cartesian_product(&rotated_scan.beacons)
            .map(|(p1, p2)| p1.sub_ref(p2));
        for dist in dists {
            let normalized = rotated_scan.beacons.iter().map(|p| p.add_ref(&dist));
            if normalized.clone().filter(|pp| all_scans.contains(pp)).count() >= 12 {
                all_scans.extend(normalized);
                return Some(dist);
            }
        }
    }
    None
}

fn find_all_beacons_and_manhatten_distance(scanner: &mut Vec<ScannerMap>) -> (usize, isize) {
    let s0 = scanner.remove(0);
    let mut total_scanner: HashSet<_> = s0.beacons.iter().cloned().collect();
    let mut dists = Vec::new();
    while !scanner.is_empty() {
        for i in (0..scanner.len()).rev() {
            if let Some(dist) = find_beacons(&mut total_scanner, &scanner[i]) {
                dists.push(dist);
                scanner.remove(i);
            }
        }
    }
    let beacon_cnt = total_scanner.len();
    let manhatten_max = dists.iter().tuple_combinations().map(|(p1, p2)| p2.manhatten_distance(p1)).max().unwrap();
    (beacon_cnt, manhatten_max)
}

fn load_input<P>(filename: P) -> io::Result<Vec<ScannerMap>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let lines: Vec<_> = io::BufReader::new(file)
        .lines()
        .filter_map(|f| f.ok())
        .collect();
    Ok(lines
        .split(|n| n.is_empty())
        .map(|text| ScannerMap::from_strings(text).unwrap())
        .collect())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parser() {
        let _ = load_input("simple_input.txt").unwrap();
    }
    #[test]
    fn find_out_count() {
        let mut s = load_input("simple_input.txt").unwrap();
        assert_eq!(find_all_beacons_and_manhatten_distance(&mut s).0, 79);
    }
    #[test]
    fn test_manhatten() {
        let mut s = load_input("simple_input.txt").unwrap();
        assert_eq!(find_all_beacons_and_manhatten_distance(&mut s).1, 3621);
    }
}

fn main() {
    let mut s = load_input("input.txt").unwrap();
    let (cnt, max_man) = find_all_beacons_and_manhatten_distance(&mut s);
    println!("beacon count: {}", cnt);
    println!("largest manhattan distance: {}", max_man);
}
