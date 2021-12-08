use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::path::Path;
struct SegmentDecoder {
    patterns: Vec<String>,
    display: Vec<String>,
}

impl SegmentDecoder {
    fn display_contains_guessable(&self) -> usize {
        self.display
            .iter()
            .filter(|f| matches!(f.len(), 2 | 3 | 4 | 7))
            .count()
    }

    fn get_bitset(&self, n: usize) -> u8 {
        self.patterns
            .iter()
            .find(|p| p.len() == n)
            .unwrap()
            .chars()
            .map(|c| (1 << (c as u8 - b'a')))
            .sum()
    }

    fn get_bitsets(&self, n: usize) -> Vec<u8> {
        self.patterns
            .iter()
            .filter(|p| p.len() == n)
            .map(|p| p.chars().map(|c| (1 << (c as u8 - b'a'))).sum())
            .collect()
    }

    fn calc_number(&self) -> usize {
        // Find all numbers via logic like subset of and so one
        // This ones are obvious because of the length
        let one = self.get_bitset(2);
        let seven = self.get_bitset(3);
        let four = self.get_bitset(4);
        let eight = self.get_bitset(7);
        // the others can be determine via masking
        let digts5 = self.get_bitsets(5);
        let three = *digts5.iter().find(|x| (one & **x) == one).unwrap();
        assert!(digts5.len() == 3);

        let digts6 = self.get_bitsets(6);
        assert!(digts6.len() == 3);
        let six = *digts6.iter().find(|x| (one & **x) != one).unwrap();

        let c = six ^ eight;
        let two = *digts5
            .iter()
            .find(|x| **x & c == c && **x != three)
            .unwrap();
        let nine = *digts6.iter().find(|x| (four & **x) == four).unwrap();
        let zero = *digts6.iter().find(|x| **x != nine && **x != six).unwrap();
        let five = *digts5.iter().find(|x| **x != two && **x != three).unwrap();
        let digits: Vec<u8> = vec![zero, one, two, three, four, five, six, seven, eight, nine];
        let cnt = self.display.len();
        // Generate the digits for the result
        self.display
            .iter()
            .enumerate()
            .map(|(i, d)| {
                let s: u8 = d.chars().map(|c| (1 << (c as u8 - b'a'))).sum();
                let p = digits.iter().position(|p| *p == s).unwrap();
                p * 10usize.pow((cnt - i - 1) as u32)
            })
            .sum()
    }
}

fn load_file<P>(filename: P) -> io::Result<Vec<SegmentDecoder>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .map(|l| {
            let mut it = l.split('|');
            SegmentDecoder {
                patterns: it
                    .next()
                    .unwrap()
                    .split(' ')
                    .filter(|s| !s.is_empty())
                    .map(str::to_string)
                    .collect(),
                display: it
                    .next()
                    .unwrap()
                    .split(' ')
                    .skip(1)
                    .map(str::to_string)
                    .collect(),
            }
        })
        .collect())
}
pub fn read_file(filepath: &str) -> String {
    let file = File::open(filepath).expect("Unable to open file");
    let mut buffered_reader = BufReader::new(file);
    let mut contents = String::new();
    buffered_reader
        .read_to_string(&mut contents)
        .expect("Unable to read file into the string");
    contents.trim_end().to_string()
}
fn main() {
    let inp = load_file("input.txt").unwrap();
    let sum: usize = inp.iter().map(|m| m.display_contains_guessable()).sum();
    println!("No digits 1, 4, 7, or 8: {}", sum);
    let total: usize = inp.iter().map(SegmentDecoder::calc_number).sum();
    println!("Total {}", total);
}
