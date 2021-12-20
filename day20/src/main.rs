use bitvec::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

type Map = HashMap<(isize, isize), u8>;

fn input_to_string(inp: &Map) {
    let of_x = inp.iter().map(|(p, _)| p.0).min().unwrap();
    let of_y = inp.iter().map(|(p, _)| p.1).min().unwrap();
    let max_x = inp.iter().map(|(p, _)| p.0).max().unwrap();
    let max_y = inp.iter().map(|(p, _)| p.1).max().unwrap();
    for y in of_y..max_y {
        for x in of_x..max_x {
            print!(
                "{}",
                match inp.get(&(x, y)).unwrap_or(&0) {
                    0 => '.',
                    1 => '#',
                    _ => unreachable!(),
                }
            );
        }
        println!();
    }
}

fn calc_values(map: &Map, p: (isize, isize), default: u8, rules: &BitVec) -> u8 {
    let mut v = 0;
    for (x, y) in [
        (p.0 - 1, p.1 - 1),
        (p.0, p.1 - 1),
        (p.0 + 1, p.1 - 1),
        (p.0 - 1, p.1),
        (p.0, p.1),
        (p.0 + 1, p.1),
        (p.0 - 1, p.1 + 1),
        (p.0, p.1 + 1),
        (p.0 + 1, p.1 + 1),
    ] {
        v = (v << 1) | (map.get(&(x, y)).unwrap_or(&default) & 1) as isize;
    }
    rules[v as usize] as u8
}

fn image_enhancement(inp: Map, rules: &BitVec, def: u8) -> Map {
    let mut map = HashMap::new();
    let of_x = inp.iter().map(|(p, _)| p.0).min().unwrap();
    let of_y = inp.iter().map(|(p, _)| p.1).min().unwrap();
    let max_x = inp.iter().map(|(p, _)| p.0).max().unwrap();
    let max_y = inp.iter().map(|(p, _)| p.1).max().unwrap();

    for y in of_y - 1..=max_y + 1 {
        for x in of_x - 1..=max_x + 1 {
            *map.entry((x, y)).or_default() = calc_values(&inp, (x, y), def, rules);
        }
    }

    map
}

fn part_a(inp: &Map, rules: &BitVec, steps: usize) -> usize {
    let mut imag = inp.clone();
    input_to_string(&imag);
    for s in 0..steps {
        let def = match rules[0] {
            true => 0,
            false => s as u8 % 2,
        };
        imag = image_enhancement(imag, rules, def);
        input_to_string(&imag);
    }
    imag.iter().map(|(_, c)| c).filter(|c| **c == 1).count()
}

fn load_input<P>(filename: P) -> io::Result<(BitVec, Map)>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let lines: Vec<_> = io::BufReader::new(file)
        .lines()
        .filter_map(|f| f.ok())
        .map(|s| {
            s.trim()
                .chars()
                .map(|c| match c {
                    '#' => true,
                    '.' => false,
                    _ => unreachable!(),
                })
                .collect::<BitVec>()
        })
        .collect();
    let rules = lines[0].clone();
    let image: Map = lines[2..]
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, col)| ((x as isize, y as isize), if *col { 1u8 } else { 0u8 }))
                .collect::<Vec<_>>()
        })
        .collect();
    Ok((rules, image))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let (rules, inp) = load_input("simple.txt").unwrap();
        let v = part_a(&inp, &rules, 2);
        assert_eq!(v, 35);
    }

    #[test]
    fn test2() {
        let (rules, inp) = load_input("input.txt").unwrap();
        let v = part_a(&inp, &rules, 2);
        println!("Part a: {}", v);
    }
}

fn main() {
    let (rules, inp) = load_input("input.txt").unwrap();
    let v = part_a(&inp, &rules, 2);
    println!("Part a: {}", v);
    let (rules, inp) = load_input("input.txt").unwrap();
    let v = part_a(&inp, &rules, 50);
    println!("Part b: {}", v);
}
